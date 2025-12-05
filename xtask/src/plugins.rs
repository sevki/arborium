//! WASM plugin build system.
//!
//! This module handles building grammar plugins as WASM components
//! and transpiling them to JavaScript for browser usage.

use std::sync::Mutex;
use std::time::Instant;

use camino::{Utf8Path, Utf8PathBuf};
use chrono::Utc;
use miette::{Context, IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use rayon::prelude::*;
use regex::Regex;

use crate::tool::Tool;
use crate::types::CrateRegistry;

/// Build options for plugins.
pub struct BuildOptions {
    /// Specific grammars to build (empty = all)
    pub grammars: Vec<String>,
    /// Optional override output directory (per-grammar subdir will be created)
    pub output_dir: Option<Utf8PathBuf>,
    /// Whether to run jco transpile after building
    pub transpile: bool,
    /// Whether to profile build times and write to plugin-timings.json
    pub profile: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            grammars: Vec::new(),
            output_dir: None,
            transpile: true,
            profile: false,
        }
    }
}

/// Timing data for a single grammar plugin build.
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginTiming {
    /// Grammar ID (e.g., "rust", "javascript")
    pub grammar: String,
    /// Total build time in milliseconds
    pub build_ms: u64,
    /// Time for cargo-component build step in milliseconds
    pub cargo_component_ms: u64,
    /// Time for jco transpile step in milliseconds (0 if transpile disabled)
    pub transpile_ms: u64,
}

/// Collection of plugin build timings.
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginTimings {
    /// When these timings were recorded
    pub recorded_at: String,
    /// Individual grammar timings
    pub timings: Vec<PluginTiming>,
}

/// A group of plugins to build together (used by CI generator).
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginGroup {
    pub index: usize,
    pub grammars: Vec<String>,
    pub total_ms: u64,
}

/// Per-language manifest entry for JS/CDN loaders.
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginManifestEntry {
    pub language: String,
    pub package: String,
    pub version: String,
    pub cdn_js: String,
    pub cdn_wasm: String,
    pub local_js: String,
    pub local_wasm: String,
}

/// Manifest of all plugins, emitted after build.
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginManifest {
    pub generated_at: String,
    pub entries: Vec<PluginManifestEntry>,
}

/// Grouping result for CI parallelization.
#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
pub struct PluginGroups {
    pub groups: Vec<PluginGroup>,
    pub max_group_ms: u64,
    pub ideal_per_group_ms: u64,
    pub efficiency: f64,
}

impl PluginGroups {
    /// Greedy LPT bin-packing to balance groups.
    pub fn from_timings(timings: &PluginTimings, num_groups: usize) -> Self {
        let num_groups = num_groups.max(1);

        let mut sorted: Vec<_> = timings.timings.iter().collect();
        sorted.sort_by(|a, b| b.build_ms.cmp(&a.build_ms));

        let mut groups: Vec<PluginGroup> = (0..num_groups)
            .map(|i| PluginGroup {
                index: i,
                grammars: Vec::new(),
                total_ms: 0,
            })
            .collect();

        for timing in sorted {
            let g = groups
                .iter_mut()
                .min_by_key(|g| g.total_ms)
                .expect("at least one group");
            g.grammars.push(timing.grammar.clone());
            g.total_ms += timing.build_ms;
        }

        groups.retain(|g| !g.grammars.is_empty());
        for (i, g) in groups.iter_mut().enumerate() {
            g.index = i;
        }

        let max_group_ms = groups.iter().map(|g| g.total_ms).max().unwrap_or(0);
        let total_ms: u64 = timings.timings.iter().map(|t| t.build_ms).sum();
        let ideal_per_group_ms = if num_groups > 0 {
            total_ms / num_groups as u64
        } else {
            0
        };
        let efficiency = if max_group_ms > 0 {
            ideal_per_group_ms as f64 / max_group_ms as f64
        } else {
            0.0
        };

        Self {
            groups,
            max_group_ms,
            ideal_per_group_ms,
            efficiency,
        }
    }
}

