use arborium::{
    html,
    tree_sitter_highlight::{HighlightConfiguration, Highlighter},
    HIGHLIGHT_NAMES,
};
use wasm_bindgen::prelude::*;

/// Helper to create a HighlightConfiguration from a grammar module
macro_rules! make_config {
    ($lang:ident, $name:expr) => {{
        HighlightConfiguration::new(
            arborium::$lang::language().into(),
            $name,
            arborium::$lang::HIGHLIGHTS_QUERY,
            arborium::$lang::INJECTIONS_QUERY,
            arborium::$lang::LOCALS_QUERY,
        )
    }};
}

/// Get a highlight configuration for the given language
fn get_config(language: &str) -> Option<Result<HighlightConfiguration, arborium::tree_sitter::QueryError>> {
    match language {
        // Programming languages
        "ada" => Some(make_config!(lang_ada, "ada")),
        "agda" => Some(make_config!(lang_agda, "agda")),
        "asm" | "assembly" => Some(make_config!(lang_asm, "asm")),
        "awk" => Some(make_config!(lang_awk, "awk")),
        "bash" | "sh" | "shell" => Some(make_config!(lang_bash, "bash")),
        "batch" | "bat" | "cmd" => Some(make_config!(lang_batch, "batch")),
        "c" | "h" => Some(make_config!(lang_c, "c")),
        "c-sharp" | "cs" | "csharp" => Some(make_config!(lang_c_sharp, "c-sharp")),
        "clojure" | "clj" => Some(make_config!(lang_clojure, "clojure")),
        "commonlisp" | "lisp" | "cl" => Some(make_config!(lang_commonlisp, "commonlisp")),
        "cpp" | "c++" | "cxx" | "hpp" => Some(make_config!(lang_cpp, "cpp")),
        "d" | "dlang" => Some(make_config!(lang_d, "d")),
        "dart" => Some(make_config!(lang_dart, "dart")),
        "elisp" | "emacs-lisp" | "el" => Some(make_config!(lang_elisp, "elisp")),
        "elixir" | "ex" | "exs" => Some(make_config!(lang_elixir, "elixir")),
        "elm" => Some(make_config!(lang_elm, "elm")),
        "erlang" | "erl" => Some(make_config!(lang_erlang, "erlang")),
        "fish" => Some(make_config!(lang_fish, "fish")),
        "fsharp" | "fs" | "f#" => Some(make_config!(lang_fsharp, "fsharp")),
        "gleam" => Some(make_config!(lang_gleam, "gleam")),
        "glsl" | "vert" | "frag" => Some(make_config!(lang_glsl, "glsl")),
        "go" | "golang" => Some(make_config!(lang_go, "go")),
        "haskell" | "hs" => Some(make_config!(lang_haskell, "haskell")),
        "hlsl" => Some(make_config!(lang_hlsl, "hlsl")),
        "java" => Some(make_config!(lang_java, "java")),
        "javascript" | "js" | "jsx" | "mjs" | "cjs" => Some(make_config!(lang_javascript, "javascript")),
        "julia" | "jl" => Some(make_config!(lang_julia, "julia")),
        "lean" => Some(make_config!(lang_lean, "lean")),
        "lua" => Some(make_config!(lang_lua, "lua")),
        "matlab" | "m" => Some(make_config!(lang_matlab, "matlab")),
        "objc" | "objective-c" | "mm" => Some(make_config!(lang_objc, "objc")),
        "perl" | "pl" | "pm" => Some(make_config!(lang_perl, "perl")),
        "php" => Some(make_config!(lang_php, "php")),
        "python" | "py" | "py3" | "python3" => Some(make_config!(lang_python, "python")),
        "r" | "rlang" => Some(make_config!(lang_r, "r")),
        "ruby" | "rb" => Some(make_config!(lang_ruby, "ruby")),
        "rust" | "rs" => Some(make_config!(lang_rust, "rust")),
        "scala" => Some(make_config!(lang_scala, "scala")),
        "starlark" | "bzl" | "bazel" => Some(make_config!(lang_starlark, "starlark")),
        "uiua" | "ua" => Some(make_config!(lang_uiua, "uiua")),
        "vb" | "vbnet" | "visualbasic" => Some(make_config!(lang_vb, "vb")),
        "verilog" | "v" | "sv" | "systemverilog" => Some(make_config!(lang_verilog, "verilog")),
        // "vim" | "viml" | "vimscript" => Some(make_config!(lang_vim, "vim")), // excluded - slow to compile
        "zig" => Some(make_config!(lang_zig, "zig")),
        // "zsh" => Some(make_config!(lang_zsh, "zsh")), // excluded - grammar incomplete

        // Markup & templating
        "css" => Some(make_config!(lang_css, "css")),
        "html" | "htm" => Some(make_config!(lang_html, "html")),
        "jinja2" | "jinja" | "j2" => Some(make_config!(lang_jinja2, "jinja2")),
        "scss" | "sass" => Some(make_config!(lang_scss, "scss")),
        "svelte" => Some(make_config!(lang_svelte, "svelte")),
        "typst" | "typ" => Some(make_config!(lang_typst, "typst")),
        "vue" => Some(make_config!(lang_vue, "vue")),
        "xml" | "xsl" | "xslt" | "svg" => Some(make_config!(lang_xml, "xml")),

        // Config files
        "caddy" => Some(make_config!(lang_caddy, "caddy")),
        "cmake" => Some(make_config!(lang_cmake, "cmake")),
        "devicetree" => Some(make_config!(lang_devicetree, "devicetree")),
        "dockerfile" | "docker" => Some(make_config!(lang_dockerfile, "dockerfile")),
        "hcl" | "terraform" | "tf" => Some(make_config!(lang_hcl, "hcl")),
        "ini" | "conf" | "cfg" => Some(make_config!(lang_ini, "ini")),
        "kdl" => Some(make_config!(lang_kdl, "kdl")),
        "meson" => Some(make_config!(lang_meson, "meson")),
        "nginx" => Some(make_config!(lang_nginx, "nginx")),
        "ninja" => Some(make_config!(lang_ninja, "ninja")),
        "nix" => Some(make_config!(lang_nix, "nix")),
        "ssh-config" => Some(make_config!(lang_ssh_config, "ssh-config")),
        "toml" => Some(make_config!(lang_toml, "toml")),
        "yaml" | "yml" => Some(make_config!(lang_yaml, "yaml")),

        // Data formats
        "capnp" => Some(make_config!(lang_capnp, "capnp")),
        "diff" | "patch" => Some(make_config!(lang_diff, "diff")),
        "dot" => Some(make_config!(lang_dot, "dot")),
        "json" | "jsonc" => Some(make_config!(lang_json, "json")),
        "ron" => Some(make_config!(lang_ron, "ron")),
        "textproto" | "pbtxt" | "textpb" => Some(make_config!(lang_textproto, "textproto")),
        "thrift" => Some(make_config!(lang_thrift, "thrift")),

        // Query languages
        "graphql" | "gql" => Some(make_config!(lang_graphql, "graphql")),
        "jq" => Some(make_config!(lang_jq, "jq")),
        "query" | "scm" => Some(make_config!(lang_query, "query")),
        "sparql" | "rq" => Some(make_config!(lang_sparql, "sparql")),
        "sql" | "mysql" | "postgresql" | "postgres" | "sqlite" => Some(make_config!(lang_sql, "sql")),

        // Assembly
        "x86asm" | "nasm" | "x86" => Some(make_config!(lang_x86asm, "x86asm")),

        _ => None,
    }
}

