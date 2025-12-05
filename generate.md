# Generation Flow

This document describes the proper flow for `cargo xtask gen` to avoid the clusterfuck of running tree-sitter on broken grammars.

## Flow Diagram

```
1. Registry Loading
   ├── Scan crates/arborium-* (legacy structure)
   ├── Scan langs/group-*/*/def/ (new structure)  
   └── Build CrateRegistry with all language definitions

2. Pre-Generation Validation (CRITICAL - CATCHES MISSING FILES EARLY)
   ├── For EVERY grammar with grammar.js:
   │   ├── Create temp wrapper with dummy tree-sitter globals
   │   ├── Run `node wrapper.js` to require grammar.js
   │   ├── If require() fails → STOP EVERYTHING, SHOW ERROR
   │   └── If require() succeeds → grammar is valid
   └── Only proceed if ALL grammars pass validation

3. Generation Phase (EXPENSIVE - CACHED)
   ├── For each grammar in parallel:
   │   ├── Check cache by hash of input files
   │   ├── If cache hit → extract cached files
   │   ├── If cache miss → run tree-sitter generate (SLOW)
   │   └── Save generated files to cache
   └── Create plans for file updates

4. Crate Generation Phase (FAST)
   ├── Generate Cargo.toml files
   ├── Generate build.rs files
   ├── Generate src/lib.rs files
   └── Execute all file operations

5. Post-Generation Lint (VERIFICATION)
   ├── Check that all expected files were generated
   ├── Validate arborium.kdl syntax
   └── Check sample files exist and are reasonable
```

## Why This Order Matters

### 1. Registry Loading
- Fast operation (~100ms)
- Discovers all language definitions
- No external dependencies

### 2. Pre-Generation Validation
- **CRITICAL PHASE** - catches missing file dependencies BEFORE expensive operations
- Uses Node.js to validate `require()` statements in grammar.js files
- Should check ALL grammars, not just ones with cross-grammar dependencies
- If ANY grammar fails validation → STOP IMMEDIATELY, don't waste time on tree-sitter

### 3. Generation Phase  
- **MOST EXPENSIVE** - tree-sitter generate can take 5-20s per grammar
- **HEAVILY CACHED** - results cached by hash of input files
- Runs in parallel for speed
- Only runs if pre-validation passed

### 4. Crate Generation
- Fast file template operations
- Creates Rust crate structure

### 5. Post-Generation Lint
- Verification that everything worked
- Catches edge cases missed by earlier phases

## Current Problems

The pre-generation validation is currently broken because:

1. **Only validates grammars with cross-grammar dependencies** - should validate ALL grammars
2. **Misses local file dependencies** - `require('../common/common')` isn't detected as needing validation
3. **Validation happens but then tree-sitter still runs on invalid grammars**

## Fix Required

The pre-validation phase should:
- Check EVERY grammar that has a grammar.js file
- Validate ALL require() statements (both cross-grammar AND local files)  
- STOP the entire process if ANY grammar fails validation
- Never let tree-sitter run on a grammar that failed pre-validation

This way we catch "Cannot find module" errors in ~100ms instead of after 5+ seconds of tree-sitter processing.