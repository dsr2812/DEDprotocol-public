# Technical Report Draft — Theoretical Framework

*Accompanying the `ded_ark_engine` Rust CLI and PyO3 bridge.*

> **Status: technical report draft — not a full paper, not peer reviewed.** This document is an
> outline of the framework, posted for transparency and citation. Sections marked "outline" are
> incomplete. It makes no clinical efficacy, safety, or device claim.

## 1. Title

**Bioelectric Depolarization as a Topological Defect: A Zero-Free Sparse-Polynomial Engine for Candidate Bioelectromagnetic Field Synthesis**

---

## 2. Abstract (~250 words)

Malignant tissues exhibit two mesoscale physical signatures that conventional molecular oncology addresses less directly: a progressive degradation of tissue-scale nematic order and a sustained depolarization of the transmembrane potential. We treat these as coupled defects of an active nematic tissue substrate and of its dielectric coherence, respectively, and introduce a computational framework — the Resonant Biophysical Debt Correction (RBDC) framework — that renders both signatures within a single algebraic representation.

The framework encodes the bioelectric and morphological state of a malignant tissue as a vector over a zero-free sparse base-38 arithmetic: a zero-free variant of base-38 whose canonical digits are `1..=37`, with zero digits omitted from the canonical form. The canonical form never stores a zero digit; zero is the empty (Void) form, and the biological state mapping excludes it by convention. Malignancy is therefore modeled not as the annihilation of a signaling field — an apoptosis analog — but as a displacement within a positive topological spectrum, modeled as reversible through corrective stimuli. The fractal dimension of the malignant boundary is mapped onto a topological attractor limit to yield a dimensionless defect-proximity index; the resting membrane potential is classified against a bipartite dielectric bifurcation threshold; and the resulting entropic-reversion polynomial is summed with the deficit polynomials of two evolutionary topological anchors — the elephant, characterized by amplified TP53 copy number, and the shark, characterized by conjectured neoplastic resistance — to synthesize a deterministic payload of phase-extraction, impedance-matching, and entropic-reversion carriers.

The framework is offered as a computational heuristic for modelling topological defect patterns and generating experimentally falsifiable hypotheses. It is implemented as a self-contained Rust engine (`ded_ark_engine`) with a Python interface.

---

## 3. Introduction (outline)

### 3.1 The malignant playbook: breakdown of tissue-level nematic order and dielectric coherence
- Healthy epithelium behaves as an **active nematic**: cells share a preferred orientational axis, and the tissue boundary approximates scale-invariant (self-similar) geometry. Topological defects in this nematic order — regions where orientational symmetry fails — have been shown experimentally to localize cell death and extrusion, coupling tissue-scale order to cell fate [1]. The tissue-scale nematic organization of invasive cancer has been analysed as an active nematic system, with ECM topological defects tracking tumour boundaries [16]; collective cancer invasion is itself a tissue-scale process [20].
- The malignant playbook, as defined here, is the observable sequence by which a tissue abandons this coherence: the fractal dimension of the boundary departs from its healthy reference, lacunarity is treated as rising (a framework reference value; §6.3), and the resting membrane potential depolarizes toward the bifurcation threshold. These are the framework's chosen **mesoscale order parameters**, not single-molecule states, and they are plausible observables for a physics-based model of transformation.
- Malignancy is therefore modeled as a **tissue-scale topological defect pattern embedded in a dielectric substrate** — a defect that is simultaneously morphological (nematic) and electrical (bioelectric). This is a model hypothesis; Saw et al. establish defect-localized death/extrusion, not a propagating cancer defect.