/// Highlight source code and return HTML
#[wasm_bindgen]
pub fn highlight(language: &str, source: &str) -> Result<String, JsValue> {
    let config_result = get_config(language);
    let mut config = config_result
        .ok_or_else(|| JsValue::from_str(&format!("Unsupported language: {}", language)))?
        .map_err(|e| JsValue::from_str(&format!("Grammar error: {}", e)))?;

    // Configure highlight names
    let names: Vec<String> = HIGHLIGHT_NAMES.iter().map(|s| s.to_string()).collect();
    config.configure(&names);

    let mut highlighter = Highlighter::new();
    let mut output = Vec::new();

    html::render(&mut output, &mut highlighter, &config, source, |_| None)
        .map_err(|e| JsValue::from_str(&format!("Render error: {}", e)))?;

    String::from_utf8(output).map_err(|e| JsValue::from_str(&format!("UTF-8 error: {}", e)))
}

/// Get list of supported languages
#[wasm_bindgen]
pub fn supported_languages() -> Vec<JsValue> {
    vec![
        // Programming languages (sorted alphabetically)
        "ada",
        "agda",
        "asm",
        "awk",
        "bash",
        "batch",
        "c",
        "c-sharp",
        "clojure",
        "commonlisp",
        "cpp",
        "d",
        "dart",
        "elisp",
        "elixir",
        "elm",
        "erlang",
        "fish",
        "fsharp",
        "gleam",
        "glsl",
        "go",
        "haskell",
        "hlsl",
        "java",
        "javascript",
        "julia",
        "lean",
        "lua",
        "matlab",
        "objc",
        "perl",
        "php",
        "python",
        "r",
        "ruby",
        "rust",
        "scala",
        "starlark",
        "uiua",
        "vb",
        "verilog",
        "vim",
        "zig",
        "zsh",
        // Markup & templating
        "css",
        "html",
        "jinja2",
        "scss",
        "svelte",
        "typst",
        "vue",
        "xml",
        // Config files
        "caddy",
        "cmake",
        "devicetree",
        "dockerfile",
        "hcl",
        "ini",
        "kdl",
        "meson",
        "nginx",
        "ninja",
        "nix",
        "ssh-config",
        "toml",
        "yaml",
        // Data formats
        "capnp",
        "diff",
        "dot",
        "json",
        "ron",
        "textproto",
        "thrift",
        // Query languages
        "graphql",
        "jq",
        "query",
        "sparql",
        "sql",
        // Assembly
        "x86asm",
    ]
    .into_iter()
    .map(JsValue::from_str)
    .collect()
}

/// Get the highlight class names
#[wasm_bindgen]
pub fn highlight_names() -> Vec<JsValue> {
    HIGHLIGHT_NAMES
        .iter()
        .map(|s| JsValue::from_str(s))
        .collect()
}
