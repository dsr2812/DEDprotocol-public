# Supplementary Document S1 — In Vitro Falsifiability Protocol

*Falsifiability Appendix to the RBDC Framework technical report draft (`manuscript/technical_report_draft.md`).*

**Title.** Specificity of the 456 Hz seed and its computed 480.6 Hz carrier on resting membrane potential and reactive-oxygen status in a malignant glioblastoma line versus a non-tumorigenic epithelial line: a blinded, three-arm in vitro protocol.

**Status.** Pre-registration draft. To be deposited (e.g., on OSF) and the analysis locked *before* data collection commences. This document defines the decision criteria in advance; post hoc modifications break the pre-registration.

**Terminology.** “456 Hz” in this protocol is the framework’s **seed/anchor** frequency. The canonical engine modulates that seed to a computed carrier of **480.6 Hz** for the malignant reference profile (§12.2). Stage 1 tests the 456 Hz seed against 420 Hz; the AC impedance pre-test measures both 456 and 480.6 Hz; Stage 2 tests the full computed payload, whose 456-channel carrier is 480.6 Hz, against a 456 Hz continuous wave. The protocol does not claim that 456 Hz and 480.6 Hz are interchangeable.

**Structure.** The protocol is organized in two stages, carrying three negative-control arms. **Stage 1** (§§1–11) isolates the *seed-frequency* claim — does the specific 456 Hz seed act differently from an arbitrary sub-kilohertz stimulus of identical amplitude and waveform? **Stage 2** (§12) tests the *pulse-train structure* claim — does the framework's computed duty cycles and dual 144/456 Hz seed-channel layout add anything beyond the bare 456 Hz continuous wave, and beyond the same structure with corrupted (exchanged) duty cycles? Both stages share the same lines, apparatus, and readouts; the stages differ only in the stimulus arms and their pre-specified decision criteria.

---

## 1. Purpose

