# From Bioelectric State to a Falsifiable AC Impedance Pre-Test

*Technical report draft on a candidate bioelectromagnetic specification generator.*

> **Status: technical report draft — not a scientific paper, not peer reviewed.** This report
> describes a deterministic specification generator and a proposed in vitro pre-test. It does
> **not** claim a biological mechanism, clinical efficacy, safety, or a validated device. The
> pre-test has not been run.
>
> **Public code:** <https://github.com/dsr2812/DEDprotocol-public>
>
> **Previous version (v1):** <https://doi.org/10.5281/zenodo.23897845>

---

## 1. What this report is — and is not

This is a technical report about a **specification generator**. It maps two measured tissue
parameters — resting membrane potential (RMP) and boundary fractal dimension (FD) — to a candidate
bioelectromagnetic stimulus specification. The mapping is deterministic and implemented in Rust.
The report also describes a small in vitro AC impedance pre-test that can test one part of the
hypothesis.

It is **not** a scientific paper in the sense of a mechanistic theory. It does not claim that the
mapping is biologically true, that the carrier frequencies are natural resonances, or that the
engine predicts a clinical outcome.

**What it is:**

- A deterministic specification generator.
- A way to turn a speculative mapping into a concrete, falsifiable pre-test.
- A public code artifact with a test suite.
- A pre-registration draft for a small lab measurement.

**What it is not:**

- A mechanistic theory of cancer or tissue repair.
- Evidence that the mapping is correct.
- A clinical study, a treatment, or a validated device.
- A claim that 456 Hz is a natural biological resonance.
- A claim that base-38 has physical significance.

The central testable question is narrow:

> **Does U87-MG show a membrane impedance resonance in the 67–118 Hz band?**

That question can be answered with a patch clamp and a small number of cells. It does not require
the full framework to be true.

### 1.1 Context, not validation

The biological premises are real. Topological defects in epithelia can localize cell death and
extrusion [1]. Membrane potential is a documented correlate of neoplasia [4,16,17]. TTFields show
that electric fields can affect tumor cells in the 100–300 kHz band [2,3,12]. Bioelectricity is an
active research area [10,11].

None of that validates the mapping in this report or the 456 Hz carrier. It motivates the
question, not the answer. The framework borrows real biophysics as **input**, not as proof.

---

## 2. The testable question

The 67–118 Hz band comes from exploratory simulations: passive membrane, Hodgkin–Huxley, and tissue
cable models. Those simulations are **not** part of the evidence for the framework. They are the
reason to measure.

The 456 Hz seed comes from a compiled database of alternative bioelectromagnetic frequencies. It
is **not** a peer-reviewed oncological constant. The engine uses it as a seed; the computed
456-channel carrier is 480.6 Hz for the reference profile. The pre-test measures 456 and 480.6 Hz
as controls. It does not assume they are natural resonances.

If the pre-test shows **no** 67–118 Hz resonance, the 456/480.6 Hz carrier hypothesis is not
supported and the larger field study should not be run. If it shows a resonance, the field study
has a mechanism to test.

The pre-test design is in `submission/PROPOSED_EXPERIMENT.md`: 8–12 U87-MG cells, AC impedance at
39, 67, 78, 117, 118, 144, 420, 456, and 480.6 Hz, paired within cell. The decision rule is the 95%
CI of the mean log-ratio `log(|Z(f)| / |Z(144)|)` excluding 0 (equivalently, the ratio CI excludes
1). A null is a useful result.

The larger field protocol is separate and has not been run. It is a pre-registration draft, not a
clinical study.

---

## 3. The computational mapping

The engine takes a profile `(v, D, λ, ν)` — resting membrane potential, boundary fractal dimension,
lacunarity, and burst frequency — and produces a candidate payload specification: carrier
frequencies, pulse-train duty cycles, pulse counts, amplitudes, and an encoded entropic-reversion
value.

Three things are true about that mapping:

- **It is deterministic.** The same input produces the same output. The public snapshot ships 95
  Rust tests. Determinism is a property of the code; it is not evidence about biology.
- **The arithmetic is an encoding.** The zero-free base-38 representation is a lossless,
  reversible way to store integers and polynomials. It is a framework hyperparameter. Another
  radix would change the strings, not the underlying values. It adds no biological truth.
- **The mapping is a modeling choice.** The step from a biological feature to an integer is a
  framework-defined quantization. It is not a theorem about tissue. The engine is a specification
  generator, not a mechanism.