Levin has proposed that cancer can be understood as a failure of large-scale bioelectric patterning: cells disconnect from the collective network that encodes anatomical goals and revert toward unicellular behaviour [28]. This is a perspective, not an established mechanism. The empirically grounded premises used here are narrower: transmembrane voltage is a documented correlate of neoplasia [4,12,13], and bioelectric state can regulate patterning and tumour development in model systems [5,29]. DED does not claim that cancer is a unicellular reversion, that gap-junction loss is universal, or that restoring voltage is sufficient to normalize a tumour. Its claim is the weaker computational one: a tissue-scale order parameter can be mapped to a candidate carrier specification and tested.

### 3.2 Limitation of the current biochemical paradigm
- Molecular oncology primarily reduces neoplastic state to discrete genetic and epigenetic alterations; it has a limited vocabulary for tissue-scale order parameters and for the membrane-level physics that govern collective cell behavior. This is a difference of emphasis, not a claim that molecular oncology ignores tissue organization.
- Physical interventions already exploit mesoscale electric effects, establishing precedent for the approach:
  - **Tumor-Treating Fields (TTFields):** alternating, low-intensity electric fields (~100–300 kHz) applied transdermally; shown to arrest proliferation in vitro and in animal models [2], and to extend survival in glioblastoma in a randomized trial [3]. TTFields demonstrate that external electric fields act on the mesoscale and can modulate tumor behavior — an electrotherapeutic modality independent of molecular targeting.
  - **Terahertz (THz) studies:** spectroscopic evidence of reproducible dielectric and optical contrast between neoplastic and normal tissue [6,7], indicating that malignant transformation manifests as a measurable change in tissue dielectric response.
  - **Bioelectric control:** transmembrane voltage is an established physiological order parameter regulating morphogenesis and regeneration [4,5]; depolarized potentials are a documented correlate of proliferative state in transformed cells [4]. Bioelectricity in cancer is now an active, NCI-recognized research area [17], and Vmem depolarization is a central electrophysiological hallmark [18].
- The synthesis: a framework is required that (i) represents malignancy as a displacement of mesoscale order parameters and (ii) maps that displacement onto a candidate corrective field — motivation for the RBDC framework and its zero-free sparse polynomial arithmetic.

---

## 4. Methodology (The Engine)

### 4.1 Zero-Free Sparse Polynomial Arithmetic (`ZeroFree38`)

> **Formal arithmetic scope.** A separate external project reports a Lean 4 certificate for the
> **balanced** base-38 dialect (159 theorem/lemma declarations, 0 sorries, Lean 4.33.1). The
> certificate source is not included in this release. DED’s `ZeroFree38` is a distinct
> **zero-free sparse** dialect; the DED and tabulated bijective forms are cross-witnessed at the
> value layer (`docs/BASE38_CROSS_WITNESS.md`), and the public DED implementation is covered by 95 Rust tests
> (169 in the full development tree, not included). The certificate covers arithmetic, not the biological model or
> the carrier protocol. Results quoted as integers/frequencies are dialect-invariant; polynomial
> strings are shown in DED’s zero-free sparse form. An external 480-profile **value-layer** parity
> run against a Rust port of the balanced lane returned **2,903 checks, 0 mismatches**; the harness
> executes the Rust port, not Lean, and is not bundled. Full scope: `docs/FORMAL_ARITHMETIC.md`.