impl PluginTimings {
    /// Load timings from a JSON file.
    pub fn load(path: &Utf8Path) -> miette::Result<Self> {
        let content = fs_err::read_to_string(path)
            .map_err(|e| miette::miette!("failed to read {}: {}", path, e))?;
        facet_json::from_str(&content)
            .map_err(|e| miette::miette!("failed to parse {}: {}", path, e))
    }

    /// Save timings to a JSON file.
    pub fn save(&self, path: &Utf8Path) -> miette::Result<()> {
        let content = facet_json::to_string_pretty(self);
        fs_err::write(path, content)
            .map_err(|e| miette::miette!("failed to write {}: {}", path, e))?;
        Ok(())
    }
}

/// Read the canonical workspace version from Cargo.toml.
///
/// `xtask gen --version` is responsible for writing this; all other commands
/// should read from here instead of taking a manual flag to avoid skew.
fn read_workspace_version(repo_root: &Utf8Path) -> Result<String> {
    let cargo_toml = repo_root.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .into_diagnostic()
        .context("failed to read Cargo.toml")?;

    let re = Regex::new(r#"(?m)^\[workspace\.package\][^\[]*?^version\s*=\s*"([^"]+)""#)
        .expect("regex is valid");

    let Some(caps) = re.captures(&content) else {
        miette::bail!(
            "workspace.package.version is missing in Cargo.toml (run `cargo xtask gen --version <x.y.z>` first)"
        );
    };

    Ok(caps[1].to_string())
}

/// Build WASM component plugins.
pub fn build_plugins(repo_root: &Utf8Path, options: &BuildOptions) -> Result<()> {
    let crates_dir = repo_root.join("crates");

    // Load registry to find grammars with generate-component: true
    let registry = CrateRegistry::load(&crates_dir)
        .map_err(|e| miette::miette!("failed to load crate registry: {}", e))?;

    // Get grammars to build
    let grammars: Vec<String> = if options.grammars.is_empty() {
        // Find all grammars with generate-component: true
        registry
            .all_grammars()
            .filter(|(_, _, grammar)| grammar.generate_component())
            .map(|(_, _, grammar)| grammar.id().to_string())
            .collect()
    } else {
        options.grammars.clone()
    };

    if grammars.is_empty() {
        println!(
            "{} No grammars have generate-component enabled",
            "○".dimmed()
        );
        println!(
            "  Add {} to a grammar's arborium.kdl to enable",
            "generate-component #true".cyan()
        );
        return Ok(());
    }

    println!(
        "{} Building {} plugin(s): {}",
        "●".cyan(),
        grammars.len(),
        grammars.join(", ")
    );

    let cargo_component = Tool::CargoComponent
        .find()
        .into_diagnostic()
        .context("cargo-component not found")?;
    let version = read_workspace_version(repo_root)?;

    let jco = if options.transpile {
        Some(
            Tool::Jco
                .find()
                .into_diagnostic()
                .context("jco not found")?,
        )
    } else {
        None
    };

    // Track timings if profiling is enabled
    let timings: Mutex<Vec<PluginTiming>> = Mutex::new(Vec::new());
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());

    // Build plugins in parallel (limit to 16 concurrent builds)
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build()
        .expect("failed to create thread pool");

    pool.install(|| {
        grammars.par_iter().for_each(|grammar| {
            let result = build_single_plugin(
                repo_root,
                &registry,
                grammar,
                options.output_dir.as_deref(),
                &version,
                &cargo_component,
                jco.as_ref(),
                options.profile,
            );

            match result {
                Ok(timing) => {
                    timings.lock().unwrap().push(timing);
                }
                Err(e) => {
                    errors.lock().unwrap().push(format!("{}: {}", grammar, e));
                }
            }
        })
    });

    // Check for errors
    let errors = errors.into_inner().unwrap();
    if !errors.is_empty() {
        for err in &errors {
            eprintln!("{} {}", "✗".red(), err);
        }
        miette::bail!("{} plugin(s) failed to build", errors.len());
    }

    let timings = timings.into_inner().unwrap();

    // Emit manifest for loaders (cdn + local) using recorded version
    let manifest = build_manifest(
        repo_root,
        &registry,
        &grammars,
        options.output_dir.as_deref(),
        &version,
    )?;
    let manifest_path = repo_root.join("langs").join("plugins.json");
    fs_err::create_dir_all(manifest_path.parent().unwrap())
        .into_diagnostic()
        .context("failed to create manifest dir")?;
    fs_err::write(&manifest_path, facet_json::to_string_pretty(&manifest))
        .into_diagnostic()
        .context("failed to write manifest")?;
    println!(
        "{} Wrote plugin manifest {}",
        "✓".green(),
        manifest_path.cyan()
    );

    // Save timings if profiling is enabled
    if options.profile {
        let timings_path = repo_root.join("plugin-timings.json");
        let plugin_timings = PluginTimings {
            recorded_at: Utc::now().to_rfc3339(),
            timings,
        };
        plugin_timings.save(&timings_path)?;
        println!("\n{} Saved timings to {}", "✓".green(), timings_path.cyan());

        // Print summary
        let total_ms: u64 = plugin_timings.timings.iter().map(|t| t.build_ms).sum();
        println!("\n{} Build time summary:", "●".cyan());
        for timing in &plugin_timings.timings {
            let pct = (timing.build_ms as f64 / total_ms as f64) * 100.0;
            println!(
                "  {} {}: {}ms ({:.1}%)",
                "→".dimmed(),
                timing.grammar,
                timing.build_ms,
                pct
            );
        }
        println!("  {} Total: {}ms", "=".dimmed(), total_ms);
    }

    Ok(())
}

