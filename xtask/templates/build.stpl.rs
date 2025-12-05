fn main() {
    // crate/ is at langs/group-*/lang/crate/
    // grammar sources are at langs/group-*/lang/def/grammar/src/
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let def_dir = manifest_dir.join("../def");
    let src_dir = def_dir.join("grammar/src");
    let grammar_dir = def_dir.join("grammar");

    println!("cargo:rerun-if-changed={}", src_dir.join("parser.c").display());
<% if has_scanner { %>
    println!("cargo:rerun-if-changed={}", grammar_dir.join("scanner.c").display());
<% } %>

    let mut build = cc::Build::new();

    build
        .include(&src_dir)
        .include(&grammar_dir) // for common/ includes like "../common/scanner.h"
        .include(src_dir.join("tree_sitter"))
        .opt_level_str("z") // optimize aggressively for size
        .warnings(false)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    // For WASM builds, use our custom sysroot (provided by arborium crate via links = "arborium")
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm")
        && let Ok(sysroot) = std::env::var("DEP_ARBORIUM_SYSROOT_PATH")
    {
        build.include(&sysroot);
    }

    build.file(src_dir.join("parser.c"));
<% if has_scanner { %>
    build.file(grammar_dir.join("scanner.c"));
<% } %>

    build.compile("tree_sitter_<%= c_symbol %>");
}
