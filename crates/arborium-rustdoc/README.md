# arborium-rustdoc

Post-process rustdoc HTML output to add syntax highlighting for non-Rust code blocks.

## Overview

Rustdoc highlights Rust code using rustc's parser, but code blocks in other languages (TOML, shell, JSON, etc.) are left as plain text. This tool processes rustdoc output and adds tree-sitter based syntax highlighting for all supported languages.

## Installation

```bash
cargo install arborium-rustdoc
```

## Usage

```bash
# Process rustdoc output in-place
arborium-rustdoc ./target/doc ./target/doc-highlighted

# Or with a specific theme
arborium-rustdoc --theme catppuccin-mocha ./target/doc ./output
```

## How it works

1. **CSS Generation**: Generates theme CSS rules for arborium's custom elements and appends them to rustdoc's CSS file (`static.files/rustdoc-*.css`)

2. **HTML Transformation**: Uses [lol_html](https://crates.io/crates/lol_html) to stream through each HTML file, finding `<pre class="language-*">` elements and replacing their content with syntax-highlighted HTML.

3. **Theme Integration**: Integrates with rustdoc's built-in theme system (light, dark, ayu) by generating CSS rules scoped to `[data-theme="..."]` selectors.

## Supported Languages

All languages supported by [arborium](https://arborium.bearcove.eu/) are available, including:

- TOML, YAML, JSON, KDL
- Bash, Fish, Zsh, PowerShell
- Python, Ruby, JavaScript, TypeScript
- Go, C, C++, Zig
- And many more...

## Library Usage

```rust
use arborium_rustdoc::{Processor, ProcessOptions};

let options = ProcessOptions {
    theme: "catppuccin-mocha".to_string(),
};

let processor = Processor::new(options)?;
let stats = processor.process("./target/doc", "./output")?;

println!("Highlighted {} code blocks", stats.blocks_highlighted);
```

## License

MIT OR Apache-2.0