/// Build a single plugin and return timing info.
fn locate_grammar<'a>(
    registry: &'a CrateRegistry,
    grammar: &str,
) -> Option<(
    &'a crate::types::CrateState,
    &'a crate::types::GrammarConfig,
)> {
    registry.configured_crates().find_map(|(_, state, cfg)| {
        cfg.grammars
            .iter()
            .find(|g| <String as AsRef<str>>::as_ref(&g.id.value) == grammar)
            .map(|g| (state, g))
    })
}

fn write_plugin_cargo_toml(
    repo_root: &Utf8Path,
    grammar: &str,
    grammar_crate: &str,
    grammar_crate_path: &Utf8Path,
    out_dir: &Utf8Path,
) -> Result<()> {
    let runtime_path = repo_root.join("crates/arborium-plugin-runtime");
    let wire_path = repo_root.join("crates/arborium-wire");

    let cargo_toml = format!(
        r#"[package]
name = "arborium-{grammar}-plugin"
version = "0.1.0"
edition = "2024"
description = "{grammar} grammar plugin for arborium"
license = "MIT"
repository = "https://github.com/bearcove/arborium"

[lib]
crate-type = ["cdylib"]

[dependencies]
arborium-plugin-runtime = {{ path = "{runtime_path}" }}
arborium-wire = {{ path = "{wire_path}" }}
"{grammar_crate}" = {{ path = "{grammar_crate_path}" }}
wit-bindgen = "0.36"

[package.metadata.component]
package = "arborium:grammar"

[package.metadata.component.target]
world = "grammar-plugin"
path = "{wit_path}"
"#,
        grammar = grammar,
        runtime_path = runtime_path.as_str(),
        wire_path = wire_path.as_str(),
        grammar_crate = grammar_crate,
        grammar_crate_path = grammar_crate_path.as_str(),
        wit_path = repo_root.join("wit/grammar.wit").as_str(),
    );

    std::fs::write(out_dir.join("Cargo.toml"), cargo_toml)
        .into_diagnostic()
        .context("failed to write temp Cargo.toml")
}

