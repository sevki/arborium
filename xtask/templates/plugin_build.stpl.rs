fn main() {
    // npm/ is at langs/group-*/lang/npm/
    // grammar sources are copied to npm/grammar/src/
    // common/ is at langs/group-*/lang/def/common/ (for scanner includes)
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("grammar/src");
    let grammar_dir = manifest_dir.join("grammar");
    let def_dir = manifest_dir.join("../def");

    println!("cargo:rerun-if-changed={}", src_dir.join("parser.c").display());
<% if has_scanner { %>
    println!("cargo:rerun-if-changed={}", src_dir.join("scanner.c").display());
<% } %>

    let mut build = cc::Build::new();

    build
        .include(&src_dir)
        .include(&grammar_dir)
        .include(src_dir.join("tree_sitter"))
        // Include def/grammar/ for common/ scanner includes (e.g. "common/scanner.h")
        .include(def_dir.join("grammar"))
        .opt_level_str("z") // optimize aggressively for size
        .warnings(false)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    // For WASM builds, use our custom sysroot
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        let sysroot = manifest_dir.join("../../../../wasm-sysroot");
        if sysroot.exists() {
            build.include(&sysroot);
        }
    }

    build.file(src_dir.join("parser.c"));
<% if has_scanner { %>
    build.file(src_dir.join("scanner.c"));
<% } %>

    build.compile("tree_sitter_<%= c_symbol %>");
}