- **Generalized construction.** The arithmetic is implemented generically as `BijectiveEncoding<B>` for any radix, with `ZeroFree38` (`B = 38`) as the biological specialization. Determinism, invertibility, and the zero-free canonical form are properties of the general construction, verified by a radix-sweep over `B ∈ {2, 10, 37, 38, 41}`. The choice of 38 is an explicit, replaceable hyperparameter — not a law of the system (see §6.2).
- **Zero-free sparse encoding.** Each state is a polynomial in a zero-free variant of base-38 whose canonical digits span `1..=37`; zero digits are omitted. The map between non-negative integers and canonical sparse forms is bijective, with zero represented by the empty (Void) form. The canonical form never stores a zero digit; a non-zero value never collapses to the Void. The biological state mapping excludes the Void by convention — a modeling choice, not a theorem.
- **Sparse canonical form.** Terms are sorted by exponent; place values with implied-zero coefficients are omitted; the encoding is normalized through the full-scale digit (37). Encoding and decoding are deterministic and lossless between integers and canonical polynomial forms. The mapping from biological features to those integers is a separate framework-defined quantization, not a theorem about the biology.
- **Design rationale.** Biological deficit is modeled not as *absence* (zero) but as *displacement* within a positive spectrum; the Void is reserved for “no state” and excluded from the biological input mapping. The framework's corrective operation is therefore modeled as **topological reversion** — a return path to a lower-order polynomial, not a collapse to zero. This distinguishes the model from additive biochemical metaphors in which loss of function equals deletion. It is a modeling choice, not a theorem.
- **State mapping.** The profile is `(v, D, λ, ν)` — resting membrane potential (mV), boundary fractal dimension, lacunarity (%), and characteristic burst frequency (Hz). The canonical payload uses `v` and `D`; `λ` is carried as metadata and does not enter the canonical payload; `ν` sets only the ŠÀR carrier. The mapping is:
  - **Gate classification:** the potential `v` is classified against the bipartite dielectric bifurcation threshold `v_g = −19 mV`; `v < v_g` is the hyperpolarized (ordered) regime, `v > v_g` the depolarized (defective) regime.
  - **Defect proximity:** the fractal dimension is normalized against the topological attractor limit, `ρ = (D − D₀)/(D* − D₀)`, where `D₀ = 1.68` is the healthy reference and `D* = 1.839` the attractor limit (the tribonacci constant). `ρ` is the dimensionless defect-proximity index.
  - **Entropic-reversion encoding:** the 456 Hz seed is scaled by defect proximity, `UŠ₂ = round(456 × (1 + ρ))`, and encoded as a zero-free base-38 polynomial. The burst frequency sets the destructive-interference (ŠÀR) carrier, not UŠ₂.
  - **ŠÀR carrier:** the burst frequency `ν` is the destructive-interference frequency (`sar_hz = ν`); it is not part of the tested sub-kilohertz payload.

