# Publishing Guide

## The New Paradigm

**The git tag is the single source of truth for versions.**

- Locally, versions are `0.0.0-dev` - they don't matter
- CI parses the version from the tag (e.g., `v0.3.0` → `0.3.0`)
- CI runs `xtask gen --version 0.3.0` which sets all versions
- Then CI publishes everything

This means:
- No version drift or mismatch issues
- No "forgot to bump version" mistakes
- No chore commits for version bumps
- Clean, simple workflow

## Release Flow

When you push a tag, here's what happens:

```
git tag v0.3.0
       │
       ▼
parse version (v0.3.0 → 0.3.0)
       │
       ▼
┌─────────────────────────────────────────────────────┐
│              PHASE 1: Core Publish             │
├─────────────────────────────────────────────────────┤
│ 1. Update arborium crate with inventory           │
│ 2. Publish arborium to crates.io              │
│ 3. Generate arborium-collection features       │
└─────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│           PHASE 2: Group Publishing           │
├─────────────────────────────────────────────────────┤
│ git tag v0.3.0-group-a → publish group-a    │
│ git tag v0.3.0-group-b → publish group-b    │
│ git tag v0.3.0-group-c → publish group-c    │
│ git tag v0.3.0-group-d → publish group-d    │
│ git tag v0.3.0-group-e → publish group-e    │
│ git tag v0.3.0-group-f → publish group-f    │
│ (Can run 2-3 groups in parallel)              │
└─────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│          PHASE 3: Collection Publish         │
├─────────────────────────────────────────────────────┤
│ 1. Update arborium-collection dependencies       │
│ 2. Publish arborium-collection to crates.io    │
└─────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│            PHASE 4: NPM Publishing           │
├─────────────────────────────────────────────────────┤
│ Build WASM plugins from published groups          │
│ Publish @arborium/arborium bundle               │
│ Publish @arborium/lang-* packages               │
└─────────────────────────────────────────────────────┘
       │
       ▼
     done
```

**Key insight**: Groups are published in sequence but can run 2-3 groups in parallel. Each group publishes both crates.io crates AND npm packages together, ensuring version synchronization.

## Two Outputs, Two Registries

### 1. Native Rust Crates → crates.io

- 98 grammar crates organized in 6 groups (`group-a` through `group-f`)
- Plus core crates (`arborium`, `arborium-collection`, `miette-arborium`, `tree-sitter-*`)
- Each group publishes independently: `cargo publish --workspace` in group directory
- **Retry-safe**: cargo warns and skips already-published versions

### 2. WASM Plugins → npm

- All grammars with `generate-component: true` in arborium.kdl
- Built via `cargo-component` for `wasm32-wasip2` from published crates
- Transpiled via `jco` for browser compatibility
- Published as `@arborium/lang-{grammar}` packages
- **Published together with crates.io** in same CI build for version sync

## Publishing Strategy

### crates.io (easy)

Cargo handles already-published versions gracefully - it warns and continues:
```
warning: crate arborium-rust@0.3.0 already exists on crates.io
```

So we can just retry failed builds and it skips what's done.

### npm (needs xtask)

npm is **not graceful** - it hard-fails with `EPUBLISHCONFLICT`:
```
npm ERR! code EPUBLISHCONFLICT
npm ERR! Cannot publish over existing version
```

**This is why we need `xtask publish`** instead of bash scripts:
- Must check if version exists before attempting publish
- Must distinguish `EPUBLISHCONFLICT` (skip, continue) from real errors (fail)
- Must handle retries properly without re-publishing what succeeded

The bash approach `npm publish || echo "failed"` **swallows real errors** - unacceptable.

## What's in Git vs Generated

### Source of Truth (completely separate)