The RBDC framework (§4.2 of the technical report draft) specifies a corrective payload anchored on a 456 Hz seed (integer harmonic of the framework's 456.383 Hz synthesis-source reference, §6.3); the computed 456-channel carrier is 480.6 Hz for the reference profile. The framework's central falsifiable claim is not that "electric fields affect cells" — that is established — but that the *specific* 456 Hz seed acts within a resonant window distinct from an arbitrary sub-kilohertz stimulus of the same amplitude. This protocol is designed to test exactly that seed specificity, and only that. It is **not** a therapeutic efficacy study; it cannot and will not claim to validate any clinical intervention.

## 2. Stage 1 — hypothesis and pre-specified decision criteria

### 2.1 Primary hypothesis
For U87-MG (baseline RMP measured in the pilot), a 4-hour continuous exposure to a 456 Hz sinusoidal seed field shifts the resting membrane potential (RMP) toward repolarization (more negative), and this shift differs from that induced by a 420 Hz field of identical amplitude and waveform. Stage 1 tests the seed; the computed 480.6 Hz carrier is measured in the pre-test and tested as part of the Stage 2 payload.

H₁: `μ_ΔRMP(456 Hz) ≠ μ_ΔRMP(420 Hz)` in U87-MG, where `ΔRMP` is the change from baseline after 4 h.

### 2.2 Specificity (secondary) prediction
The same comparison in the non-tumorigenic MCF-10A line shows no differential shift:
H₂: `μ_ΔRMP(456 Hz) = μ_ΔRMP(420 Hz)` in MCF-10A.

### 2.3 Falsification rule (pre-specified; to be locked at deposit)
The framework's resonant-window claim is **not supported** if, in U87-MG, the adjusted contrast `456 Hz − 420 Hz` fails to reach statistical significance (two-sided α = 0.05, corrected for the pre-specified contrast set; see §8). A null result on H₁ — whether because neither frequency has any effect or because both act identically — is a legitimate, publishable **negative result under the stated power**. It does not prove the absence of an effect; it fails to detect the pre-specified contrast.

## 3. Cell lines and culture

| Line | Identity | Role | Rationale |
|------|----------|------|-----------|
| U87-MG | Glioblastoma, human | Malignant (baseline RMP measured in the pilot) | Widely characterized; the framework treats it as depolarized, but the baseline RMP is measured in the pilot rather than assumed; canonical framework reference profile (FD 1.79, §6.3 of the technical report draft) |
| MCF-10A | Breast, non-tumorigenic epithelial, human | Healthy (polarized) | Non-tumorigenic, near-normal polarization; standard healthy-epithelium control |

Culture conditions (both lines, identical protocol): DMEM/F12 (U87-MG) or recommended complete MCF-10A medium, 10% FBS (or defined supplements per MCF-10A standard), 1% penicillin/streptomycin, 37 °C, 5% CO₂, humidified. Passage range 3–12; lines authenticated (STR) and mycoplasma-tested at experiment start. Passage-matched batches only.

## 4. Exposure apparatus and dosimetry

Two independent coupling modalities are permitted (see §10 for caveats); the chosen modality is fixed for the entire study and reported:

1. **Helmholtz coil (low-frequency EMF).** Paired circular coils, equal diameter, driven by a sinusoidal current source. Target: **1.0 mT RMS** at well-center (coil axis). Report the *calculated induced electric field* (V/m) at the well surface; this is the physical quantity of interest and must be stated, not merely the B-field.
2. **Capacitive-contact electrode plate (alternative).** Insulated planar electrodes beneath the well plate, as used in low-frequency field studies, with the applied voltage chosen to produce a target field of comparable magnitude to modality 1.

Arm specificity is achieved by frequency alone: waveform (sinusoid, continuous), amplitude, and geometry are identical across the 456 Hz and 420 Hz arms.

**Sham.** Identical apparatus, no current applied. The sham plate is physically separated from active plates (≥ 2 m) to avoid stray fields; orientation is orthogonal to the coil axis.

**Non-thermal control.** Temperature is logged continuously (in-well probe) for the duration of exposure. The protocol is invalidated if any active arm deviates from sham by more than ΔT ≥ 0.5 °C, since the hypothesis requires non-thermal effects.

## 5. Stage 1 study arms

| Arm | Frequency | Amplitude | Duration | n per line |
|-----|-----------|-----------|----------|------------|
| RBDC target | 456 Hz | fixed per §4 | 4 h | 8 wells |
| Sub-kHz control | 420 Hz | fixed per §4 | 4 h | 8 wells |
| Sham | — | 0 | 4 h | 8 wells |

Allocation: blinded, block-randomized by plate position to control for spatial field inhomogeneity; operators performing readouts are blind to arm.

## 6. Readouts and endpoints

### 6.1 Primary endpoint
**ΔRMP at 4 h** (change from individual-well baseline), measured by two concordant methods:
- **FluoVolt voltage-sensitive dye imaging.** FluoVolt per manufacturer protocol; null-point calibration with gramicidin (0-depolarization reference) and valinomycin (K⁺-dependent reference) to convert fluorescence to millivolts. ≥ 50 cells/well, field-averaged.
- **Automated patch clamp (e.g., Nanion Patchliner).** Current-clamp recording of RMP; ≥ 3 successful seals/well, whole-cell configuration.

Concordance rule: an RMP shift is considered confirmed only if both methods agree in sign. Discordant wells are flagged, not discarded, and reported.

### 6.2 Secondary endpoints
- **Intracellular ROS at 4 h** (DCFH-DA or CellROX; MitoSOX for mitochondrial ROS as an optional sub-study), normalized to sham.
- **Viability at 24 h** (MTT or CCK-8) as an auxiliary safety readout — this protocol does not use viability as a decision criterion.

### 6.3 Timing
T₀ baseline (dye + patch clamp) → 4 h continuous exposure → T₄ immediate post-exposure readout; ROS measured at T₄. Patch-clamp reads are staggered across wells so that every read falls within 15 min of exposure end.

## 7. Experimental procedure (timeline)

1. **Day 0:** Plate cells at standardized density (e.g., 5×10⁴/well); allow 48 h adhesion.
2. **Day 2:** Baseline RMP (FluoVolt + patch clamp) and baseline ROS.
3. **Day 2, T₀:** Begin exposure; continuous 4 h.
4. **T₄:** Immediate post-exposure RMP + ROS.
5. **Day 3:** Viability readout.
6. **Analysis:** locked per §8.

## 8. Statistical design and power

- **Design:** 2 lines × 3 arms × 8 wells. Primary analysis restricted to U87-MG.
- **Model:** one-way ANOVA per line with the pre-specified contrast set: (i) `456 − 420` in U87-MG (the decision contrast), (ii) `456 − Sham` in U87-MG, (iii) `456 − 420` in MCF-10A (specificity). Multiple-comparison adjustment over the pre-specified contrast set (e.g., Holm or Dunnett). Unadjusted contrast p-values are also reported, per pre-registration norms.
- **Power:** no prior effect-size data exist for this regime; this is an explicit pilot. n = 8/arm was set using an uncorrected one-sided calculation (d ≥ 1.3, α = 0.05, ≈80%). After Holm correction over the pre-specified contrasts, the power is lower (roughly 0.55–0.6 by a noncentral-t estimate). The n is fixed *a priori*; results are reported as effect-size estimates with CIs, not as a validated dose-response. A confirmatory study should increase n.
- **Direction:** for the decision contrast in U87-MG, the framework predicts repolarization (ΔRMP more negative at 456 Hz than at 420 Hz). A significant shift in the *opposite* direction does not support the framework's directional claim, and is reported as such.

## 9. Falsification and confirmation rules

| Outcome | Interpretation |
|---------|----------------|
| 456 Hz shifts RMP in U87-MG significantly more than 420 Hz, toward repolarization | Resonant-window hypothesis provisionally supported (one cell line, one readout); does not establish efficacy |
| No significant `456 − 420` difference in U87-MG | **Resonant-window hypothesis not supported** at the stated power |
| 456 − 420 significant in U87-MG but 456 − Sham not significant | Support weakened; requires explanation (control-arm anomaly) and is treated as inconclusive |
| 456 − 420 significant in U87-MG *and* in MCF-10A | Specificity prediction not supported (H₂); effect is non-specific |

## 10. Quality controls and limitations

- **Blinding:** field operators and readout operators are distinct; readout operators blind to arm; unblinding only after analysis lock.
- **Batch effects:** all three arms per line run in the same plate batch; plate randomization covers position effects.
- **Known limitations (stated in advance):** (i) U87-MG is a single GBM line; no generalization to other lines or tissues is claimed. (ii) In vitro RMP measured in monolayer differs from tissue context (no nematic tissue order, no stroma). (iii) The 4 h window tests only acute, non-thermal effects. (iv) Sub-kilohertz EMF exposure via Helmholtz coils induces small E-fields; the reported induced E-field (V/m) is the meaningful dosimetric quantity, and its value governs interpretability. (v) In Stage 1, the framework's pulse-train structure (snap/gate duty cycles, §4.2 of the technical report draft) is intentionally **not** tested — Stage 1 isolates the seed-frequency claim only. Stage 2 (§12) tests the pulse-train structure directly, using the same lines, apparatus, and readouts.

## 11. Relation to the framework

- Technical report §4.2 — carrier and payload structure (what is under test here: the 456 Hz seed; its computed carrier is 480.6 Hz).
- Technical report §6.1 — the TTFields/RBDC orthogonality argument; this protocol concerns the RBDC regime alone.
- Technical report §6.3 — validation status; this document is the concrete falsifiability instrument that §6.3 references.
- Technical report §6.5 — robustness of the *model* (sensitivity analysis); this protocol tests the *physical* specificity of one model output.

---

## 12. Stage 2 — Pulse-train structure of the computed payload

Stage 1 (above) tests whether the *carrier frequency* is specific. Stage 2 tests a second, logically independent claim: that the framework's **computed pulse-train structure** — the dual 144/456 Hz channel layout, the computed duty cycles, and the snap/gate pulse trains — is behaviorally distinguishable from a bare 456 Hz continuous wave (CW) of the same time-averaged power (RMS). This is the claim the framework actually makes in technical report §4.2; Stage 1 deliberately did not exercise it (see §10(v)).

Taken together, the two stages carry three negative-control arms that pre-empt the obvious non-specificity objections: **Arm A** (field-off sham, §4/§5/§12.3) controls for the apparatus and for any effect not requiring a field at all; **Arm B** (off-harmonic 420 Hz sub-kHz control, §5) controls for the null hypothesis that *any* sub-kilohertz field acts identically — that the specific carrier frequency is irrelevant; **Arm C** (structure-and-power control, §12.3) exchanges the duty cycles and scales both amplitudes by one common factor to match total time-averaged power. Because amplitude and timing both change, Arm C tests whether the computed structure is distinguishable from a power-matched mismatched structure; it is **not** a pure geometric-timing control.

### 12.1 Stage 2 hypothesis

H₃: In U87-MG, 4 h of the **sub-kilohertz payload portion** shifts ΔRMP toward repolarization by more than an equal-time-averaged **456 Hz CW**:

`μ_ΔRMP(payload) ≠ μ_ΔRMP(456 Hz CW)` in U87-MG.

**Falsification rule.** If the adjusted contrast `payload − 456 Hz CW` fails to reach statistical significance in U87-MG (two-sided α = 0.05, corrected over the Stage 2 contrast set), the framework's claim that the pulse-train structure is *necessary* is **not supported** at this n — the carrier alone is sufficient within the tested contrast. This is a legitimate, publishable **negative result under the stated power**: the payload structure then is not distinguishable from a single-frequency carrier at this n; it does not prove the absence of a structure effect.

### 12.2 Computed payload parameters (programmed values; reported counts marked)

The payload is synthesized from the canonical engine for the malignant reference profile (RMP −7.0 mV, FD 1.79, lacunarity 41%, NLTL burst 39 Hz), as in technical report §4.2. This protocol tests the **sub-kilohertz portion**: the 144/456 Hz seed channels plus the snap/gate trains. It does **not** test the 33 THz / 0.1 THz ratchets, the 39 Hz destructive carrier, or the optional TME block. The table below is generated from the engine, not hand-tuned; the snap amplitude is computed as 0.692 from the formula in the table.

| Channel | Parameter | Computed value | Derivation |
|---------|-----------|----------------|------------|
| 144 Hz gate carrier | frequency | 155.7 Hz | `144 Hz × (1 + m144)`, `m144 = 3/37` (Shark place-0 coefficient) |
| 456 Hz seed → snap carrier | frequency | 480.6 Hz | `456 Hz × (1 + m456)`, `m456 = 2/37` (Shark + Elephant place-1 coefficients) |
| Gate pulse train | pulse frequency | 252 Hz | `144 Hz × (1 + IM)`, IM = 0.75 |
| Gate pulse train | duty cycle | 75% | IM = 0.75 |
| Gate pulse train | pulse count (reported) | 12 | `\|gate deviation\|` (12 mV) |
| Gate pulse train | amplitude | 0.75 (normalized) | IM |
| Snap pulse train | pulse frequency | 468.3 Hz | `456 Hz × (1 + 1/37)` (Shark place-1 coefficient) |
| Snap pulse train | duty cycle | 7.9% | Shark units coefficient `3/38` |
| Snap pulse train | pulse count (reported) | 2 | Shark digit count |
| Snap pulse train | amplitude | 0.692 (normalized) | Void proximity for FD 1.79: (1.79 − 1.68) / (1.839 − 1.68) = 0.692 |

Both carriers are emitted simultaneously (dual-channel); the gate and snap pulse trains gate their respective carriers at the computed duty cycles. The trains are **continuous rectangular trains** at the stated frequency and duty cycle. The engine's `pulses_per_burst` field is reported as a count but does not define an inter-burst period; **no burst repetition rate is assumed or programmed**. The waveform generator is programmed with frequency, duty cycle, amplitude, and duration only. All values are computed, not tuned; the programming script is archived at pre-registration.

**Spectral caveat.** Rectangular gating produces sidebands. The reference model predicts components including 155.7, 407.7, 456.0, 480.6, and 659.7 Hz; the 456.0 Hz component is an exact sideband (480.65 − 2×468.32), not the 480.6 Hz carrier. The actual amplifier output spectrum must be measured and archived (§12.6); the 420/456 CW controls are not assumed to be pure counterparts until that measurement exists.

### 12.3 Stage 2 arms

**Arm C (structure-and-power control).** This arm delivers the payload's carrier set and pulse counts, with the two computed duty cycles **exchanged** between channels (snap train at the gate's computed 75% duty cycle, gate train at the snap's computed 7.9% duty cycle). Because exchanging duty cycles changes each channel's time-averaged power, the two amplitudes are scaled by **one common factor** so that the **total** time-averaged power matches the true payload; per-channel power residuals are reported. For the reference profile, the common scale is 1.0675, giving gate amplitude 0.8006 and snap amplitude 0.7385. Amplitude and timing therefore both change. The arm is a **structure-and-power** control, not a pure timing control; a significant `payload − mismatched` difference cannot be attributed to the computed timing alone.

| Arm | Stimulus | Description | n per line |
|-----|----------|-------------|------------|
| 456 Hz CW | Continuous 456 Hz sinusoid | Carrier-only condition (Stage 1 control condition, re-run in the same batch) | 8 wells |
| Sub-kilohertz payload | Dual 144/456 Hz seed channels (computed carriers 155.7/480.6 Hz) + snap/gate trains, computed values per §12.2 | The tested sub-kilohertz portion (not the 33 THz / 0.1 THz ratchets, 39 Hz carrier, or TME block) | 8 wells |
| 144 Hz only *(optional)* | Continuous 144 Hz sinusoid | Channel dissociability check — asks whether the 144 Hz gate channel alone carries the effect | 8 wells |
| Mismatched payload (Arm C) | Sub-kilohertz payload with snap/gate duty cycles exchanged; amplitudes scaled by one common factor to match total time-averaged power | Structure-and-power control — same carriers and total energy, but amplitude and timing both change; per-channel residuals reported | 8 wells |
| Sham | — | No current, apparatus identical to §4 | 8 wells |

The 456 Hz CW arm is **RMS-matched** to the payload by total time-averaged power so the comparison isolates structure, not integrated exposure. For the reference profile, the RMS-matched CW amplitude is 0.6780; the duty-weighted mean amplitude (0.6171) would under-expose the CW arm by about 9% RMS / 17% power. Allocation, blinding, and readouts are identical to Stage 1 (§5, §6, §7).

### 12.4 Stage 2 statistical design

- **Design:** 2 lines × 5 arms × 8 wells (or × 4 if the optional 144 Hz arm is dropped). Primary analysis restricted to U87-MG.
- **Contrast set (pre-specified):** (i) `payload − 456 Hz CW` in U87-MG (the decision contrast), (ii) `payload − Sham` in U87-MG, (iii) `payload − 456 Hz CW` in MCF-10A (specificity), (iv) `payload − mismatched payload` in U87-MG (the Arm C structure-and-power contrast). Multiple-comparison adjustment over the pre-specified set; unadjusted p-values reported.
- **Power:** same pilot convention as Stage 1 (§8): n = 8/arm was set by an uncorrected one-sided calculation (d ≥ 1.3, ≈80%); after Holm correction over the Stage 2 contrast set, power is lower. Effect-size estimates with CIs; a confirmatory study should increase n. Not a validated dose-response.
- **Direction:** the framework predicts the payload repolarizes U87-MG more than 456 Hz CW. A significant shift in the opposite direction does not support the directional claim.

### 12.5 Stage 2 falsification and confirmation rules

| Outcome | Interpretation |
|---------|----------------|
| Payload shifts RMP in U87-MG significantly more than 456 Hz CW, toward repolarization | Pulse-train structure claim provisionally supported (one line, one readout); does not establish efficacy |
| No significant `payload − 456 Hz CW` difference in U87-MG | **Pulse-train structure claim not supported** at this n — the bare carrier is sufficient within the tested contrast |
| `payload − 456 Hz CW` significant in U87-MG but `payload − Sham` not | Support weakened; control-arm anomaly; inconclusive |
| `payload − 456 Hz CW` significant in U87-MG *and* in MCF-10A | Specificity not supported; the structure effect is non-specific |
| `payload − mismatched payload` not significant in U87-MG | The specific computed structure is not distinguishable from a power-matched mismatched structure; the geometric-information claim is **not supported** at this n |
| `payload − mismatched payload` significant, but `mismatched payload − Sham` also significant | Structure differs, but amplitude and timing both changed; the responsible feature is not localized. The *computed optimum* claim is weakened. |

### 12.6 Stage 2 quality controls and limitations

- All Stage 1 quality controls (§10) apply unchanged.
- **Duty-cycle verification:** the programmed duty cycles are verified by oscilloscope capture of the amplifier output at the start and end of each exposure day; deviations > 1 percentage point invalidate that day's data.
- **Time-averaged power (RMS) match:** the CW arm is matched to the payload's total RMS; the Arm C mismatched arm is scaled by one common factor to match the payload's total time-averaged power. Per-channel and total residuals are reported and adjusted as covariates in sensitivity analysis. The duty-weighted mean amplitude is not used as the matching target; it under-exposes the CW arm by about 9% RMS / 17% power at the reference profile.
- **Spectral verification:** record the amplifier output spectrum at the start and end of each exposure day. Rectangular gating produces sidebands; the reference payload contains an exact 456.0 Hz sideband (480.65 − 2×468.32) and a 407.7 Hz sideband. The measured spectrum is archived with the duty-cycle capture. If the 456.0 Hz sideband is present at comparable amplitude to the 480.6 Hz carrier, the Stage 2 contrast is interpreted as a structure contrast, not a pure carrier contrast.
- **Limitations specific to Stage 2:** (i) A null on H₃ does not disentangle which structural feature (duty cycle, dual-channel coexistence, or pulse-train structure) is irrelevant — it tests the structure *as computed* in one step; a significant H₃ likewise does not localize the responsible feature. (ii) The Arm C mismatched arm corrupts the duty cycles *by exchange* and scales both amplitudes by one common factor to match total power; amplitude and timing therefore both change. A null there rules out the specific computed structure, but does not test every alternative timing, and a residual effect in the mismatched arm leaves open a graded rather than all-or-nothing role for duty-cycle geometry (see §12.5). (iii) Stage 2 shares Stage 1's single-line, monolayer, acute-window limitations. (iv) The nominal “456 Hz” label is an anchor: the actual carrier is 480.6 Hz, and the reference payload also contains an exact 456.0 Hz sideband. The Stage 2 contrast therefore tests the computed structure against a 456 Hz CW, not a pure 480.6 Hz carrier. A future stage should add a 480.6 Hz CW arm or select controls from the measured spectrum.

### 12.7 Relation to the framework

- Technical report §4.2 — carrier and payload structure (the object of Stage 2).
- Technical report §6.5 — robustness of the *model*; Stage 2 tests the *physical* claim that the computed structure is behaviorally distinct from its carrier.
- The universality coupling (a separate layer not included in this draft; scale-invariance desync modulation of the 456 Hz seed channel) is *not* part of this protocol's fixed payload; a future Stage 3 would compare the payload with and without the desync term.
