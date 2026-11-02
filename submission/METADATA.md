# Technical report metadata

**Status:** technical report draft (outline), not peer reviewed. Not a clinical or device claim.

**Latest paper:** v2.0.0 — [`manuscript/technical_report_v2.md`](../manuscript/technical_report_v2.md).
This file is the v1.0.0 posting metadata, retained for the record.

## Title

**Bioelectric Depolarization as a Topological Defect: A Zero-Free Sparse-Polynomial Engine for
Candidate Bioelectromagnetic Field Synthesis**

## Author

Daniel Doyle — Independent Researcher

## Date

2026-11-02

## Version

Engine: `v1.0.0` (public snapshot). This submission package is posting documentation.

## DOI

Paper v2: **10.5281/zenodo.23898046** — <https://doi.org/10.5281/zenodo.23898046>

Concept DOI: **10.5281/zenodo.23897844** — <https://doi.org/10.5281/zenodo.23897844>

## License

- Manuscript and docs: **CC BY 4.0** — see [`LICENSE-docs`](../LICENSE-docs)
- Code: **MIT** — see [`LICENSE`](../LICENSE)

## Repository

<https://github.com/dsr2812/DEDprotocol-public>

## Posting server

OSF Preprints (post as a technical report draft; recommended). Optional: arXiv `q-bio` if endorsed.

## Subject

Life Sciences → Computational Biology; Medicine and Health Sciences → Oncology (if available).

## Keywords

bioelectricity; topological defects; zero-free arithmetic; base-38; membrane potential;
candidate bioelectromagnetic field; pre-registration draft; reproducibility; cancer modeling

## Abstract

Malignant tissues exhibit two mesoscale physical signatures that conventional molecular
oncology addresses less directly: a progressive degradation of tissue-scale nematic order and a
sustained depolarization of the transmembrane potential. We treat these as coupled defects of an
active nematic tissue substrate and of its dielectric coherence, respectively, and introduce a
computational framework — the Resonant Biophysical Debt Correction (RBDC) framework — that
renders both signatures within a single algebraic representation.

The framework encodes the bioelectric and morphological state of a malignant tissue as a vector
over a zero-free sparse base-38 arithmetic: a zero-free variant of base-38 whose canonical
digits are `1..=37`, with zero digits omitted from the canonical form. The canonical form never
stores a zero digit; zero is the empty (Void) form, and the biological state mapping excludes it
by convention. Malignancy is therefore
modeled not as the annihilation of a signaling field — an apoptosis analog — but as a
displacement within a positive topological spectrum, modeled as reversible through corrective
stimuli. The fractal dimension of the malignant boundary is mapped onto a topological attractor
limit to yield a dimensionless defect-proximity index; the resting membrane potential is
classified against a bipartite dielectric bifurcation threshold; and the resulting
entropic-reversion polynomial is summed with the deficit polynomials of two evolutionary
topological anchors — the elephant, characterized by amplified TP53 copy number, and the shark,
characterized by conjectured neoplastic resistance — to synthesize a deterministic payload of
phase-extraction, impedance-matching, and entropic-reversion carriers.

The framework is offered as a computational heuristic for modelling topological defect patterns
and generating experimentally falsifiable hypotheses. It is implemented as a self-contained
Rust engine (`ded_ark_engine`) with a Python interface.

## What it is

- A deterministic computational engine for mapping two active inputs — membrane potential and
  fractal dimension — to a candidate stimulus specification.
- A pre-registration draft for an in vitro specificity test (not yet deposited).
- A reproducible artifact: Rust tests, CLI smoke test, and CI. An external Lean certificate
  for the balanced arithmetic lane and a value-layer parity receipt are reported but not
  bundled.

## What it is not

- Not a treatment, cure, safety claim, or clinical claim.
- Not a claim that the arithmetic is biology.
- Not a claim that the carrier has been validated; the lab test has not been run.
- Not a claim that 456 Hz is “the” biological frequency.

## Competing interests

None.

## Funding

None.

## Data and code availability

Code and manuscript: <https://github.com/dsr2812/DEDprotocol-public> (tag `v1.0.0`). Exploratory
simulation reports are available from the author.

## Contact

dd-drs1-QED@outlook.com
