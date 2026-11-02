# DEDprotocol — Scope and Evidence

**Purpose.** One page that says what the paper can claim, what it can only cite, what is a model
choice, and what is unknown. This is a scope map, not an accusation: it protects the work from
being misread as a medical claim. If a sentence in the paper is not supported by a row here, it
does not go in the paper.

**Honesty labels:** 🟢 measured / verified · 🟡 partial / model / cited input · 🔴 gap /
hypothesis · ⚪ unknown / out of scope.

---

## 1. What is verified now

| Claim | Receipt | Label |
|---|---|---|
| The zero-free sparse base-38 encoding is deterministic, lossless and reversible. | Public Rust kernel tests; `cd rust && cargo test --release` → **95 passed, 0 failed** (with `PYO3_PYTHON=python3`). | 🟢 |
| The canonical form never stores a zero digit; zero is the empty (Void) form; the biological mapping excludes it by convention. | Public Rust kernel tests (`ZeroFree38::from_u128(0)` is the Void; the test suite covers zero). | 🟢 for the representation; 🟡 for the biological convention (modeling choice) |
| The DED engine is deterministic: same input → same output. | Public Rust kernel tests: **95 passed, 0 failed**. A separate 480-profile value-layer parity run against an external balanced-lane port reported **2,903 checks, 0 mismatches**; that harness is not bundled. The legacy Python pipeline is **not** part of this public release. | 🟢 for the public Rust engine; 🟡 for the external parity report; ⚪ for the legacy Python pipeline |
| The CLI runs and emits a deterministic Base-38 inversion vector per candidate row. | Public Rust tests plus the CI CLI smoke test (`--help`, `--version`). | 🟢 |
| An **external** Lean certificate covers the **balanced** base-38 arithmetic (signed digits −18..=19), not the biology and not the exact zero-free dialect. | A separate external project reports 159 theorem/lemma declarations, 0 sorries, Lean 4.33.1. **The certificate source is not included in this repository and was not re-run for this release.** The zero-free dialect is covered by the public Rust tests and the value-level cross-witness. | 🟡 external; ⚪ for biology |
| DED's `ZeroFree38` and a tabulated bijective base-38 implementation agree at the **value** layer on the cross-witness samples; their digit forms differ. A separate external Lean certificate covers the balanced lane. | [`BASE38_CROSS_WITNESS.md`](BASE38_CROSS_WITNESS.md). The DED side is checked by public tests; the other lanes are external. | 🟢 for the DED side; 🟡 for the external comparison |

For the exact arithmetic scope, see [`FORMAL_ARITHMETIC.md`](FORMAL_ARITHMETIC.md).

## 2. What is a cited input, not a DED measurement

These are the biological anchors the paper uses. They are peer-reviewed measurements from the
literature. The paper must present them as **inputs**, not as results of this framework.

| Input | Value | Source | Label |
|---|---|---|---|
| Malignant vs healthy resting membrane potential | −7.0 ± 1.3 mV vs −23.0 ± 1.4 mV (n = 30, p < 0.01) | EpCAM⁺ cancer stem cells vs normal liver stem cells, human HCC [12] | 🟡 cited |
| Rodent corroboration | Morris 7777 hepatoma −19.8 ± 7.1 mV vs hepatocytes −37.1 ± 4.3 mV; mouse fibrosarcoma −14.3 ± 5.4 mV vs corneal fibroblasts −42.5 ± 5.4 mV (p = 0.001) | [13] | 🟡 cited |
| Tumour-interface fractal dimension | 1.79 ± 0.09 (glioblastoma multiforme) | [14] | 🟡 cited |
| Healthy fractal-dimension range | ≈1.66–1.69 | [15] | 🟡 cited |
| Lacunarity | 41% framework reference | synthesis source (lung squamous-cell carcinoma nuclei) | 🟡 carried metadata; not used in the canonical payload |
| Characteristic burst frequency | 39 Hz; sets ŠÀR only | synthesis source | 🟡 only the 39 Hz destructive carrier; not in the tested sub-kHz payload |
| Topological defects in epithelia | defect-localized death/extrusion | [1] | 🟡 cited |
| Membrane potential and cancer progression | depolarization correlate | [4] | 🟡 cited |
| TTFields | 100–300 kHz, mitoinhibitory | [2,3] | 🟡 cited |
| Peto's paradox / elephant TP53 / shark genome | evolutionary resistance anchors | [9–11] | 🟡 cited |

**Rule:** none of these numbers becomes “DED measured X.” The paper can say “using the published
anchor X [ref], the engine computes Y.”

See [`EXPERT_SOURCES.md`](EXPERT_SOURCES.md) for the exact expert citations, what they support,
what they challenge, and what the search did not find.

## 3. What is a model choice, not a law