fn write_plugin_lib_rs(
    repo_root: &Utf8Path,
    grammar: &str,
    grammar_crate: &str,
    out_dir: &Utf8Path,
) -> Result<()> {
    let lib_rs = format!(
        r#"//! {grammar} grammar plugin for arborium.
#![allow(unsafe_op_in_unsafe_fn)]

wit_bindgen::generate!({{
    world: "grammar-plugin",
    path: "{wit_path}",
}});

use arborium_plugin_runtime::{{HighlightConfig, PluginRuntime}};
use arborium_wire::Edit as WireEdit;
use std::cell::RefCell;

// Import the generated types
use arborium::grammar::types::{{Edit, Injection, ParseError, ParseResult, Span}};

thread_local! {{
    static RUNTIME: RefCell<Option<PluginRuntime>> = const {{ RefCell::new(None) }};
}}

fn get_or_init_runtime() -> &'static RefCell<Option<PluginRuntime>> {{
    RUNTIME.with(|r| {{
        let mut runtime = r.borrow_mut();
        if runtime.is_none() {{
            let config = HighlightConfig::new(
                {grammar_crate}::language(),
                {grammar_crate}::HIGHLIGHTS_QUERY,
                {grammar_crate}::INJECTIONS_QUERY,
                {grammar_crate}::LOCALS_QUERY,
            )
            .expect("failed to create highlight config");
            *runtime = Some(PluginRuntime::new(config));
        }}
        // SAFETY: We're returning a reference to thread-local storage which lives
        // for the duration of the WASM instance.
        unsafe {{ &*(r as *const _) }}
    }})
}}

struct PluginImpl;

impl exports::arborium::grammar::plugin::Guest for PluginImpl {{
    fn language_id() -> String {{
        "{grammar}".to_string()
    }}

    fn injection_languages() -> Vec<String> {{
        // TODO: Parse injection queries to determine which languages are injected
        Vec::new()
    }}

    fn create_session() -> u32 {{
        get_or_init_runtime()
            .borrow_mut()
            .as_mut()
            .expect("runtime not initialized")
            .create_session()
    }}

    fn free_session(session: u32) {{
        get_or_init_runtime()
            .borrow_mut()
            .as_mut()
            .expect("runtime not initialized")
            .free_session(session);
    }}

    fn set_text(session: u32, text: String) {{
        get_or_init_runtime()
            .borrow_mut()
            .as_mut()
            .expect("runtime not initialized")
            .set_text(session, &text);
    }}

    fn apply_edit(session: u32, text: String, edit: Edit) {{
        let wire_edit = WireEdit {{
            start_byte: edit.start_byte,
            old_end_byte: edit.old_end_byte,
            new_end_byte: edit.new_end_byte,
            start_row: edit.start_row,
            start_col: edit.start_col,
            old_end_row: edit.old_end_row,
            old_end_col: edit.old_end_col,
            new_end_row: edit.new_end_row,
            new_end_col: edit.new_end_col,
        }};
        get_or_init_runtime()
            .borrow_mut()
            .as_mut()
            .expect("runtime not initialized")
            .apply_edit(session, &text, &wire_edit);
    }}

    fn parse(session: u32) -> Result<ParseResult, ParseError> {{
        let result = get_or_init_runtime()
            .borrow_mut()
            .as_mut()
            .expect("runtime not initialized")
            .parse(session);

        match result {{
            Ok(r) => Ok(ParseResult {{
                spans: r
                    .spans
                    .into_iter()
                    .map(|s| Span {{
                        start: s.start,
                        end: s.end,
                        capture: s.capture,
                    }})
                    .collect(),
                injections: r
                    .injections
                    .into_iter()
                    .map(|i| Injection {{
                        start: i.start,
                        end: i.end,
                        language: i.language,
                        include_children: i.include_children,
                    }})
                    .collect(),
            }}),
            Err(e) => Err(ParseError {{
                message: e.message,
            }}),
        }}
    }}
}}

export!(PluginImpl);
"#,
        grammar = grammar,
        grammar_crate = grammar_crate,
        wit_path = repo_root.join("wit/grammar.wit").as_str(),
    );

    std::fs::write(out_dir.join("src/lib.rs"), lib_rs)
        .into_diagnostic()
        .context("failed to write temp lib.rs")
}

fn write_package_json(language: &str, version: &str, plugin_dir: &Utf8Path) -> Result<()> {
    let package_json = generate_package_json(language, version);
    let package_json_path = plugin_dir.join("package.json");
    std::fs::write(&package_json_path, package_json)
        .into_diagnostic()
        .context(format!("failed to write {}", package_json_path))?;
    Ok(())
}