The elephant and shark polynomials (`1·38` and `3 + 1·38`) are an **optional encoding layer**. They
are structural choices, not measured animal resonances. They are inspired by comparative oncology
[13,14], though the evolutionary interpretation is contested [15]. They can be removed without
changing the pre-test. The core testable question does not depend on them.

The engine's output is a candidate specification. It is not a prescription, a treatment plan, or a
prediction of clinical outcome.

---

## 4. Provenance of the constants

The framework mixes constants of different status. This report separates them explicitly.

**Grounded inputs (peer-reviewed measurements):**

- Malignant vs healthy RMP: −7.0 ± 1.3 mV vs −23.0 ± 1.4 mV in human liver cancer stem cells vs
  normal liver stem cells [6].
- Rodent corroboration: Morris 7777 hepatoma and mouse fibrosarcoma vs normal counterparts [7].
- Tumor-interface fractal dimension: 1.79 ± 0.09 in glioblastoma multiforme [8].
- Healthy fractal-dimension range: ≈1.66–1.69 [9].

**Mathematical constants:**

- The tribonacci constant 1.839 is the real root of `x³ − x² − x − 1 = 0`.
- The 19.47° tetrahedral angle is `arcsin(1/3)` exactly.

**Model threshold:**

- The −19 mV gate is a model threshold inside the documented depolarization range. It is
  cell-type dependent. It is not a universal biological constant [4].

**Heuristic seeds (not peer-reviewed oncological constants):**

- 456.383 Hz, approximated as 456 Hz, from a compiled alternative bioelectromagnetic frequency
  database. Its mapping to biological effect is **unmeasured**.
- 39 Hz burst reference, 144 Hz reference carrier, and 41% lacunarity are framework reference
  values, not measured universal constants.

The 456 Hz seed is the most important provenance problem. It is an alternative-medicine number,
not a natural membrane resonance. The simulations point to 67–118 Hz, not 456 Hz. The pre-test is
designed to measure the membrane directly, with 456/480.6 Hz as controls.

---

## 5. What the mapping does not claim

- **No physical pathway.** There is no proposed mechanism from “fractal dimension = 1.79” to “emit
  480.6 Hz.” The mapping is an encoding, not a mechanism.
- **No evidence from determinism.** A deterministic mapping can be arbitrary. Determinism is not
  truth.
- **No biological role for the arithmetic.** Base-38 is a storage choice. It does not make the
  biology more likely.
- **No clinical prediction.** The engine does not predict response, survival, or safety.
- **No natural resonance claim for 456 Hz.** It is a heuristic seed. The pre-test measures it as a
  control, not as an established resonance.
- **No claim that the elephant/shark anchors are measured biology.** They are structural encoding
  choices.

---

## 6. What would falsify the framework

The framework is deliberately narrow enough to fail.

- **Primary:** no 67–118 Hz membrane impedance resonance in U87-MG. Then the membrane-resonance
  hypothesis is not supported, and the 456/480.6 Hz carrier hypothesis is not supported.
- **Secondary:** 456/480.6 Hz indistinguishable from the 420 Hz off-harmonic control. Then the
  carrier-specificity claim is not supported.
- **Stage 2:** the computed payload indistinguishable from a bare carrier of the same time-averaged
  power. Then the pulse-train structure claim is not supported.
- **A null is publishable.** It does not prove the absence of an effect; it fails to detect the
  pre-specified effect at the stated power.

---

## 7. Limitations

- **No mechanism.** The report describes a mapping and a measurement, not a causal pathway.
- **No in vivo or clinical data.** The pre-test is in vitro. No patients, no animals.
- **Single cell line, pilot scale.** U87-MG only; 8–12 cells. Small effects may be missed.
- **The mapping is arbitrary.** The choice of inputs, encoding, anchors, and carriers is a modeling
  decision. Another designer could choose differently.
- **Base-38 is a hyperparameter.** It is not a law and has no claimed physical significance.
- **456 Hz is a heuristic seed.** It comes from a non-mainstream source, and the simulations point
  away from it.
- **The elephant/shark anchors are optional.** They are not required for the pre-test.
- **The protocol has not been run.** No efficacy, safety, or device claim is made.

---

## 8. Conclusion

The value of this report is not that it proves a mechanism. It is that it turns a speculative
mapping into a concrete, falsifiable pre-test.

The next step is the lab, not another paper. If the pre-test shows no 67–118 Hz resonance, the
framework is not supported and the negative result should be reported. If it shows a resonance,
there is a mechanism to test in a larger study.

