# Cover letter

**To:** a bioelectricity, cell-biology, or electrophysiology lab
**From:** Daniel Doyle, independent researcher
**Subject:** A small, pre-specified AC impedance pre-test — U87-MG, 8–12 cells

---

Dear colleague,

I am an independent researcher. I have built a deterministic computational framework that maps
two measured tissue parameters — resting membrane potential and boundary fractal dimension — to
a candidate bioelectromagnetic stimulus specification. The engine and the pre-registration draft
are public:

- Repository: <https://github.com/dsr2812/DEDprotocol-public>
- Engine release: `v1.0.0`
- Scope and evidence: `docs/SCOPE_AND_EVIDENCE.md`
- One-page pre-test: `submission/PROPOSED_EXPERIMENT.md`

I am looking for a lab to run a **small AC impedance pre-test**, not the full field study:

- **Cells:** U87-MG.
- **Method:** whole-cell patch clamp; small-signal AC impedance (5–10 pA current or ±5 mV
  voltage command).
- **Frequencies:** 39, 67, 78, 117, 118, 144, 420, 456, 480.6 Hz.
- **n:** 8–12 cells, paired within cell.
- **Controls:** 67/78 Hz positive control, 144 Hz reference, 420/456/480.6 Hz negative
  controls.
- **Decision rule:** paired log-ratio `log(|Z(f)|/|Z(144)|)`; the 95% CI of the mean
  log-ratio excludes 0 (equivalently, the ratio CI excludes 1).
- **No field exposure. No patients. No genetic modification.**

The pre-test is designed to fail fast. If there is no membrane resonance in the 67–118 Hz band,
the 456/480.6 Hz carrier hypothesis is not supported, and the larger field study should not be
run. If there is a resonance, the field study has a mechanism.

The protocol is a pre-registration draft. It will be deposited on OSF and the analysis locked
before any data are collected.

Simulations used to design the pre-test (passive membrane, Hodgkin–Huxley, tissue cable,
Monte Carlo) are exploratory and are not part of the paper's claims. They point to the
67–118 Hz band, not to 456 Hz.

**What you would get:** the pre-registration draft, analysis code, the computational payload
values, and a co-authored study if you run it. Authorship and data terms can be agreed in
writing before data collection.

**What this is not:** not a treatment, not a cure, not a clinical claim, and not a
field-exposure study. It is a measurement that decides whether a larger study is worth doing.

I have no funding for this. The pre-test is small enough for a student project or a pilot slot.
If it is not for you, no problem — if it is, the pre-registration draft and analysis plan are
ready.

Thank you for reading.

Daniel Doyle
Independent researcher
dd-drs1-QED@outlook.com