```
sources/                           ← LANGUAGE DEFINITIONS (committed)
├── rust/
│   ├── arborium.kdl          ← SOURCE OF TRUTH
│   ├── grammar/
│   │   ├── grammar.js        ← tree-sitter grammar
│   │   └── scanner.c         ← custom scanner (if any)
│   ├── queries/
│   │   └── highlights.scm    ← highlight queries
│   └── samples/              ← test samples
├── javascript/
├── html/
├── c/
├── cpp/
├── python/
├── [all 98+ languages...]

langs/ (generated, gitignored)
├── group-squirrel/               (Web languages)
│   ├── rust/
│   │   ├── crate/               ← Static linking crate (generated)
│   │   └── npm/                ← WASM plugin package (generated)
│   ├── javascript/
│   ├── html/
│   └── [other web languages...]
├── group-deer/                  (C family)
│   ├── c/
│   ├── cpp/
│   ├── objc/
│   └── [other C family languages...]
├── group-fox/                    (Systems languages)
│   ├── python/
│   ├── go/
│   ├── java/
│   └── [other systems languages...]
├── group-bear/                   (Web frameworks)
│   ├── typescript/
│   ├── tsx/
│   ├── svelte/
│   ├── vue/
│   └── [other web frameworks...]
├── group-wolf/                   (Data/config)
│   ├── json/
│   ├── yaml/
│   ├── toml/
│   ├── xml/
│   └── [other data formats...]
└── group-otter/                  (Scripting/other)
    ├── bash/
    ├── perl/
    ├── php/
    ├── ruby/
    └── [other scripting languages...]
```
```

### Generated (gitignored)

```
langs/group-{animal}/{lang}/
├── crate/                    ← Static linking crate (generated)
│   ├── Cargo.toml            ← GENERATED by xtask gen
│   ├── build.rs              ← GENERATED by xtask gen
│   ├── src/lib.rs            ← GENERATED by xtask gen
│   └── grammar/
│       └── src/              ← GENERATED by xtask gen (tree-sitter generate)
│           ├── parser.c
│           ├── grammar.json
│           └── ...
└── npm/                      ← WASM plugin package (generated)
    ├── Cargo.toml            ← GENERATED for cargo-component
    ├── src/
    │   └── bindings.rs      ← GENERATED bindings
    └── package.json          ← GENERATED npm package
```

### Non-generated crates (hand-written, committed)

These crates don't have `arborium.kdl` and are fully hand-written:
- `arborium` (main crate)
- `arborium-test-harness`
- `arborium-sysroot`
- `arborium-host`
- `arborium-wire`
- `arborium-plugin-runtime`
- `miette-arborium`

## What `xtask gen --version X.Y.Z` Does

1. **Updates core crate versions:**
   - `arborium/Cargo.toml` version = "X.Y.Z"
   - `arborium-collection/Cargo.toml` version = "X.Y.Z"

2. **Generates group workspace files:**
   - `langs/group-{animal}/Cargo.toml` with member crates and version "X.Y.Z"

3. **Generates grammar crate files:**
   - `langs/group-{animal}/{lang}/crate/Cargo.toml` with version "X.Y.Z"
   - `build.rs` with correct C compilation setup
   - `src/lib.rs` with language exports
   - `grammar/src/*` via tree-sitter generate

4. **Generates WASM plugin packages:**
   - `langs/group-{animal}/{lang}/npm/Cargo.toml` for cargo-component build
   - `langs/group-{animal}/{lang}/npm/package.json` for npm publishing
   - `langs/group-{animal}/{lang}/npm/src/bindings.rs` generated bindings

When called without `--version`, uses `0.0.0-dev` (fine for local dev since path deps ignore versions).

## Workflows

### Local Development

```bash
# Edit arborium.kdl, grammar.js, queries, etc.

# Regenerate (uses 0.0.0-dev version, doesn't matter locally)
cargo xtask gen

# Build and test
cargo build
cargo test
```

### Release

```bash
# That's it. Just tag and push.
git tag v0.3.0
git push origin v0.3.0

# CI does the rest:
# 1. Parse version from tag
# 2. xtask gen --version 0.3.0
# 3. Parallel: publish crates.io + build WASM plugins
# 4. After WASM: publish npm
```

## Artifacts Published

| Registry | Package | Count |
|----------|---------|-------|
| crates.io | `arborium` (core with inventory) | 1 |
| crates.io | `arborium-collection` (feature-gated) | 1 |
| crates.io | `arborium-{lang}` (static crates) | 98 |
| crates.io | `arborium-test-harness` | 1 |
| crates.io | `arborium-sysroot` | 1 |
| crates.io | `tree-sitter-patched-arborium` | 1 |
| crates.io | `tree-sitter-highlight-patched-arborium` | 1 |
| crates.io | `miette-arborium` | 1 |
| npmjs.com | `@arborium/arborium` (bundle) | 1 |
| npmjs.com | `@arborium/lang-{lang}` (WASM plugins) | 98 |

## TODO

- [ ] Implement `xtask groups generate` command for optimal 6-group creation
- [ ] Implement inventory system in arborium crate
- [ ] Create arborium-collection crate with feature flags
- [ ] Update `xtask publish` command for:
  - [ ] Group-based publishing (6 groups with individual tags)
  - [ ] Combined crates.io + npm publishing per group
  - [ ] Inventory-aware dependency resolution
- [ ] Update generate caching to tree-sitter-cli output only
- [ ] Standardize wasm-opt settings to -Oz
- [ ] Unify release.yml and npm-publish.yml into single workflow