fn build_manifest(
    repo_root: &Utf8Path,
    registry: &CrateRegistry,
    grammars: &[String],
    output_override: Option<&Utf8Path>,
    version: &str,
) -> Result<PluginManifest> {
    let mut entries = Vec::new();

    for grammar in grammars {
        // Locate grammar state
        let (state, _) = locate_grammar(registry, grammar)
            .ok_or_else(|| miette::miette!("grammar `{}` not found for manifest", grammar))?;

        // local path (either override or langs/*/*/npm/<lang>)
        let local_root = if let Some(base) = output_override {
            if base.is_absolute() {
                base.to_owned()
            } else {
                repo_root.join(base)
            }
        } else {
            state
                .crate_path
                .parent()
                .expect("lang directory")
                .join("npm")
        };
        let local_js = local_root.join("grammar.js");
        let local_wasm = local_root.join("grammar.core.wasm");

        let package = format!("@arborium/{}", grammar);
        let cdn_base = format!(
            "https://cdn.jsdelivr.net/npm/@arborium/{}@{}",
            grammar, version
        );

        entries.push(PluginManifestEntry {
            language: grammar.clone(),
            package: package.clone(),
            version: version.to_string(),
            cdn_js: format!("{}/grammar.js", cdn_base),
            cdn_wasm: format!("{}/grammar.core.wasm", cdn_base),
            local_js: format!("/{}", local_js),
            local_wasm: format!("/{}", local_wasm),
        });
    }

    Ok(PluginManifest {
        generated_at: Utc::now().to_rfc3339(),
        entries,
    })
}

/// Build a single plugin and return timing info.
fn build_single_plugin(
    repo_root: &Utf8Path,
    registry: &CrateRegistry,
    grammar: &str,
    output_override: Option<&Utf8Path>,
    version: &str,
    cargo_component: &crate::tool::ToolPath,
    jco: Option<&crate::tool::ToolPath>,
    profile: bool,
) -> Result<PluginTiming> {
    let grammar_start = Instant::now();
    println!("{} {}", "Building:".cyan(), grammar);

    let (crate_state, _) = locate_grammar(registry, grammar).ok_or_else(|| {
        miette::miette!(
            "grammar `{}` not found in registry (generate components must be enabled)",
            grammar
        )
    })?;

    let grammar_crate_path = &crate_state.crate_path;
    let grammar_crate_name = &crate_state.name;

    // Decide output path: override/<grammar> or langs/.../npm
    let plugin_output = if let Some(base) = output_override {
        let base = if base.is_absolute() {
            base.to_owned()
        } else {
            repo_root.join(base)
        };
        base.join(grammar)
    } else {
        grammar_crate_path
            .parent()
            .expect("lang directory")
            .join("npm")
    };
    std::fs::create_dir_all(&plugin_output)
        .into_diagnostic()
        .context("failed to create plugin output directory")?;

    // Create an isolated temp plugin crate to avoid workspace deps
    let _temp_dir = tempfile::tempdir().into_diagnostic()?;
    let temp_path =
        Utf8PathBuf::from_path_buf(_temp_dir.path().to_path_buf()).expect("temp path utf8");
    std::fs::create_dir_all(temp_path.join("src"))
        .into_diagnostic()
        .context("failed to create temp src dir")?;

    write_plugin_cargo_toml(
        repo_root,
        grammar,
        grammar_crate_name,
        grammar_crate_path,
        &temp_path,
    )?;
    write_plugin_lib_rs(repo_root, grammar, grammar_crate_name, &temp_path)?;

    let plugin_crate = format!("arborium-{}-plugin", grammar);

    // Build with cargo component from the temp directory
    let cargo_start = Instant::now();
    let output = cargo_component
        .command()
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-wasip1",
            "--manifest-path",
            temp_path.join("Cargo.toml").as_str(),
        ])
        .output()
        .into_diagnostic()
        .context("failed to run cargo-component")?;
    let cargo_component_ms = cargo_start.elapsed().as_millis() as u64;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        miette::bail!("cargo-component build failed:\n{}", stderr);
    }

    // Find the built wasm file
    let wasm_file = temp_path
        .join("target/wasm32-wasip1/release")
        .join(format!("{}.wasm", plugin_crate.replace('-', "_")));

    if !wasm_file.exists() {
        miette::bail!("expected wasm file not found: {}", wasm_file);
    }

    // Copy to output directory
    let dest_wasm = plugin_output.join("grammar.wasm");
    std::fs::copy(&wasm_file, &dest_wasm)
        .into_diagnostic()
        .context("failed to copy wasm file")?;

    // Transpile with jco if enabled
    let mut transpile_ms = 0u64;
    if let Some(jco) = jco {
        let transpile_start = Instant::now();
        let output = jco
            .command()
            .args([
                "transpile",
                dest_wasm.as_str(),
                "--instantiation",
                "async",
                "--quiet",
                "-o",
                plugin_output.as_str(),
            ])
            .output()
            .into_diagnostic()
            .context("failed to run jco")?;
        transpile_ms = transpile_start.elapsed().as_millis() as u64;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            miette::bail!("jco transpile failed:\n{}", stderr);
        }

        // Calculate total wasm bundle size
        let total_wasm_size: u64 = std::fs::read_dir(&plugin_output)
            .into_diagnostic()?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "wasm"))
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum();

        println!(
            "  {} {} ({})",
            "✓".green(),
            grammar,
            format_size(total_wasm_size as usize)
        );
    } else {
        println!("  {} {}", "✓".green(), grammar);
    }

    let build_ms = grammar_start.elapsed().as_millis() as u64;

    if profile {
        println!(
            "    {} {}ms (cargo: {}ms, jco: {}ms)",
            "⏱".dimmed(),
            build_ms,
            cargo_component_ms,
            transpile_ms
        );
    }

    // Write npm package.json alongside artifacts
    write_package_json(grammar, version, &plugin_output)?;

    Ok(PluginTiming {
        grammar: grammar.to_string(),
        build_ms,
        cargo_component_ms,
        transpile_ms,
    })
}