### 4.2 Evolutionary Topological Anchors and Field Synthesis (`ApexSynthesizer`)
- **Peto's paradox** [8]: neoplastic incidence does not scale with cell number or body mass, implying evolved resistance mechanisms in large, long-lived organisms.
- **The elephant anchor.** The Proboscidean genome carries an amplified complement of *TP53* copies — at least 20 copies (40 alleles), including 19 retrogenes, in *Loxodonta* — associated with an enhanced DNA-damage response and proposed as a mechanism of cancer resistance [9]. The framework encodes this as the deficit polynomial `1·38` (value 38).
- **The shark anchor.** Comparative elasmobranch genomics reveals ancient gene families and expansions in DNA-repair and apoptosis-associated pathways, proposed as correlates of the low neoplasia incidence conjectured in sharks [10,11]. The framework represents this as a high-impedance dielectric boundary, encoded as `3 + 1·38` (value 41).
- **What the anchors are not.** The elephant and shark anchors are heuristic encodings of two comparative-biology observations, not mechanistic claims that *TP53* copy number or elasmobranch DNA-repair expansions generate a specific electromagnetic frequency. The elephant *TP53* expansion is well documented [9], but its evolutionary interpretation is contested: Moore (2025) argues that selection on reproduction, not cancer prevention alone, may explain it [21]. The 33 THz "elephant ratchet" and 0.1 THz "shark MPTP" are framework-internal scaling labels, not measured animal properties and not therapeutic targets.
- **Compensatory synthesis.** Deficit coefficients are mapped onto carrier channels by place value: place-0 coefficients modulate the framework's 144 Hz reference carrier; place-1 coefficients modulate the framework's 456 Hz seed channel. The tumor polynomial is summed with both anchor polynomials to yield the **compensated phase-extraction, impedance-matching, and entropic-reversion (PE-IM-ER) vector**, whose coefficients parameterize:
  - the seed-channel frequencies (144 Hz and 456 Hz), whose computed reference carriers are 155.7 and 480.6 Hz after the fixed anchor modulation indices;
  - the two pulse trains — the dielectric snap train and the gate push train — with explicit duty cycles, reported pulse counts, and normalized amplitudes (the gate push at the bifurcation-derived frequency, the snap train driven by the shark anchor's place-0 coefficient); the canonical waveform is a continuous rectangular train, so the count is reported but does not define an inter-burst envelope;
  - the destructive-interference carrier at antiphase π;
  - the 33 THz elephant ratchet and 0.1 THz shark MPTP carriers, each deficit-modulated;
  - the 19.47 GHz tetrahedral diagnostic carrier (an internal modulation reference, not part of the sub-kilohertz protocol);
  - the compensated entropic-reversion polynomial and its integer value.
  - (An optional TME-coupled block exists in the implementation but is not part of the canonical payload described here.)
- **Output.** The synthesizer emits a fully deterministic, parameterized payload — carrier frequencies, duty cycles, pulse counts, amplitudes, the 33 THz / 0.1 THz ratchet carriers, and the compensated UŠ₂ value. It is a *specification*, not an efficacy claim.

---

## 5. Conclusion

The `ded_ark_engine` toolchain — a self-contained Rust CLI with a Python bridge — operationalizes the RBDC framework as a reproducible, auditable computational heuristic. Given a profile with two active inputs — membrane potential and fractal dimension — plus two carried fields — lacunarity and characteristic burst frequency — it deterministically produces a candidate PE-IM-ER payload specification: a fixed 144/456 Hz seed-channel skeleton with computed reference carriers 155.7/480.6 Hz, fixed anchor modulation indices, voltage-dependent gate parameters, and geometry-dependent snap amplitude and UŠ₂. Lacunarity does not enter the canonical payload; burst frequency sets only the 39 Hz ŠÀR carrier, which the in vitro protocol does not test. The output is a candidate specification, not a personalized treatment and not an efficacy claim; its predicted biological effect is an open, falsifiable hypothesis. Each output is a concrete specification against which controlled bioelectric experiments can be designed; the framework's predictions — the explicit mapping from the two active inputs to the gate and snap specification — are the substance of its testability. The engine positions topological defect resolution as a computational organizing principle, not as a demonstrated therapy.

---

## 6. Limitations and Falsifiable Predictions

### 6.1 The carrier-regime discrepancy: TTFields and RBDC as orthogonal modalities
The corrective carriers specified by the framework occupy the sub-kilohertz regime (the 144/456 Hz seed channels and their deficit modulations, giving 155.7/480.6 Hz for the reference profile), three orders of magnitude below the 100–300 kHz band of Tumor-Treating Fields [2,3]. TTFields are established to exert mitoinhibitory effects, hypothesized to act through dielectrophoretic forces on polar macromolecules — notably tubulin — during mitosis (M-phase) [19]. The RBDC framework does not claim to reproduce this mechanism. Its stated hypothesis is distinct: sub-kilohertz bioelectric phase-locking and mechanotransductive coupling, targeting voltage-gated ion-channel gating and tissue tensegrity in the resting (G0/G1) regime. The two modalities are therefore framed as orthogonal — mitoinhibitory (TTFields) versus topologically corrective (RBDC). Whether a sub-kilohertz regime exerts measurable effects on intact tissue is an open, falsifiable question, and one the framework is committed to answering experimentally rather than asserting.

### 6.2 Base-38 as a framework-specific hyperparameter
The radix 38 is a choice, not a law. The arithmetic is implemented as a general zero-free sparse polynomial system `BijectiveEncoding<B>` in which determinism, invertibility, and the zero-free canonical form hold for any radix (verified by a radix-sweep for `B ∈ {2, 10, 37, 38, 41}`); `ZeroFree38` is one specialization. The biological instantiation selects 38 as a domain parameter. No physical significance is claimed for the radix itself, and nothing in the framework's formal results depends on it.

**Anticipated critique.** *The choice of a Base-38 radix appears arbitrary or numerological.*
**Defense.** Base-38 is not asserted as a universal physical law but as a deliberate computational hyperparameter chosen to construct a **zero-free sparse state space**. In ordinary Base-10 or continuous real-number representations, zero is an ordinary value — and in biological modeling the zero of a field is the state that conflates with apoptosis, necrosis, or signal nullification. The canonical `ZeroFree38` form never stores a zero digit (digits `1..=37`); zero is the empty (Void) form, and the biological state mapping excludes it by convention. This is a **modeling choice, not a theorem**: it does not make zero mathematically unrepresentable, and it does not eliminate the zero-state singularity from arithmetic. What it does is keep the canonical biological state forms free of stored zero digits and reserve the Void for “no state.” The construction is not an artifact of radix 38: the radix-sweep above (`B ∈ {2, 10, 37, 38, 41}`) shows the same zero-free canonical form at other radices, and the base-100 `PercentEncoding` instantiation shows it at a second scale. Base-38 is simply the chosen hyperparameter that aligns the macroscopic biological deficit with the framework's structural normalization limits. The defense is structural, not numerological: it rests on the zero-free canonical form and the modeling convention, not on the numeric value 38.

### 6.3 Validation status and constant provenance
The framework generates falsifiable specifications — concrete carriers, duty cycles, pulse counts, and amplitudes — but it has not been subjected to in vivo dose-response validation. The profile has four fields, but only two drive the canonical payload: membrane potential (gate train) and fractal dimension (defect proximity, snap amplitude, UŠ₂). Lacunarity is carried but does not enter the canonical payload; burst frequency sets only the 39 Hz ŠÀR carrier, which the in vitro protocol does not test. The external Lean 4 certificate (159 theorem/lemma declarations, 0 sorries) covers the **balanced** base-38 arithmetic family, not the zero-free sparse dialect used here and not the biology; its source is not bundled. See `docs/FORMAL_ARITHMETIC.md`.

**Independently verified (peer-reviewed) inputs.** The membrane-potential anchors are direct measurements. The malignant reference (−7.0 mV) and healthy baseline (−23.0 mV) reproduce, to the reported precision, the resting membrane potentials of EpCAM⁺ cancer stem cells vs normal liver stem cells recorded by microelectrode impalement in human hepatocellular carcinoma and healthy liver tissue (n = 30; −7.0 ± 1.3 mV vs −23.0 ± 1.4 mV, p < 0.01) [12]. The depolarized-toward-zero direction is independently corroborated by in vivo recordings in rodent tumor cells — Morris 7777 hepatoma (−19.8 ± 7.1 mV vs −37.1 ± 4.3 mV in hepatocytes) and mouse fibrosarcoma (−14.3 ± 5.4 mV vs −42.5 ± 5.4 mV in corneal fibroblasts), p = 0.001 for both tissue pairs [13]. The fractal-dimension anchor 1.79 is consistent with the published tumor-interface fractal dimension of glioblastoma multiforme (D₂(2) = 1.79 ± 0.09) [14], and the healthy baseline 1.68 sits within the published range of non-tumorous gray-matter general-structure fractal-dimension estimates (≈1.66–1.69) [15]. These anchors are therefore peer-reviewed patch-clamp/impalement and fractal-boundary measurements, not framework-specific postulates. They come from different tissues — HCC membrane potentials, glioblastoma fractal geometry — and are assembled into a **composite reference profile**; the profile is not a single patient or a single tissue.

**Synthesis-source inputs (heuristic anchors).** The reference resonance 456.383 Hz, and the integer harmonic approximation 456 Hz used by the engine (12 × 38), are framework-specified constants drawn from the framework's synthesis source. Their provenance is the alternative bio-electromagnetic frequency database literature (compiled resonant-frequency lists) rather than mainstream peer-reviewed oncology; the framework retains them as heuristic anchors and does not claim independent clinical validation for them. Their mapping to biological effect is unmeasured. The lacunarity reference (41%, lung squamous-cell carcinoma nuclei) and the NLTL burst peak (39 Hz) are likewise synthesis-source model inputs with no independent measurement attached.

The framework's claim is limited to determinism, reproducibility, and the generation of testable payload specifications — not to efficacy.

### 6.4 Constant-grounding triage
The engine's constants fall into two classes, distinguished deliberately:

- **Grounded in established mathematics, physics, or biology.** The −19 mV gate threshold (the engine's “19/19” gate) lies within the documented depolarization range of transformed cells relative to their normal counterparts (the exact bifurcation threshold is cell-type dependent); the topological attractor limit `D* = 1.839` is the tribonacci constant, the real root of x³ − x² − x − 1 = 0; the 19.47° tetrahedral angle is arcsin(1/3) exactly, and the 19.47 GHz tetrahedral diagnostic carrier is that angle expressed in GHz at full f64 precision.
- **Framework-internal scaling anchors (heuristic baselines, not claimed universal laws).** The 144 Hz reference and 456 Hz seed are framework reference values; 456 Hz is a compiled-database anchor (§6.3). Their computed reference carriers are 155.7 Hz and 480.6 Hz after deficit modulation. These serve as dimensionless scaling anchors for the computational model only. The biological instantiation of the encoding (radix 38) is likewise a hyperparameter (§6.2). Historical or alternative-tuning references that are not used by the canonical payload are excluded from this list.

### 6.5 Robustness and bifurcation structure
A parameter-sensitivity analysis (implemented in the engine's test suite and exposed via the Python bridge) sweeps the resting membrane potential from −30 mV to −5 mV in 0.5 mV steps and the fractal dimension from 1.60 to 1.90. Results: (i) the gate-mediated payload terms are continuous in the membrane potential and reach a minimum exactly at the −19 mV bifurcation gate; (ii) the structural payload (carrier channels, deficit polynomials, entropic-reversion polynomial) is invariant to membrane-potential perturbation, so small input errors do not propagate to the emitted specification; (iii) the gate-side classification flips exactly at the bifurcation threshold; (iv) the fractal mapping saturates below the healthy reference and above the attractor limit, and is monotone non-decreasing between them. The model is thereby a continuous mathematical heuristic with one isolated, physically motivated phase transition — not a brittle numeric script.

### 6.6 Comparative oncology: context, not validation
Comparative oncology is relevant to this framework as context and as a caution, not as evidence for the carrier. Long-lived, large-bodied mammals have evolved diverse resistance strategies: *TP53* retrogene expansion and enhanced p53-dependent apoptosis in elephants [9], a system whose evolutionary interpretation is contested [21]; high-molecular-mass hyaluronan and early contact inhibition in naked mole rats [22]; enhanced DNA double-strand-break repair via CIRBP in bowhead whales [23]; tumour-suppressor duplication, slow proliferation, and damage hypersensitivity in xenarthrans [25]; and interferon-β-mediated concerted necrosis in blind mole rats [26]. Bats are a useful corrective to cell-intrinsic stories: bat fibroblasts show elevated p53-dependent apoptosis and a *TP53* duplication in *M. lucifugus*, yet are readily transformed by two oncogenic hits, implicating immune or niche-level mechanisms [24]. Reviews emphasise that these systems are redundant and multi-layered [27]. None of these mechanisms is an electromagnetic carrier, and none is implemented in the engine. They do not validate RBDC. They support two limited statements: cancer resistance can arise through different tissue-level strategies, so a tissue-scale framework is not categorically implausible; and a single-axis carrier hypothesis is far simpler than evolved resistance and should be presented as a pilot-scale specificity hypothesis. Comparative oncology motivates combination and multi-layer thinking; it does not supply a payload.

---

## References

[1] Saw, T.B., et al. Topological defects in epithelia govern cell death and extrusion. *Nature* 544, 212–216 (2017).

[2] Kirson, E.D., et al. Alternating electric fields arrest cell proliferation in animal tumor models and human brain tumors. *Proc. Natl. Acad. Sci. U.S.A.* 104, 10152–10157 (2007).

[3] Stupp, R., et al. Effect of tumor-treating fields plus maintenance temozolomide vs maintenance temozolomide alone on survival in patients with glioblastoma: a randomized clinical trial. *JAMA* 314, 2535–2543 (2015).

[4] Yang, M. & Brackenbury, W.J. Membrane potential and cancer progression. *Front. Physiol.* 4, 185 (2013).

[5] Levin, M. Bioelectric signaling: reprogramming cells and tissues to control morphogenesis and regeneration. *Annu. Rev. Cell Dev. Biol.* 33, 203–240 (2017).

[6] Woodward, R.M., et al. Terahertz pulse imaging of ex vivo basal cell carcinoma. *J. Invest. Dermatol.* 120, 72–78 (2003).

[7] Yu, C., Fan, S., Sun, Y. & Pickwell-MacPherson, E. The potential of terahertz imaging for cancer diagnosis: a review of investigations to date. *Quant. Imaging Med. Surg.* 2, 33–45 (2012).

[8] Peto, R. Epidemiology, multistage models, and short-term mutagenicity tests. In *Origins of Human Cancer*, Cold Spring Harbor Laboratory Press (1977).

[9] Abegglen, L.M., et al. Potential mechanisms for cancer resistance in elephants and comparative cellular response to DNA damage in humans. *JAMA* 314, 1850–1860 (2015). doi:10.1001/jama.2015.13134.

[10] Venkatesh, B., et al. Elephant shark genome provides unique insights into gnathostome evolution. *Nature* 505, 174–179 (2014).

[11] Marra, N.J., et al. White shark genome reveals ancient eumetazoan genes in elasmobranchs. *BMC Genomics* 20, 697 (2019).

[12] Bautista, W., Lipschitz, J., McKay, A. & Minuk, G.Y. Cancer stem cells are depolarized relative to normal stem cells derived from human livers. *Ann. Hepatol.* 16, 297–303 (2017). doi:10.5604/16652681.1231590.

[13] Binggeli, R. & Cameron, I.L. Cellular potentials of normal and cancerous fibroblasts and hepatocytes. *Cancer Res.* 40, 1830–1835 (1980).

[14] Sánchez, J. & Martín-Landrove, M. Morphological and fractal properties of brain tumors. *Front. Physiol.* 13, 878391 (2022).

[15] Battalapalli, D., Vidyadharan, S., Prabhakar Rao, B.V.V.S.N., Yogeeswari, P., Kesavadas, C. & Rajagopalan, V. Fractal dimension: analyzing its potential as a neuroimaging biomarker for brain tumor diagnosis using machine learning. *Front. Physiol.* 14, 1201617 (2023).

[16] Gottheil, P., Bhattacharyya, S., Lettl, K., Friedrich, P., Roth, K., Rivera-Moreno, S., Merkel, M., Aktas, B., Sauer, I., Daneshgar, A., Wieland, J., Kubitschke, H., Wegscheider, A.-S., Yeomans, J.M. & Käs, J.A. Self organisation of invasive breast cancer driven by the interplay of active and passive nematic dynamics. arXiv:2412.01285 (2024). doi:10.48550/arXiv.2412.01285.

[17] Mathews, J., Erickson, P., Kuchling, F., Jawanda, N., Pio-Lopez, L., Pai, V.P. & Levin, M. Meeting review: “National Cancer Institute Conference on Cancer Bioelectricity” September 12, 2024. *Bioelectricity* 7(1), 94–104 (2025). doi:10.1089/bioe.2024.0049.

[18] Li, K., Zhang, Y., Qian, Y., Qin, H., Zhang, H., Li, C., Peng, C., Zhang, J. & Feng, S. Tumor electrophysiology in precision tumor therapy. *iMetaMed* 1(1) (2025). doi:10.1002/imm3.70002.

[19] Khagi, S., Kotecha, R., Gatson, N.T.N., Jeyapalan, S., Abdullah, H.I., Avgeropoulos, N.G., Batzianouli, E.T., Giladi, M., Lustgarten, L. & Goldlust, S.A. Recent advances in Tumor Treating Fields (TTFields) therapy for glioblastoma. *The Oncologist* 30(2), oyae227 (2025). doi:10.1093/oncolo/oyae227.

[20] Cheung, K.J. & Horne-Badovinac, S. Collective cell migration modes in development, tissue repair and cancer. *Nat. Rev. Mol. Cell Biol.* 26(10), 741–758 (2025). doi:10.1038/s41580-025-00858-9.

[21] Moore, T. Peto's paradox and elephant TP53 retrogenes: cancer or reproduction? *Reproduction* 170(2), e250254 (2025). doi:10.1530/REP-25-0254.

[22] Tian, X., Azpurua, J., Hine, C., Vaidya, A., Myakishev-Rempel, M., Ablaeva, J., Mao, Z., Nevo, E., Gorbunova, V. & Seluanov, A. High-molecular-mass hyaluronan mediates the cancer resistance of the naked mole rat. *Nature* 499, 346–349 (2013). doi:10.1038/nature12234.

[23] Firsanov, D., Zacher, M., Tian, X., et al. Evidence for improved DNA repair in the long-lived bowhead whale. *Nature* 648, 717–725 (2025). doi:10.1038/s41586-025-09694-5.

[24] Athar, F., Zheng, Z., Riquier, S., et al. Limited cell-autonomous anticancer mechanisms in long-lived bats. *Nature Communications* 16, 4125 (2025). doi:10.1038/s41467-025-59403-z.

[25] Vazquez, J.M., Pena, M.T., Muhammad, B., Kraft, M., Adams, L.B. & Lynch, V.J. Parallel evolution of reduced cancer risk and tumor suppressor duplications in Xenarthra. *eLife* 11, e82558 (2022). doi:10.7554/eLife.82558.

[26] Gorbunova, V., Hine, C., Tian, X., Ablaeva, J., Gudkov, A.V., Nevo, E. & Seluanov, A. Cancer resistance in the blind mole rat is mediated by concerted necrotic cell death mechanism. *PNAS* 109(47), 19392–19396 (2012). doi:10.1073/pnas.1217211109.

[27] Seluanov, A., Gladyshev, V.N., Vijg, J. & Gorbunova, V. Mechanisms of cancer resistance in long-lived mammals. *Nature Reviews Cancer* 18(7), 433–441 (2018). PMID 29622806.

[28] Levin, M. Bioelectrical approaches to cancer as a problem of the scaling of the cellular self. *Prog. Biophys. Mol. Biol.* 165, 102–113 (2021). doi:10.1016/j.pbiomolbio.2021.04.007.

[29] Chernet, B.T. & Levin, M. Transmembrane voltage potential is an essential cellular parameter for the detection and control of tumor development in a *Xenopus* model. *Dis. Model. Mech.* 6(3), 595–607 (2013). doi:10.1242/dmm.010835.

---

## License

Text and figures: **CC BY 4.0** — see [`LICENSE-docs`](../LICENSE-docs). Code: **MIT** — see
[`LICENSE`](../LICENSE). This is a theoretical/computational technical report draft; it makes no
clinical efficacy, safety, or device claim.
