# Public release note — v1.0.0

**Paper DOI:** [10.5281/zenodo.23897845](https://doi.org/10.5281/zenodo.23897845)

This directory is a **clean public snapshot**. It contains the technical report draft, the Rust
engine, the CLI, the public docs, the in vitro pre-registration draft, and the posting kit. The
development history is not included.

## Included

- `README.md`, `LICENSE`, `LICENSE-docs`, `CITATION.cff`, `.gitignore`, `SECURITY.md`
- `manuscript/` — technical report draft, lab brief, pre-registration draft, frequency provenance
- `rust/` — engine, CLI, tests
- `scripts/rust_bridge.py`
- `docs/` — arithmetic scope, cross-witness, comparative oncology, expert sources, scope map,
  language policy
- `test_data/symbiosis_candidates.csv` — synthetic CLI demo only
- `submission/` — technical report posting kit

## Verify before publishing

```bash
cd rust
PYO3_PYTHON=python3 cargo test --release
# expect: 95 passed, 0 failed

./target/release/ded_ark_cli --help
./target/release/ded_ark_cli --version
./target/release/ded_ark_cli ../test_data/symbiosis_candidates.csv

cd ..
python3 scripts/rust_bridge.py
```

The full development tree (not included) has 169 tests; the public snapshot ships 95. The external Lean
certificate and the external parity harness are reported in the docs but are **not bundled**.

## Publish

1. Create a **new** public repository, e.g. `dsr2812/DEDprotocol-public`.
2. Push this directory as its first commit. Do not push the development repository's history.
   Push the tag explicitly: `git push origin v1.0.0` (some `gh repo create --push` flows push
   only the branch).
3. Update the CI badge and repository URLs in `README.md` and `CITATION.cff` if the public
   repository name differs.
4. Enable GitHub Actions. The workflow runs the Rust tests, the CLI smoke test, and a secret
   scan.
5. In **Settings → Code security and analysis**, enable **Private vulnerability reporting** so
   `SECURITY.md` has a working private channel.
6. Create the `v1.0.0` release from the tag. Attach the source zip if desired.

## Boundary

The paper is a **technical report draft**, not a peer-reviewed preprint. It makes no clinical
efficacy, safety, or treatment claim. The in vitro protocol has not been run. The public repo
should not be described as a treatment, a cure, or a validated device.