That is the whole claim: a specification generator, a pre-test, and a result either way.

---

## References

[1] Saw, T.B., et al. Topological defects in epithelia govern cell death and extrusion. *Nature* 544, 212–216 (2017).

[2] Kirson, E.D., et al. Alternating electric fields arrest cell proliferation in animal tumor models and human brain tumors. *Proc. Natl. Acad. Sci. U.S.A.* 104, 10152–10157 (2007).

[3] Stupp, R., et al. Effect of tumor-treating fields plus maintenance temozolomide vs maintenance temozolomide alone on survival in patients with glioblastoma: a randomized clinical trial. *JAMA* 314, 2535–2543 (2015).

[4] Yang, M. & Brackenbury, W.J. Membrane potential and cancer progression. *Front. Physiol.* 4, 185 (2013).

[5] Levin, M. Bioelectric signaling: reprogramming cells and tissues to control morphogenesis and regeneration. *Annu. Rev. Cell Dev. Biol.* 33, 203–240 (2017).

[6] Bautista, W., Lipschitz, J., McKay, A. & Minuk, G.Y. Cancer stem cells are depolarized relative to normal stem cells derived from human livers. *Ann. Hepatol.* 16, 297–303 (2017). doi:10.5604/16652681.1231590.

[7] Binggeli, R. & Cameron, I.L. Cellular potentials of normal and cancerous fibroblasts and hepatocytes. *Cancer Res.* 40, 1830–1835 (1980).

[8] Sánchez, J. & Martín-Landrove, M. Morphological and fractal properties of brain tumors. *Front. Physiol.* 13, 878391 (2022).

[9] Battalapalli, D., Vidyadharan, S., Prabhakar Rao, B.V.V.S.N., Yogeeswari, P., Kesavadas, C. & Rajagopalan, V. Fractal dimension: analyzing its potential as a neuroimaging biomarker for brain tumor diagnosis using machine learning. *Front. Physiol.* 14, 1201617 (2023).

[10] Mathews, J., Erickson, P., Kuchling, F., Jawanda, N., Pio-Lopez, L., Pai, V.P. & Levin, M. Meeting review: “National Cancer Institute Conference on Cancer Bioelectricity” September 12, 2024. *Bioelectricity* 7(1), 94–104 (2025). doi:10.1089/bioe.2024.0049.

[11] Li, K., Zhang, Y., Qian, Y., Qin, H., Zhang, H., Li, C., Peng, C., Zhang, J. & Feng, S. Tumor electrophysiology in precision tumor therapy. *iMetaMed* 1(1) (2025). doi:10.1002/imm3.70002.

[12] Khagi, S., Kotecha, R., Gatson, N.T.N., Jeyapalan, S., Abdullah, H.I., Avgeropoulos, N.G., Batzianouli, E.T., Giladi, M., Lustgarten, L. & Goldlust, S.A. Recent advances in Tumor Treating Fields (TTFields) therapy for glioblastoma. *The Oncologist* 30(2), oyae227 (2025). doi:10.1093/oncolo/oyae227.

[13] Abegglen, L.M., et al. Potential mechanisms for cancer resistance in elephants and comparative cellular response to DNA damage in humans. *JAMA* 314, 1850–1860 (2015). doi:10.1001/jama.2015.13134.

[14] Marra, N.J., et al. White shark genome reveals ancient eumetazoan genes in elasmobranchs. *BMC Genomics* 20, 697 (2019).

[15] Moore, T. Peto's paradox and elephant TP53 retrogenes: cancer or reproduction? *Reproduction* 170(2), e250254 (2025). doi:10.1530/REP-25-0254.

[16] Levin, M. Bioelectrical approaches to cancer as a problem of the scaling of the cellular self. *Prog. Biophys. Mol. Biol.* 165, 102–113 (2021). doi:10.1016/j.pbiomolbio.2021.04.007.

[17] Chernet, B.T. & Levin, M. Transmembrane voltage potential is an essential cellular parameter for the detection and control of tumor development in a *Xenopus* model. *Dis. Model. Mech.* 6(3), 595–607 (2013). doi:10.1242/dmm.010835.

---

## License

Text and figures: **CC BY 4.0** — see [`LICENSE-docs`](../LICENSE-docs). Code: **MIT** — see
[`LICENSE`](../LICENSE). This is a theoretical/computational technical report draft; it makes no
clinical efficacy, safety, or device claim.
