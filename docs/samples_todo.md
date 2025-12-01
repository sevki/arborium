# Sample Sourcing TODO

Goal: add real-world, permissively licensed sample files to each grammar’s `info.toml` under `[[samples]]` with attribution (`path`, `description`, `link`, `license`).

## Done
- asm: NASM test `test/struc.asm` (BSD-2-Clause) → `crates/arborium-asm/samples/struc.asm`
- bash: Bash3Boilerplate `main.sh` (MIT) → `crates/arborium-bash/samples/bash3boilerplate_main.sh`
- c: `portfoliocourses/c-example-code/queue_linked_list.c` (MIT) → `crates/arborium-c/samples/queue_linked_list.c`
- java: Spring Petclinic `Vet.java` (Apache-2.0) → `crates/arborium-java/samples/Vet.java`
- python: Rich `examples/dynamic_progress.py` (MIT) → `crates/arborium-python/samples/dynamic_progress.py`

## Still TODO (need permissive samples)
- Remaining languages without `[[samples]]` blocks populated.

## Guidance for adding more
1. Pick one real-world file per language from MIT/Apache/BSD/CC0 repos (avoid trivial “hello world”).  
2. Vendor it under `crates/arborium-<lang>/samples/` and add a `[[samples]]` block with:
   - `path` (relative in this repo)  
   - `description` (what the sample demonstrates)  
   - `link` (upstream source URL)  
   - `license` (upstream license)  
3. Include license text if required by the upstream license.
