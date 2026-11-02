> *Dedicated to the memory of those we have lost, and to those still fighting.*

# ded_ark_engine

[![CI](https://github.com/dsr2812/DEDprotocol-public/actions/workflows/ci.yml/badge.svg)](https://github.com/dsr2812/DEDprotocol-public/actions/workflows/ci.yml)

> **Status:** code snapshot `v1.0.0`; paper revision `v2.0.0`. Theoretical/computational
> technical report. Not peer reviewed. No clinical efficacy, safety, or device claim.
>
> **Latest paper (v2.0.0):** [From Bioelectric State to a Falsifiable AC Impedance Pre-Test](manuscript/technical_report_v2.md)
> · [DOI](https://doi.org/10.5281/zenodo.23898046)
> · [Concept DOI](https://doi.org/10.5281/zenodo.23897844)
> · [Zenodo record](https://zenodo.org/records/23898046)
>
> **Previous version (v1.0.0):** [technical_report_draft.md](manuscript/technical_report_draft.md)
> · [DOI](https://doi.org/10.5281/zenodo.23897845)

A computational heuristic for modelling topological defect patterns in malignant
tissue matrices and generating candidate corrective payload specifications. The
engine models neoplastic transformation as a coupled loss of tissue-scale nematic
order and bioelectric dielectric coherence, and maps the resulting "debt" state
onto a candidate — not validated — bioelectromagnetic payload.

This is a **theoretical / computational framework**: it generates deterministic,
falsifiable specifications from input parameters. A separate external project (not included in
this repository) reports a Lean 4 certificate for the **balanced** base-38 dialect (159
theorem/lemma declarations, 0 sorries); the certificate source is **not bundled and was not
re-run for this release**. The exact DED **zero-free sparse** dialect is a distinct form,
cross-witnessed at the value layer against a bijective base-38 implementation
([`BASE38_CROSS_WITNESS`](docs/BASE38_CROSS_WITNESS.md)) and covered by the public Rust test
suite. The proof covers the arithmetic, never the biology. It makes no claim of clinical
efficacy.

**Scope and evidence:** [`docs/SCOPE_AND_EVIDENCE.md`](docs/SCOPE_AND_EVIDENCE.md) — what the
engine does, what is cited, what is modeled, what is hypothesis, and what is out of scope.

**Expert sources:** [`docs/EXPERT_SOURCES.md`](docs/EXPERT_SOURCES.md) — the peer-reviewed map of
what supports the framing, what challenges it, and what the search did not find.

**Comparative oncology:** [`docs/COMPARATIVE_ONCOLOGY.md`](docs/COMPARATIVE_ONCOLOGY.md) — verified
animal-resistance sources and the boundary: context and caution, not validation of the carrier.

## 1. Theoretical premise

**Resonant Biophysical Debt Correction (RBDC).** Malignancy is treated as a
displacement of two mesoscale order parameters from their healthy reference:

- **Nematic order.** Epithelia behave as an active nematic substrate; the
  malignant boundary departs from scale-invariant geometry (fractal dimension
  rises; lacunarity is a framework reference value). Saw et al. show that
  topological defects localize cell death/extrusion [1]; the extension to a
  propagating cancer defect is a model hypothesis, not an established result.
- **Dielectric coherence.** The resting membrane potential depolarizes toward a
  bifurcation threshold (−19 mV), shifting the tissue into a defective electrical
  regime [2].

The framework's corrective principle is modeled as **topological reversion** — a
return path from the displaced state to the reference state — rather than
annihilation of the state (apoptosis/necrosis analog). See
`manuscript/technical_report_draft.md` for the full theoretical development.

## 2. Zero-Free Sparse Polynomial Arithmetic

The engine encodes biological state as a polynomial in **zero-free sparse base-38
arithmetic** (`ZeroFree38` in `rust/src/core.rs`):

- Canonical digits span `1..=37`; zero digits are omitted from the stored form.
- The canonical form never stores a zero digit; zero is the empty (Void) form. The
  biological state mapping excludes the Void by convention — a modeling choice, not
  a theorem.
- Terms are stored in **sparse canonical form** — sorted by exponent, implied-zero
  coefficients omitted — and normalized through the full-scale digit (37).
- Encoding is a lossless bijection between non-negative integers and canonical sparse
  forms, e.g. `850 = 14 + 22·38`, `535 = 3 + 14·38`.

In the biological mapping, deficit is therefore treated as *displacement within a
positive spectrum*, not as absence — a modeling convention, not a theorem.

## 3. Building the Rust engine

```bash
cd rust
cargo build --release
```

The crate builds two targets:

- `libded_ark_engine` — `cdylib` + `rlib`. The PyO3 extension module is loaded
  directly by Python (see below).
- `ded_ark_cli` — the standalone command-line binary.

For the Python bridge, point the build at the target interpreter:

```bash
export PYO3_PYTHON=python3   # Python 3.11–3.13; CI pins 3.11
cd rust && cargo build --release
```

Test suite (no external test framework):

```bash
cd rust && cargo test --release
```

The public snapshot ships **95 Rust tests**; the full development tree (not included) has
**169**. The difference is modules that are not part of this public release. The external Lean
certificate and the external parity harness are reported in the docs but are **not bundled**.

PyO3 0.23 supports Python 3.11–3.13. On Python 3.14, the build may need
`PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1`; CI pins Python 3.11.

## 4. Command-line interface

The CLI runs the symbiosis inversion pipeline over a candidate CSV, or the
built-in mock candidates, and prints boxed Base-38 inversion telemetry.

```bash
cd rust
./target/release/ded_ark_cli --help
./target/release/ded_ark_cli --version
./target/release/ded_ark_cli                                 # built-in mock candidates
./target/release/ded_ark_cli ../test_data/symbiosis_candidates.csv
```

Exit codes: `0` success, `1` file/parse error, `2` usage error. The output ends with a
theoretical-model note; it is not a clinical or biological claim.

| Argument | Behavior |
|----------|----------|
| *(none)* | runs the built-in mock candidate audit |
| `<csv_path>` | parses candidates, prints one Base-38 inversion vector per row |

CSV columns: `organism, stage, continuous_score, target_frequency_hz`. The bundled
`test_data/symbiosis_candidates.csv` is a synthetic demo, not biological data.

## 5. Python bridge

`scripts/rust_bridge.py` locates the repo root, rebuilds the extension if any
Rust source is stale, and loads the module.

```bash
python3 scripts/rust_bridge.py                       # CLI-style bridge demo
```

The bridge builds the extension with the interpreter running it. On Python 3.14 it sets
`PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` automatically; CI pins Python 3.11.

Programmatic use:

```python
import scripts.rust_bridge as rb

engine = rb.load_engine()                             # PyO3 module: ded_ark_engine

rows = engine.process_csv("test_data/symbiosis_candidates.csv")
# rows: [(candidate_id, raw_beacon_u128, base38_polynomial_string), ...]

print(engine.to_base38(850))                          # "14 + 22·38"

apex = engine.ApexSynthesizer()
apex.parse_deficit("3 + 1·38")                        # 41
payload = apex.synthesize(-7.0, 1.79, 41.0, 39.0)
# payload: dict of carriers, modulation indices, pulse trains, UŠ₂ vector
```

## 6. Apex Topological Anchors and PE-IM-ER synthesis

**Peto's paradox** — neoplastic incidence does not scale with cell number or body
mass — motivates two evolutionary anchors whose proposed resistance is encoded as
deficit polynomials and added to the tumor state [3,4]:

| Anchor | Biological basis | Deficit polynomial | Value |
|--------|------------------|--------------------|-------|
| Elephant | amplified *TP53* copy number (at least 20 copies / 40 alleles, including 19 retrogenes, in *Loxodonta*) | `1·38` | 38 |
| Shark | expanded DNA-repair / apoptosis gene families (conjectural correlate of low neoplasia incidence) | `3 + 1·38` | 41 |

The *TP53* expansion is well documented; whether cancer prevention was the primary
selective driver is debated [5]. The engine's elephant and shark anchors are heuristic
encodings, not measured animal resonances.

**Payload synthesis** (`ApexSynthesizer` in `rust/src/apex_synthesizer.rs`):

1. **Gate classification.** Membrane potential is classified against the
   bipartite dielectric bifurcation threshold (−19 mV).
2. **Defect proximity.** Fractal dimension is normalized against the topological
   attractor limit (`ρ = (D − D₀)/(D* − D₀)`, `D₀ = 1.68`, `D* = 1.839`).
3. **Entropic-reversion vector.** The 456 Hz seed is scaled by defect proximity:
   `UŠ₂ = round(456 × (1 + ρ))`, then encoded as a zero-free base-38 polynomial.
   The burst frequency sets the 39 Hz ŠÀR carrier, not UŠ₂.
4. **Carrier modulation.** Place-0 deficit coefficients modulate the framework's
   144 Hz reference carrier; place-1 coefficients modulate the 456 Hz seed channel.
5. **PE-IM-ER payload.** The tumor polynomial plus both anchor polynomials yield
   the compensated vector: modulated 144/456 Hz carriers (seed channels after modulation), the dielectric snap
   train and gate-push train (duty cycle, pulse count, amplitude), the 33 THz
   elephant ratchet and 0.1 THz shark MPTP carriers (each deficit-modulated), the
   destructive-interference carrier at antiphase π, and the compensated UŠ₂ value.
   An optional TME-coupled block exists in the engine but is not part of the
   canonical reference output. The 19.47 GHz tetrahedral diagnostic carrier
   (`im_hz`) is an internal modulation reference, not part of the sub-kilohertz protocol.

The 144/456 seed-channel frequencies and anchor modulation indices are fixed by the canonical
anchor polynomials; the computed reference carriers are 155.68/480.65 Hz. Only the gate train
(`v`) and the snap amplitude/UŠ₂ (`D`) vary with the input. Lacunarity is carried but unused;
burst frequency moves only the 39 Hz ŠÀR carrier, which the in vitro protocol does not test.

Reference outputs (malignant profile: −7.0 mV, FD 1.79, 41% lacunarity, 39 Hz):

```
144 Hz seed -> carrier : 155.68 Hz   (m144 = 3/37)
456 Hz seed -> carrier : 480.65 Hz   (m456 = 2/37)
snap train     : 468.32 Hz | duty 7.89% | 2 pulses | amp 0.692
gate push      : 252.00 Hz  | duty 75.0% | 12 pulses | amp 0.750
elephant ratchet: 33.89 THz (deficit-modulated)
shark MPTP     : 0.108 THz  (deficit-modulated)
destructive ŠÀR: 39.00 Hz @ antiphase π
UŠ₂ compensated: 14 + 22·38  (= 850)  [tumor 771 + 38 + 41]
```

Healthy profile reference: UŠ₂ = `3 + 14·38` (= 535).

**Spectral note.** Rectangular gating produces sidebands. The reference payload contains an
exact 456.0 Hz sideband (480.65 − 2×468.32) as well as the 480.6 Hz carrier; the 456 Hz CW
control is therefore not a pure carrier-only counterpart. Measure the actual output spectrum
before the lab (see protocol §12.6).

## 7. Repository layout

- `rust/src/core.rs` — `ZeroFree38` zero-free sparse polynomial kernel
- `rust/src/ded_engine.rs` — resonance/audit engine, tumor profiles, gate logic
- `rust/src/symbiosis.rs` — symbiosis candidate inversion
- `rust/src/apex_synthesizer.rs` — Apex anchors, PE-IM-ER payload synthesis
- `rust/src/lib.rs` — PyO3 bindings (`SymbiosisInverter`, `ApexSynthesizer`,
  `process_csv`, `to_base38`)
- `rust/src/main.rs` — `ded_ark_cli` binary
- `scripts/rust_bridge.py` — Python bridge / extension loader
- `manuscript/technical_report_draft.md` — technical report draft (outline)
- `manuscript/LAB_BRIEF.md` — one-page collaborator brief
- `manuscript/supplementary/in_vitro_falsifiability_protocol.md` — pre-registration draft lab protocol
- `manuscript/supplementary/frequency_provenance.md` — provenance of the heuristic anchors
- `docs/WORDS_AND_THEIR_LICENSES.md` — language policy: what each word is licensed to say
- `docs/BASE38_CROSS_WITNESS.md` — ZeroFree38 × bijective base-38 witness: values agree on every sample, forms differ and the difference is named
- `docs/SCOPE_AND_EVIDENCE.md` — claim and evidence map
- `docs/COMPARATIVE_ONCOLOGY.md` — context and boundaries
- `docs/EXPERT_SOURCES.md` — verified expert source map
- `docs/FORMAL_ARITHMETIC.md` — exact arithmetic scope and external-certificate label
- `SECURITY.md` — private vulnerability reporting
- `submission/` — technical report and posting package
- `LICENSE`, `LICENSE-docs` — code MIT; manuscript/docs CC BY 4.0
- `CITATION.cff` — citation metadata

## References

[1] Saw, T.B., et al. Topological defects in epithelia govern cell death and
extrusion. *Nature* 544, 212–216 (2017).

[2] Yang, M. & Brackenbury, W.J. Membrane potential and cancer progression.
*Front. Physiol.* 4, 185 (2013).

[3] Abegglen, L.M., et al. Potential mechanisms for cancer resistance in
elephants and comparative cellular response to DNA damage in humans. *JAMA* 314,
1850–1860 (2015). doi:10.1001/jama.2015.13134.

[4] Marra, N.J., et al. White shark genome reveals ancient eumetazoan genes in
elasmobranchs. *BMC Genomics* 20, 697 (2019).

[5] Moore, T. Peto's paradox and elephant TP53 retrogenes: cancer or reproduction?
*Reproduction* 170(2), e250254 (2025). doi:10.1530/REP-25-0254.

## License

**Code: MIT** — see [LICENSE](LICENSE). **Manuscript and docs: CC BY 4.0** — see
[LICENSE-docs](LICENSE-docs). As stated above: this is a theoretical / computational framework;
it makes no claim of clinical efficacy.
