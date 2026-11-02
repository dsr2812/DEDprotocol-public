# One-page AC impedance pre-test — U87-MG

**Status:** pre-registration draft; not yet deposited. The analysis will be locked before data
collection.

**Purpose:** before any field pilot, measure whether U87-MG has a membrane impedance
resonance in the 67–118 Hz band, or at 456/480 Hz. This is a **measurement**, not a treatment
study.

**Why:** simulations of passive membrane, Hodgkin–Huxley, and tissue cable all say the same
thing: 456 Hz is far above the natural membrane resonance; the 67–118 Hz band is the plausible
one. The pre-test decides whether a larger field study has a mechanism.

**Terminology:** 456 Hz is the framework’s seed; 480.6 Hz is the computed 456-channel carrier
for the reference profile. This pre-test measures both.

---

## The ask

- **Cells:** U87-MG (one line; MCF-10A optional later).
- **Method:** whole-cell patch clamp; small-signal AC impedance.
  - Current clamp: inject a small sinusoidal current (5–10 pA) at each frequency, record Vm.
  - Or voltage clamp: small AC command (±5 mV), record current.
  - Use a lock-in amplifier or the amplifier’s impedance tool.
- **Frequencies:** 39, 67, 78, 117, 118, 144, 420, 456, 480.6 Hz.
  - A log sweep 10–1000 Hz is welcome; the listed points are the analysis set.
- **Temperature:** 37 °C, recorded.
- **n:** 8–12 cells, paired within cell (same cell swept through the list).
- **Readouts:** |Z| and phase at each frequency; membrane capacitance and access resistance;
  stable resting Vm.
- **No field exposure. No patients. No genetic modification.**

---

## Controls

- **Positive control:** 67/78 Hz should show the HH-like peak. If it does not, the preparation
  or the measurement is invalid.
- **Reference:** 144 Hz.
- **Negative controls:** 420, 456, 480.6 Hz. If these look like 144, the 456/480.6 Hz carrier
  hypothesis is not supported.
- **Optional:** 1947 Hz as a far-tail control.

---

## Analysis and decision rule

- Compute the paired log-ratio `log(|Z(f)| / |Z(144)|)` for each cell.
- **Resonance detected** if the 95% CI of the mean log-ratio excludes 0 (two-sided;
  equivalently, the ratio CI excludes 1).
- Report the ratio and CI, not just p-values.
- Pre-specify the frequency list, reference, and decision rule in the pre-registration deposit
  before the run.

**Expected effect sizes from the model:**

| pair | impedance ratio | cells for 80% power (σ=0.2) |
|---|---:|---:|
| 67 / 144 | 2.11 | 2 |
| 78 / 144 | 2.00 | 2 |
| 118 / 144 | 1.28 | 6 |
| 456 / 480 | 0.999 | not detectable |

For a realistic 28% effect at 118 Hz, 6 cells; use **8–12** for safety. A 5% effect would need
>100 cells, so it is not a pilot.

---

## What the result means

- **67–118 Hz peak, no 456 peak** → the membrane resonates in that band; 456 is not supported.
  You have a mechanism and a reason to redesign the carrier.
- **456 peak** → you have a mechanism and a reason to run the field pilot.
- **No peak anywhere** → the cell model is wrong; fix the measurement before any pilot.

---

## What this is not

- Not a treatment, not a cure, not a clinical claim.
- Not a safety claim.
- Not a field-exposure study.
- Not a test of the 40-well protocol; it is the pre-test that decides whether that protocol is
  worth running.

---

## Deliverables from the lab

- |Z| and phase per cell per frequency.
- Raw traces and analysis file.
- A short methods note: amplifier, pipette solution, temperature, access resistance, n.
- Data shared anonymized; analysis code archived.

---

## Cover note for a lab

> We are looking for a small, low-burden patch-clamp pre-test: 8–12 U87-MG cells, AC impedance
> at a handful of frequencies, no field exposure and no genetic modification. It decides
> whether a larger bioelectric field study has a plausible mechanism. The decision rule and
> analysis are pre-specified in a pre-registration draft, to be deposited before data
> collection. A null is a useful result; it saves the cost of the larger study.