| Choice | Status | What the paper may say |
|---|---|---|
| Radix 38 | 🟡 framework hyperparameter | “a deliberate zero-free sparse encoding; the sweep shows the properties hold for other radices; no physical significance is claimed.” |
| Void exclusion from biological states | 🟡 modeling convention | “zero is the empty (Void) form; the biological mapping excludes it by convention; this is a choice, not a theorem.” |
| 144 Hz reference, 456 Hz seed | 🟡 scaling anchors | “framework seed references; the computed reference carriers are 155.7/480.6 Hz; no physical significance is claimed.” |
| −19 mV gate | 🟡 model threshold | “a bifurcation threshold in the documented depolarization range; cell-type dependent.” |
| 19.47° tetrahedral angle | 🟢 mathematical identity | `arcsin(1/3)`; the planetary-resonance reading is **not** claimed. |
| 1.839 tribonacci attractor | 🟢 mathematical constant | the real root of `x³ − x² − x − 1 = 0`. |
| Elephant 38 / shark 41 deficit polynomials | 🟡 model encoding | “anchor polynomials chosen to represent the evolutionary anchors; not measured biological quantities.” |
| PE-IM-ER payload | 🟡 deterministic specification | “a candidate specification; not an efficacy claim.” |

## 4. What is hypothesis or unknown

| Claim | Status | What the paper must say |
|---|---|---|
| Sub-kilohertz carriers induce topological reversion in intact tissue. | 🔴 hypothesis | “untested; the framework generates the specification, not the evidence.” |
| The specific carrier frequencies/duty cycles have a biological effect. | 🔴 hypothesis | “falsifiable prediction; no in vitro/in vivo validation in this work.” |
| Base-38 has biological significance. | ⚪ unknown | “the radix is a computational choice; no physical claim.” |
| The framework predicts clinical outcomes. | ⚪ out of scope | “no clinical claim; no safety or efficacy claim.” |
| The legacy DED dataset generalizes to clinical samples. | ⚪ unknown | “a computational dataset, not a clinical cohort.” |
| TTFields and RBDC share a mechanism. | ⚪ unknown | “orthogonal modalities; the mechanism is not claimed.” |

## 5. Legacy dataset numbers — not in this public release

The old Python dataset and its statistics are not included. They are not evidence for
the current paper. The current paper uses the peer-reviewed anchors in §2 and the paper's §6.3. Do not quote
old dataset numbers as the current result.

## 6. Material outside this release

Material from other projects is not evidence for this paper. Origin/lore notes, toy simulations,
unrelated medical material, and unpublished working drafts are excluded from the evidence base. Only
this repository and the cited peer-reviewed literature are evidence.

- Independent audit notes: context only, never a result.
- Origin/lore notes: **exclude**; some contain false or overstated claims.
- Toy simulations: **park**; not evidence.
- Unrelated medical or cell-biology material: **park** until it has a source, a number, and a
  label. Do not merge projects.

## 7. Public release scope

This repository is a **filtered public export**, not the full development history.

**In scope**

- `README.md`, `LICENSE`, `LICENSE-docs`, `CITATION.cff`, `.gitignore`, `SECURITY.md`
- `manuscript/technical_report_draft.md`
- `manuscript/LAB_BRIEF.md`
- `manuscript/supplementary/in_vitro_falsifiability_protocol.md`
- `manuscript/supplementary/frequency_provenance.md`
- `rust/` (engine, CLI, tests)
- `scripts/rust_bridge.py`
- `docs/BASE38_CROSS_WITNESS.md`
- `docs/COMPARATIVE_ONCOLOGY.md`
- `docs/EXPERT_SOURCES.md`
- `docs/FORMAL_ARITHMETIC.md`
- `docs/SCOPE_AND_EVIDENCE.md`
- `docs/WORDS_AND_THEIR_LICENSES.md`
- `test_data/symbiosis_candidates.csv` (synthetic demo only)
- `submission/` (technical report posting kit)

**Out of scope; not included**

- Business, funding, and pitch material.
- Patent and legal drafts.
- Internal review records and audit trails.
- Legacy Python pipeline and legacy datasets.
- Local caches, secrets, build artifacts, and generated images.

**Rule:** none of the out-of-scope documents may be cited in the paper as evidence for efficacy,
safety, or clinical benefit. They may be cited only as context for future plans, with the banner
intact.

## 8. The paper’s one-sentence claim

> Given a profile with two active inputs — membrane potential and fractal dimension — plus two
> carried fields — lacunarity and burst frequency — the engine deterministically produces a
> candidate bioelectromagnetic payload specification; in the canonical payload, lacunarity is
> carried but unused and burst frequency sets only the 39 Hz ŠÀR carrier, which the in vitro
> protocol does not test. The arithmetic model is cross-witnessed against an external bijective
> base-38 lane; a separate external Lean certificate covers the balanced lane. The public DED
> implementation is tested, the biological anchors are cited, and the biological effect is an
> open, falsifiable hypothesis.

If a sentence goes beyond that, it needs a new row in this scope map.

## 9. Reproduce the receipts

```bash
# Rust kernel — public snapshot
cd rust
PYO3_PYTHON=python3 cargo test --release
# expect: 95 passed, 0 failed

# CLI
cargo build --release
./target/release/ded_ark_cli --help
./target/release/ded_ark_cli --version

# Python bridge (builds the extension if stale)
cd ..
python3 scripts/rust_bridge.py
```

The public snapshot ships **95 Rust tests**. The full development tree (not included) has **169**. The
external Lean certificate and the external parity harness are not part of this repository and
must be verified separately before they are cited.

The legacy Python pipeline is not part of this public release, so older development commands are
not published here.