/// Clean plugin build artifacts.
pub fn clean_plugins(repo_root: &Utf8Path, output_dir: &str) -> Result<()> {
    let output_path = repo_root.join(output_dir);
    if output_path.exists() {
        std::fs::remove_dir_all(&output_path)
            .into_diagnostic()
            .context("failed to remove output directory")?;
        println!("{} Removed {}", "✓".green(), output_path);
    } else {
        println!("{} Nothing to clean", "○".dimmed());
    }
    Ok(())
}

fn format_size(bytes: usize) -> String {
    if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1000 {
        format!("{:.2} KB", bytes as f64 / 1000.0)
    } else {
        format!("{} B", bytes)
    }
}

// =============================================================================
// npm package generation
// =============================================================================

/// Generate a package.json for a grammar plugin.
fn generate_package_json(language: &str, version: &str) -> String {
    // Escape any special characters in the language name for JSON
    let safe_language = language.replace('\\', "\\\\").replace('"', "\\\"");

    format!(
        r#"{{
  "name": "@arborium/{language}",
  "version": "{version}",
  "description": "Arborium syntax highlighting grammar for {language}",
  "type": "module",
  "main": "./grammar.js",
  "module": "./grammar.js",
  "types": "./grammar.d.ts",
  "exports": {{
    ".": {{
      "import": "./grammar.js",
      "types": "./grammar.d.ts"
    }},
    "./grammar.js": "./grammar.js",
    "./grammar.core.wasm": "./grammar.core.wasm"
  }},
  "files": [
    "grammar.js",
    "grammar.d.ts",
    "grammar.core.wasm",
    "interfaces"
  ],
  "keywords": [
    "arborium",
    "syntax-highlighting",
    "tree-sitter",
    "{language}",
    "wasm"
  ],
  "author": "Amos Wenger <amos@bearcove.net>",
  "license": "MIT OR Apache-2.0",
  "repository": {{
    "type": "git",
    "url": "git+https://github.com/bearcove/arborium.git"
  }},
  "homepage": "https://github.com/bearcove/arborium",
  "bugs": {{
    "url": "https://github.com/bearcove/arborium/issues"
  }},
  "publishConfig": {{
    "access": "public"
  }}
}}
"#,
        language = safe_language,
        version = version
    )
}
