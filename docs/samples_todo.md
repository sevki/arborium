# Sample Sourcing TODO

Goal: add real-world, permissively licensed sample files to each grammar’s `info.toml` under `[[samples]]` with attribution (`path`, `description`, `link`, `license`).

## Candidates (license checked at repo level)

- asm: yds12/x64-roadmap (MIT) — e.g., `tasks/01-hello-world/hello.asm`.
- c: portfoliocourses/c-example-code (MIT) — small standalone C programs.
- python: danielborowski/fibonacci-heap-python (MIT) — `fib-heap.py`.
- bash, java: still need MIT/Apache/BSD/CC0 sources identified.

## Next steps
1. For each language, pick one small sample file in an MIT/Apache/BSD/CC0 repo.  
2. Add a `[[samples]]` block with:
   - `path` (relative path inside our repo where the sample will live or be vendored)  
   - `description` (what it shows)  
   - `link` (upstream source URL)  
   - `license` (from upstream)  
3. If we vendor the sample into our repo, include the upstream license text if required.

