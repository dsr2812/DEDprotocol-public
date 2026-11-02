# Lab Brief — A Pre-Specified Test of 456/480.6 Hz Carrier Specificity

**Status:** collaboration request. The protocol is a pre-registration draft, not a completed study.
**Full protocol:** [`supplementary/in_vitro_falsifiability_protocol.md`](supplementary/in_vitro_falsifiability_protocol.md)
**Framework:** theoretical/computational; no clinical efficacy claim.

---

## The ask

We are looking for a cell-biology or bioelectricity lab to run a **blinded, pre-specified,
pilot-scale in vitro test** of one specific claim:

> The framework’s 456 Hz seed shifts the resting membrane potential of a malignant line
> differently from a 420 Hz field of identical amplitude and waveform. The computed
> 456-channel carrier is 480.6 Hz; the pre-test measures both 456 and 480.6 Hz.

This is a **specificity test**, not a therapeutic efficacy study. A null is a publishable
**negative result under the stated power**; it does not prove the absence of an effect.

## What is tested

**Stage 1 — carrier specificity**

| Arm | Frequency | Duration | n per line |
|---|---|---|---|
| Target (seed) | 456 Hz | 4 h | 8 wells |
| Off-harmonic control | 420 Hz | 4 h | 8 wells |
| Sham | no field | 4 h | 8 wells |

- **Terminology:** 456 Hz is the framework’s seed; 480.6 Hz is the computed 456-channel carrier
  for the reference profile. Stage 1 tests the seed. The AC impedance pre-test measures both
  frequencies; Stage 2 tests the computed payload.
- **Cells:** U87-MG (malignant; baseline RMP measured in the pilot, not assumed) and MCF-10A
  (non-tumorigenic epithelial).
- **Readouts:** ΔRMP by FluoVolt **and** patch clamp; ROS at 4 h; viability at 24 h as an
  auxiliary safety readout.
- **Controls:** temperature logged; active arms invalidated if ΔT ≥ 0.5 °C; blinded allocation;
  plate-position randomization; operators blind to arm.

**Stage 2 — pulse-train structure (optional second stage)**

The **sub-kilohertz portion tested here** (dual 144/456 Hz seed channels + snap/gate trains;
the computed 456-channel carrier is 480.6 Hz) is compared with a bare 456 Hz continuous wave, a
structure-and-power control, and sham. This tests
whether the computed structure is distinguishable from a bare carrier and from a power-matched
mismatched structure. It does **not** test the 33 THz / 0.1 THz ratchets, the 39 Hz destructive
carrier, or the optional TME block.

The 456 Hz CW control is **RMS-matched** to the payload by total time-averaged power, not by
duty-weighted mean amplitude. Because the payload’s 456-channel carrier is 480.6 Hz, this is a
structure contrast, not a pure 480.6 Hz carrier contrast. The Stage 2 control exchanges the duty
cycles and scales both amplitudes by one common factor to match total power, so amplitude and
timing both change; it is not a pure timing control. Per-channel residuals are reported.

**Spectrum:** rectangular gating produces sidebands. The reference payload contains an exact
456.0 Hz sideband (480.65 − 2×468.32) as well as the 480.6 Hz carrier. The amplifier output
spectrum is measured and archived before the run; the 456/420 arms are interpreted against that
measured spectrum, not against the carrier labels alone.

## What would falsify the claim

- **Stage 1:** if the adjusted `456 Hz − 420 Hz` contrast in U87-MG is not significant
  (two-sided α = 0.05, corrected over the pre-specified set), the resonant-window claim is
  **not supported** at the stated power.
- **Stage 2:** if the `payload − 456 Hz CW` contrast is not significant, the pulse-train
  structure claim is **not supported** at the stated power.
- A significant effect in the healthy MCF-10A line does not support the specificity prediction.
- A significant shift in the opposite direction does not support the directional claim.

All decision rules are written down before data collection. Post hoc changes break the
pre-specification.

## What the lab needs

- Cell culture for U87-MG and MCF-10A (authenticated, mycoplasma-tested, passage-matched).
- One exposure modality, fixed for the study:
  - Helmholtz coil at 1.0 mT RMS with calculated induced E-field at the well surface, **or**
  - capacitive-contact electrode plate with comparable field.
- Temperature logging at the well; sham plate physically separated (≥ 2 m).
- FluoVolt voltage-sensitive dye imaging **and** automated patch clamp (or a lab with both).
- ROS assay (DCFH-DA / CellROX; MitoSOX optional).
- Blinded allocation and readout; a named PI responsible for the pre-registration deposit.

**Not required:** patients, animal ethics, clinical approvals. The study is in vitro. Standard
cell-culture biosafety approvals apply.

## What the lab gets

- The pre-registration draft, analysis plan, and decision rules.
- The computational payload values for Stage 2, computed from the engine and archived before
  programming.
- A co-authored study if the lab runs it; authorship order and data rights to be agreed in
  writing before data collection.
- A falsifiable, publishable result either way. A null is not a failed collaboration; it is the
  answer.

## What we are not claiming

- Not a treatment, not a cure, not a clinical claim.
- Not a safety claim.
- Not a claim that 456 Hz is “the” biological frequency.
- Not a claim that base-38 has physical significance.

The claim under test is narrow: **does this specific carrier act differently from an
amplitude-matched off-harmonic control in a depolarized cell line?**

## Next step

1. A 30-minute call with the PI and whoever would run the exposure/readouts.
2. The lab reviews the protocol for feasibility, dosimetry, and cell-line conditions.
3. Lock the protocol and deposit it on OSF (or an equivalent registry) **before** data collection.
4. Run Stage 1. Report the pre-specified contrasts, effect sizes, and CIs — including a null.

**Contact:** open an issue or discussion on the repository:
`https://github.com/dsr2812/DEDprotocol-public`

---

*This brief is a collaboration document. It is not medical advice and makes no clinical claim.
The full protocol is the authoritative document.*
