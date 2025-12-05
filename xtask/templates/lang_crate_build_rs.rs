fn main() {
    let src_dir = "grammar/src";

    println!("cargo:rerun-if-changed={}/parser.c", src_dir);
    {
        scanner_section
    }
    let mut build = cc::Build::new();

    build
        .include(src_dir)
        .include("grammar") // for common/ includes like "../common/scanner.h"
        .include(format!("{}/tree_sitter", src_dir))
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    #[cfg(target_env = "msvc")]
    build.flag("-utf-8");

    build.file(format!("{}/parser.c", src_dir));
    {
        scanner_compile
    }

    build.compile("tree_sitter_{c_symbol}");
}
