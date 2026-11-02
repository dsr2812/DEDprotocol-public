//! DED Protocol (ŠÀR-IM-UŠ₂) resonance engine — theoretical biophysical
//! simulation. Consumes a `TumorProfile` and derives the three correction
//! vectors against the 144 Hz master clock. All output is a computational
//! model of a theoretical system; it asserts no physical or biological reality.

use crate::core::ZeroFree38;

/// 144 Hz reference carrier.
pub const REFERENCE_144_HZ: f64 = 144.0;

/// The 19/19 membrane gate (mV). Cells at this voltage sit on the gate.
///
/// # Equilibrium Singularity
/// At exactly `rmp_mv == GATE_MV` the gate deviation is zero and every
/// gate-mediated correction term annihilates: `calculate_im()` returns `0.0`,
/// so the tetrahedral carrier is emitted unmodulated (`1 + 0`), the UŠ₂ snap
/// gain collapses to unity (pure 456 Hz seed), and the IM payload is the null
/// vector. The gate is therefore the model's healthy phase-lock equilibrium
/// point: a cell resting precisely on the 19/19 membrane is already at perfect
/// resonance, and the engine emits no IM correction. Note that this does NOT
/// null the full payload — `ush2` and `calculate_shar` remain finite and are
/// set by the fractal/lacunarity/NLTL channels, which are independent of the
/// membrane voltage.
pub const GATE_MV: f64 = -19.0;

/// Grounded (healthy) resting membrane potential (mV).
///
/// # Tissue attribution
/// −23.0 mV is the empirically recorded resting potential of EpCAM⁺ normal
/// liver stem cells (−23.0 ± 1.4 mV, microelectrode impalement)
/// [Cancer Stem Cells are Depolarized Relative to Normal Stem Cells Derived
/// from Human Livers, Ann. Hepatol. 16(2):297–303 (2017),
/// doi:10.5604/16652681.1231590]. It sits in the same negative range as the
/// in vivo normal counterparts measured by
/// [Binggeli & Cameron, Cancer Res. 40:1830–1835 (1980)]
/// (hepatocytes −37.1 ± 4.3 mV; corneal fibroblasts −42.5 ± 5.4 mV).
pub const HEALTHY_RMP_MV: f64 = -23.0;

/// Depolarized (malignant) resting membrane potential (mV).
///
/// # Tissue attribution
/// −7.0 mV is the empirically recorded resting potential of EpCAM⁺ cancer
/// stem cells from human hepatocellular carcinoma (−7.0 ± 1.3 mV, n = 30,
/// p < 0.01 vs −23.0 ± 1.4 mV in normal stem cells)
/// [Ann. Hepatol. 16(2):297–303 (2017), doi:10.5604/16652681.1231590].
/// The depolarized-toward-zero direction is independently corroborated by
/// in vivo tumor cells: Morris 7777 hepatoma −19.8 ± 7.1 mV and mouse
/// fibrosarcoma −14.3 ± 5.4 mV, both significantly lower (less negative) than
/// their normal counterparts (p = 0.001)
/// [Binggeli & Cameron, Cancer Res. 40:1830–1835 (1980)].
pub const MALIGNANT_RMP_MV: f64 = -7.0;

/// Full healthy-to-malignant membrane span across the gate (mV).
pub const MEMBRANE_SPAN_MV: f64 = 16.0;

/// Healthy reference fractal dimension.
///
/// # Tissue attribution
/// 1.68 is the framework's healthy reference. It falls within
/// the published range of non-tumorous gray-matter fractal-dimension
/// estimates (general-structure FD ≈ 1.66–1.69)
/// [Fractal dimension: analyzing its potential as a neuroimaging biomarker
/// for glioma grade classification, Front. Physiol. 14:1201617 (2023)].
pub const HEALTHY_FD: f64 = 1.68;

/// Tribonacci Void Limit fractal dimension (geometric dissociation).
///
/// # Tissue attribution
/// The tribonacci constant 1.839 (φ·φ⁻¹ + 1) is a framework-internal anchor;
/// it is not a measured morphometric value. It is deliberately placed beyond
/// the highest published tumor-boundary fractal dimensions so the model's
/// "void limit" is an asymptotic attractor that real tissue never reaches
/// (e.g., GBM interface D₂(2) = 1.79 ± 0.09
/// [Sánchez & Martín-Landrove, Front. Physiol. 13:878391 (2022)]).
pub const TRIBONACCI_VOID_FD: f64 = 1.839;

/// 456 Hz carrier seed (Hz) for the UŠ₂ mapping. The
/// engine uses the integer harmonic 456.0 Hz (12 × 38); the synthesis source
/// specifies 456.383 Hz, approximated here for the Base-38 polynomial mapping.
pub const CARRIER_SEED_456_HZ: f64 = 456.0;

/// Tetrahedral carrier (GHz), the IM modulation target.
///
/// Derived from the tetrahedral vertex eigenmode: the angle `asin(1/3)` swept
/// by a regular tetrahedron's face against its circumsphere is 19.4712206...
/// degrees. The carrier is that continuous eigenmode expressed in GHz at full
/// f64 precision (NOT the base-10 truncated decimal 19.47). Verified against
/// `(1.0/3.0).asin().to_degrees() * 1e9` in `test_tetrahedral_is_eigenmode`.
pub const TETRAHEDRAL_GHZ: f64 = 19.471_220_634_490_692e9;

/// Golden ratio — the continuous root of x² − x − 1 = 0, carried at full f64
/// precision (identical to `(1.0 + 5.0_f64.sqrt()) / 2.0`). Verified in
/// `test_phi_is_golden_ratio`.
pub const PHI: f64 = std::f64::consts::GOLDEN_RATIO;

/// Healthy (neutral) extracellular pH of the tumor microenvironment.
pub const HEALTHY_PH: f64 = 7.4;

/// Malignant (Warburg-acidic) extracellular pH of the tumor microenvironment.
pub const MALIGNANT_PH: f64 = 6.8;

/// Normoxic tissue-oxygen reference (mmHg), at or above which there is no
/// HIF-1α-stabilized immune-evasion drive.
pub const HEALTHY_HYPOXIA_MMHG: f64 = 30.0;

/// Hypoxic reference (mmHg), at or below which HIF-1α stabilization and the
/// immune-evasion drive saturate.
pub const MALIGNANT_HYPOXIA_MMHG: f64 = 10.0;

/// Biophysical snapshot of a single cell: the canonical heuristic
/// reference values that anchor the RBDC computational model. These encode the
/// framework's theoretical baselines for a depolarized, geometrically
/// dissociated malignant cell. They are canonical anchors for the simulation,
/// not absolute diagnostic thresholds.
///
/// # Provenance of anchor values
/// The membrane-potential anchors are **measured** values: the −7.0 / −23.0 mV
/// pair reproduces EpCAM⁺ cancer-vs-normal liver stem cells
/// [Ann. Hepatol. 16(2):297–303 (2017)]; the −14.3 mV fibrosarcoma and
/// −19.8 mV hepatoma values corroborate the direction
/// [Binggeli & Cameron, Cancer Res. 40:1830–1835 (1980)]. The fractal
/// dimension 1.79 matches published GBM tumor-interface estimates
/// [Sánchez & Martín-Landrove, Front. Physiol. 13:878391 (2022)]; 1.68 sits
/// within the published non-tumorous gray-matter FD range
/// [Front. Physiol. 14:1201617 (2023)]. The lacunarity (41%) and NLTL burst
/// (39 Hz) values are **synthesis-source** references — they are model inputs
/// taken from the framework's own source material, not from independent
/// measurements, and must be treated as such.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TumorProfile {
    /// Resting membrane potential (mV). Depolarized (malignant) reference =
    /// −7.0; grounded (healthy) baseline = −23.0. Both anchors are measured
    /// values from EpCAM⁺ cancer stem cells of human hepatocellular carcinoma
    /// vs normal liver stem cells
    /// [Ann. Hepatol. 16(2):297–303 (2017), doi:10.5604/16652681.1231590],
    /// consistent with the documented depolarization of tumor cells relative
    /// to their normal counterparts [Binggeli & Cameron, Cancer Res.
    /// 40:1830–1835 (1980)] and the conceptual review of Vₘ in proliferation
    /// [Yang & Brackenbury, Front. Physiol. 4:185 (2013)].
    pub rmp_mv: f64,
    /// Nuclear boundary fractal dimension. Malignant reference = 1.79; healthy
    /// (φ-lattice) baseline = 1.68; maximum entropy attractor limit = 1.839.
    /// The 1.79 value is consistent with the published GBM tumor-interface
    /// fractal dimension D₂(2) = 1.79 ± 0.09
    /// [Sánchez & Martín-Landrove, Front. Physiol. 13:878391 (2022)]; the 1.68
    /// baseline sits within the published non-tumorous gray-matter FD range
    /// [Front. Physiol. 14:1201617 (2023)]. Fractal geometry is an established
    /// morphometric descriptor of tumor boundaries
    /// [Cross, J. Pathol. 182:1–8 (1997)].
    pub fractal_dim: f64,
    /// Spatial heterogeneity (%), i.e., the void/lacunarity structure of the
    /// tissue matrix. Malignant reference ≈ 41%, taken from the framework's
    /// synthesis source for lung squamous-cell carcinoma nuclei; the model
    /// asserts no independent measurement of this value. Elevated lacunarity
    /// tracking fractal dimension in malignant tissue is documented
    /// [Sánchez & Martín-Landrove, Front. Physiol. 13:878391 (2022)].
    pub lacunarity_pct: f64,
    /// Spectral peak (Hz) of action potential-like bursts in the model's
    /// coupled nonlinear transmission line (NLTL) excitable-system reference.
    /// Malignant peak = 39.0. This is a framework-internal model reference
    /// (synthesis-source); it is not presented as an independently measured
    /// physiological quantity.
    pub nltl_burst_hz: f64,
}

impl TumorProfile {
    /// Grounded, phase-locked reference cell.
    pub fn healthy() -> Self {
        Self {
            rmp_mv: HEALTHY_RMP_MV,
            fractal_dim: HEALTHY_FD,
            lacunarity_pct: 0.0,
            nltl_burst_hz: 0.0,
        }
    }

    /// Depolarized malignant reference cell: the canonical profile for a fully
    /// depolarized (−7.0 mV), geometrically dissociated (FD 1.79) malignant
    /// tissue matrix with high lacunarity (41%) and a 39 Hz NLTL burst peak.
    ///
    /// 456 Hz carrier seed: the engine uses the integer
    /// harmonic 456.0 Hz (12 × 38); the synthesis source specifies 456.383 Hz,
    /// approximated here for the Base-38 polynomial mapping.
    pub fn malignant() -> Self {
        Self {
            rmp_mv: MALIGNANT_RMP_MV,
            fractal_dim: 1.79,
            lacunarity_pct: 41.0,
            nltl_burst_hz: 39.0,
        }
    }
}

/// Tumor-microenvironment snapshot: the Warburg metabolic state of the tissue
/// matrix, characterized by the acidic cloak (extracellular pH) and the
/// hypoxic drive (interstitial oxygen partial pressure, mmHg).
///
/// Two channels are derived:
/// * `calculate_dielectric_attenuation` — the acidic cloak: the pH drop
///   toward the 6.8 reference raises the local dielectric permittivity of the
///   interstitial matrix, attenuating externally applied fields. The engine
///   responds by raising the amplitude of the 144 Hz grounding carrier.
/// * `calculate_immune_evasion_deficit` — the hypoxic drive: at or below the
///   10 mmHg reference, HIF-1α is stabilized and PD-L1 immune evasion is
///   upregulated. The deficit modulates the ŠÀR (Phase-Extraction) channel to
///   target the stabilized hypoxic burst.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TmeProfile {
    /// Extracellular pH of the tumor microenvironment (healthy 7.4, malignant 6.8).
    pub ph_level: f64,
    /// Interstitial oxygen partial pressure (mmHg; healthy >30, malignant <10).
    pub hypoxia_mmhg: f64,
}

impl TmeProfile {
    /// Normoxic, neutral-pH reference microenvironment.
    pub fn healthy() -> Self {
        Self {
            ph_level: HEALTHY_PH,
            hypoxia_mmhg: HEALTHY_HYPOXIA_MMHG,
        }
    }

    /// Warburg-acidic, hypoxic malignant reference microenvironment.
    pub fn malignant() -> Self {
        Self {
            ph_level: MALIGNANT_PH,
            hypoxia_mmhg: MALIGNANT_HYPOXIA_MMHG,
        }
    }

    /// Acidic-cloak dielectric shielding factor in `0.0..=1.0`.
    ///
    /// Maps the pH drop from the 7.4 norm toward the 6.8 reference: `0.0` at
    /// the norm (no cloak), `1.0` at or below the reference (full shielding).
    #[inline]
    pub fn calculate_dielectric_attenuation(&self) -> f64 {
        let span = HEALTHY_PH - MALIGNANT_PH;
        ((HEALTHY_PH - self.ph_level) / span).clamp(0.0, 1.0)
    }

    /// Hypoxic immune-evasion drive in `0.0..=1.0`.
    ///
    /// Maps the oxygen-pressure drop from the 30 mmHg norm toward the 10 mmHg
    /// HIF-1α-stabilization threshold: `0.0` at the norm (no evasion),
    /// `1.0` at or below the reference (saturated evasion).
    #[inline]
    pub fn calculate_immune_evasion_deficit(&self) -> f64 {
        let span = HEALTHY_HYPOXIA_MMHG - MALIGNANT_HYPOXIA_MMHG;
        ((HEALTHY_HYPOXIA_MMHG - self.hypoxia_mmhg) / span).clamp(0.0, 1.0)
    }

    /// The immune-evasion drive quantized to basis points and encoded as a
    /// zero-free Base-38 polynomial — the HIF-1α evasion deficit vector.
    pub fn immune_evasion_polynomial(&self) -> ZeroFree38 {
        let basis_points = (self.calculate_immune_evasion_deficit() * 10_000.0).round() as u128;
        ZeroFree38::from_u128(basis_points)
    }
}

/// Which side of the 19/19 membrane the cell's voltage sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateSide {
    /// Voltage below the gate (grounded side, healthy).
    Below,
    /// Voltage above the gate (depolarized side, malignant).
    Above,
}

/// Geometric/topological state derived from a `TumorProfile`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopologyCalculator {
    profile: TumorProfile,
}

impl TopologyCalculator {
    pub fn new(profile: TumorProfile) -> Self {
        Self { profile }
    }

    pub fn profile(&self) -> &TumorProfile {
        &self.profile
    }

    /// Signed voltage distance from the 19/19 gate (mV).
    /// Negative = grounded (below the gate); positive = depolarized (above).
    ///
    /// # Equilibrium Singularity
    /// At `0.0` (i.e., `rmp_mv == GATE_MV == -19.0 mV`) the cell sits exactly
    /// on the gate. Because `calculate_im()` is normalized to `|gate_deviation|`,
    /// this point is a genuine null of the gate-mediated IM channel — the
    /// correction payload vanishes and the engine emits no IM modulation. It is
    /// the healthy phase-lock equilibrium of the membrane channel only; the
    /// fractal and NLTL channels (and therefore `ush2` and `shar`) are
    /// decoupled from voltage and remain finite here.
    #[inline]
    pub fn gate_deviation(&self) -> f64 {
        self.profile.rmp_mv - GATE_MV
    }

    /// Side of the 19/19 membrane the voltage occupies.
    #[inline]
    pub fn gate_side(&self) -> GateSide {
        if self.gate_deviation() > 0.0 {
            GateSide::Above
        } else {
            GateSide::Below
        }
    }

    /// Fractional geometric stretch from the healthy reference (1.68) toward the
    /// Tribonacci Void Limit (1.839), clamped to `0.0..=1.0`.
    /// 0.0 = fully phase-locked; 1.0 = at the Void Limit.
    #[inline]
    pub fn void_proximity(&self) -> f64 {
        let span = TRIBONACCI_VOID_FD - HEALTHY_FD;
        ((self.profile.fractal_dim - HEALTHY_FD) / span).clamp(0.0, 1.0)
    }

    /// Complement of `void_proximity`: remaining lattice lock, `0.0..=1.0`.
    #[inline]
    pub fn lattice_lock(&self) -> f64 {
        1.0 - self.void_proximity()
    }
}

/// The ŠÀR-IM-UŠ₂ resonance engine.
#[derive(Debug, Clone)]
pub struct DedResonanceEngine {
    profile: TumorProfile,
    master_clock_hz: f64,
    topology: TopologyCalculator,
}

impl DedResonanceEngine {
    pub fn new(profile: TumorProfile, master_clock_hz: f64) -> Self {
        let topology = TopologyCalculator::new(profile);
        Self {
            profile,
            master_clock_hz,
            topology,
        }
    }

    /// Engine bound to the canonical 144 Hz master clock.
    pub fn with_master_clock(profile: TumorProfile) -> Self {
        Self::new(profile, REFERENCE_144_HZ)
    }

    pub fn profile(&self) -> &TumorProfile {
        &self.profile
    }

    pub fn master_clock_hz(&self) -> f64 {
        self.master_clock_hz
    }

    pub fn topology(&self) -> &TopologyCalculator {
        &self.topology
    }

    /// ŠÀR — destructive-interference frequency. A sinusoid is fully nulled by
    /// an equal-amplitude antiphase (π-shifted) component at the same
    /// frequency, so the required emission equals the burst rate itself.
    #[inline]
    pub fn calculate_shar(&self) -> f64 {
        self.profile.nltl_burst_hz
    }

    /// NLTL envelope node: half the burst rate, where the shock-front envelope
    /// collapses to a standing null.
    #[inline]
    pub fn shar_envelope_hz(&self) -> f64 {
        self.profile.nltl_burst_hz / 2.0
    }

    /// IM — modulation index applied to the 19.47 GHz tetrahedral carrier,
    /// derived from the magnitude of the gate deviation normalized to the full
    /// healthy-to-malignant membrane span. Clamped to `0.0..=1.0`.
    ///
    /// # Equilibrium Singularity
    /// At the gate (`rmp_mv == -19.0 mV`) the deviation is zero, so `im == 0.0`
    /// exactly and the carrier is emitted unmodulated: the IM payload is the
    /// null vector. This is the model's phase-lock equilibrium — a cell resting
    /// on the 19/19 membrane needs no IM correction. Moving 1 mV off the gate
    /// on either side produces an identical `im` (the term is symmetric in
    /// `|deviation|`); `gate_side()` distinguishes which side is occupied.
    #[inline]
    pub fn calculate_im(&self) -> f64 {
        (self.topology.gate_deviation().abs() / MEMBRANE_SPAN_MV).clamp(0.0, 1.0)
    }

    /// Tetrahedral carrier after IM amplitude modulation.
    #[inline]
    pub fn modulated_carrier_hz(&self) -> f64 {
        TETRAHEDRAL_GHZ * (1.0 + self.calculate_im())
    }

    /// UŠ₂ — the Base-38 harmonic polynomial required to mechanically snap the
    /// nuclear geometry back to the healthy reference.
    ///
    /// The 456 Hz carrier seed initializes the polynomial. Its base-38
    /// expansion is `12·38¹` — the units place is a perfect multiple of 38 and
    /// collapses to an implied zero. A snap gain of `1 + void_proximity` scales
    /// the seed with geometric stretch: at full phase-lock the gain is unity
    /// (pure 456 Hz); at the Void Limit it doubles to 912 Hz (`24·38¹`, again
    /// with an implied-zero units place).
    pub fn calculate_ush2(&self) -> ZeroFree38 {
        let gain = 1.0 + self.topology.void_proximity();
        let snapped_hz = (CARRIER_SEED_456_HZ * gain).round() as u128;
        ZeroFree38::from_u128(snapped_hz)
    }

    /// Integer evaluation of `calculate_ush2`.
    #[inline]
    pub fn ush2_value(&self) -> u128 {
        self.calculate_ush2().to_u128().unwrap_or(0)
    }

    /// Compute all three correction vectors in one pass.
    pub fn audit(&self) -> ResonanceAudit {
        let shar = self.calculate_shar();
        let im = self.calculate_im();
        let ush2 = self.calculate_ush2();
        ResonanceAudit {
            shar_hz: shar,
            shar_envelope_hz: self.shar_envelope_hz(),
            im,
            modulated_carrier_hz: self.modulated_carrier_hz(),
            ush2: ush2.clone(),
            ush2_value: ush2.to_u128().unwrap_or(0),
            gate_deviation_mv: self.topology.gate_deviation(),
            gate_side: self.topology.gate_side(),
            void_proximity: self.topology.void_proximity(),
            lattice_lock: self.topology.lattice_lock(),
            rmp_mv: self.profile.rmp_mv,
            fractal_dim: self.profile.fractal_dim,
            lacunarity_pct: self.profile.lacunarity_pct,
            nltl_burst_hz: self.profile.nltl_burst_hz,
            master_clock_hz: self.master_clock_hz,
        }
    }
}

/// Complete ŠÀR-IM-UŠ₂ correction vector set.
#[derive(Debug, Clone)]
pub struct ResonanceAudit {
    pub rmp_mv: f64,
    pub fractal_dim: f64,
    pub lacunarity_pct: f64,
    pub nltl_burst_hz: f64,
    pub master_clock_hz: f64,
    pub gate_deviation_mv: f64,
    pub gate_side: GateSide,
    pub void_proximity: f64,
    pub lattice_lock: f64,
    pub shar_hz: f64,
    pub shar_envelope_hz: f64,
    pub im: f64,
    pub modulated_carrier_hz: f64,
    pub ush2: ZeroFree38,
    pub ush2_value: u128,
}

impl std::fmt::Display for ResonanceAudit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.gate_side {
            GateSide::Above => "ABOVE",
            GateSide::Below => "BELOW",
        };
        writeln!(f, "+==========================================+")?;
        writeln!(f, "|  DED RESONANCE AUDIT — SAR-IM-US2       |")?;
        writeln!(f, "+==========================================+")?;
        writeln!(
            f,
            "  Profile:  RMP {:+.1} mV | FD {:.3} | Lacunarity {:.1}% | Burst {:.1} Hz",
            self.rmp_mv, self.fractal_dim, self.lacunarity_pct, self.nltl_burst_hz
        )?;
        writeln!(f, "  Master Clock: {:.2} Hz", self.master_clock_hz)?;
        writeln!(f)?;
        writeln!(f, "  Topology")?;
        writeln!(f, "    Gate (19/19):     {:+.1} mV", GATE_MV)?;
        writeln!(
            f,
            "    Gate deviation:   {:+.2} mV ({side})",
            self.gate_deviation_mv
        )?;
        writeln!(
            f,
            "    Void proximity:   {:.3} ({:.1}%)",
            self.void_proximity,
            self.void_proximity * 100.0
        )?;
        writeln!(f, "    Lattice lock:     {:.3}", self.lattice_lock)?;
        writeln!(f)?;
        writeln!(f, "  Correction Vectors")?;
        writeln!(
            f,
            "    SAR  (destructive): {:>8.2} Hz @ antiphase \u{3c0}   [envelope null: {:.2} Hz]",
            self.shar_hz, self.shar_envelope_hz
        )?;
        writeln!(
            f,
            "    IM   (modulation):  {:>8.4} -> carrier {:.2} GHz -> {:.2} GHz",
            self.im,
            TETRAHEDRAL_GHZ / 1e9,
            self.modulated_carrier_hz / 1e9
        )?;
        writeln!(
            f,
            "    US2  (harmonic):    {} -> {} (= {})",
            CARRIER_SEED_456_HZ, self.ush2, self.ush2_value
        )?;
        Ok(())
    }
}

/// Instantiate a malignant reference profile, run the engine, and print the
/// ŠÀR-IM-UŠ₂ correction vectors to the terminal.
pub fn run_audit() -> ResonanceAudit {
    let engine = DedResonanceEngine::with_master_clock(TumorProfile::malignant());
    let audit = engine.audit();
    println!("{audit}");
    audit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Digit;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn term(place: u32, digit: u8) -> crate::core::Term {
        crate::core::Term {
            place,
            digit: unsafe { Digit::new_unchecked(digit) },
        }
    }

    /// Geometric purity: the tetrahedral carrier must equal the continuous
    /// eigenmode asin(1/3) expressed in degrees, scaled to GHz — NOT the
    /// base-10 truncated decimal 19.47.
    #[test]
    fn test_tetrahedral_is_eigenmode() {
        let eigenmode_ghz = (1.0_f64 / 3.0_f64).asin().to_degrees() * 1e9;
        let rel_err = (TETRAHEDRAL_GHZ - eigenmode_ghz).abs() / eigenmode_ghz;
        assert!(
            rel_err < 1e-12,
            "TETRAHEDRAL_GHZ deviates from asin(1/3) eigenmode: rel {rel_err:e}"
        );
        assert_ne!(
            TETRAHEDRAL_GHZ, 19.47e9,
            "must not be the truncated decimal"
        );
    }

    /// Geometric purity: PHI must be the golden ratio (1+sqrt(5))/2 at full
    /// f64 precision — not a base-10 truncation.
    #[test]
    fn test_phi_is_golden_ratio() {
        let golden = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert_eq!(
            PHI, golden,
            "PHI must equal (1+sqrt(5))/2 at full f64 precision"
        );
        assert_ne!(PHI, 1.618, "PHI must not be a base-10 truncation");
    }

    #[test]
    fn topology_gate_deviation() {
        let healthy = TopologyCalculator::new(TumorProfile::healthy());
        let malignant = TopologyCalculator::new(TumorProfile::malignant());
        assert!(approx(healthy.gate_deviation(), -4.0));
        assert!(approx(malignant.gate_deviation(), 12.0));
        assert_eq!(healthy.gate_side(), GateSide::Below);
        assert_eq!(malignant.gate_side(), GateSide::Above);
    }

    #[test]
    fn topology_void_proximity() {
        let healthy = TopologyCalculator::new(TumorProfile::healthy());
        let malignant = TopologyCalculator::new(TumorProfile::malignant());
        let span = TRIBONACCI_VOID_FD - HEALTHY_FD;
        assert!(approx(healthy.void_proximity(), 0.0));
        assert!(approx(healthy.lattice_lock(), 1.0));
        assert!(approx(malignant.void_proximity(), 0.11 / span));
        assert!(approx(malignant.lattice_lock(), 1.0 - 0.11 / span));

        let clamped = TopologyCalculator::new(TumorProfile {
            rmp_mv: -7.0,
            fractal_dim: 2.5,
            lacunarity_pct: 41.0,
            nltl_burst_hz: 39.0,
        });
        assert!(approx(clamped.void_proximity(), 1.0));
    }

    #[test]
    fn shar_neutralizes_burst() {
        let engine = DedResonanceEngine::with_master_clock(TumorProfile::malignant());
        assert!(approx(engine.calculate_shar(), 39.0));
        assert!(approx(engine.shar_envelope_hz(), 19.5));
    }

    #[test]
    fn im_from_gate_deviation() {
        let healthy = DedResonanceEngine::with_master_clock(TumorProfile::healthy());
        let malignant = DedResonanceEngine::with_master_clock(TumorProfile::malignant());
        assert!(approx(healthy.calculate_im(), 0.25));
        assert!(approx(malignant.calculate_im(), 0.75));
        assert!(approx(
            malignant.modulated_carrier_hz(),
            TETRAHEDRAL_GHZ * 1.75
        ));
    }

    #[test]
    fn ush2_harmonic_polynomial() {
        let healthy = DedResonanceEngine::with_master_clock(TumorProfile::healthy());
        let malignant = DedResonanceEngine::with_master_clock(TumorProfile::malignant());

        let ush2 = healthy.calculate_ush2();
        assert_eq!(ush2.to_string(), "12·38");
        assert_eq!(ush2.terms(), &[term(1, 12)]);
        assert_eq!(healthy.ush2_value(), 456);

        let ush2 = malignant.calculate_ush2();
        assert_eq!(ush2.terms(), &[term(0, 11), term(1, 20)]);
        assert_eq!(malignant.ush2_value(), 771);
    }

    #[test]
    fn ush2_at_void_limit() {
        let void = DedResonanceEngine::with_master_clock(TumorProfile {
            rmp_mv: -7.0,
            fractal_dim: TRIBONACCI_VOID_FD,
            lacunarity_pct: 41.0,
            nltl_burst_hz: 39.0,
        });
        let ush2 = void.calculate_ush2();
        assert_eq!(ush2.to_string(), "24·38");
        assert_eq!(void.ush2_value(), 912);
    }

    #[test]
    fn audit_round_trip() {
        let engine = DedResonanceEngine::with_master_clock(TumorProfile::malignant());
        let audit = engine.audit();
        assert_eq!(audit.gate_side, GateSide::Above);
        assert!(approx(audit.im, 0.75));
        assert_eq!(audit.ush2_value, 771);
    }

    #[test]
    fn equilibrium_singularity_at_gate() {
        let on_gate = DedResonanceEngine::with_master_clock(TumorProfile {
            rmp_mv: GATE_MV,
            fractal_dim: HEALTHY_FD,
            lacunarity_pct: 0.0,
            nltl_burst_hz: 0.0,
        });
        let audit = on_gate.audit();
        assert!(approx(audit.gate_deviation_mv, 0.0));
        assert_eq!(audit.gate_side, GateSide::Below);
        assert!(approx(audit.im, 0.0));
        assert!(approx(audit.modulated_carrier_hz, TETRAHEDRAL_GHZ));
        assert_eq!(audit.ush2_value, 456);
        assert!(approx(audit.void_proximity, 0.0));
    }

    #[test]
    fn im_symmetric_about_gate() {
        let below = DedResonanceEngine::with_master_clock(TumorProfile {
            rmp_mv: GATE_MV - 4.0,
            fractal_dim: HEALTHY_FD,
            lacunarity_pct: 0.0,
            nltl_burst_hz: 0.0,
        });
        let above = DedResonanceEngine::with_master_clock(TumorProfile {
            rmp_mv: GATE_MV + 4.0,
            fractal_dim: HEALTHY_FD,
            lacunarity_pct: 0.0,
            nltl_burst_hz: 0.0,
        });
        assert!(approx(below.calculate_im(), above.calculate_im()));
        assert_eq!(below.topology().gate_side(), GateSide::Below);
        assert_eq!(above.topology().gate_side(), GateSide::Above);
    }

    #[test]
    fn healthy_tme_has_no_cloak_and_no_evasion() {
        let tme = TmeProfile::healthy();
        assert_eq!(tme.calculate_dielectric_attenuation(), 0.0);
        assert_eq!(tme.calculate_immune_evasion_deficit(), 0.0);
        assert!(tme.immune_evasion_polynomial().is_zero());
        assert_eq!(tme.immune_evasion_polynomial().to_string(), "∅");
    }

    #[test]
    fn malignant_tme_saturates_shielding_and_evasion() {
        let tme = TmeProfile::malignant();
        assert!(approx(tme.calculate_dielectric_attenuation(), 1.0));
        assert!(approx(tme.calculate_immune_evasion_deficit(), 1.0));
        assert_eq!(tme.immune_evasion_polynomial().to_u128(), Some(10_000));
        assert_eq!(
            tme.immune_evasion_polynomial().to_string(),
            "6 + 35·38 + 6·38^2"
        );
    }

    #[test]
    fn dielectric_attenuation_scales_with_ph_drop() {
        let norm = TmeProfile {
            ph_level: 7.4,
            hypoxia_mmhg: 30.0,
        };
        assert_eq!(norm.calculate_dielectric_attenuation(), 0.0);
        let mid = TmeProfile {
            ph_level: 7.1,
            hypoxia_mmhg: 30.0,
        };
        assert!(approx(mid.calculate_dielectric_attenuation(), 0.5));
        let acidic = TmeProfile {
            ph_level: 6.5,
            hypoxia_mmhg: 30.0,
        };
        assert_eq!(acidic.calculate_dielectric_attenuation(), 1.0);
    }

    #[test]
    fn immune_evasion_scales_with_hypoxia() {
        let normoxic = TmeProfile {
            ph_level: 7.4,
            hypoxia_mmhg: 40.0,
        };
        assert_eq!(normoxic.calculate_immune_evasion_deficit(), 0.0);
        let mid = TmeProfile {
            ph_level: 7.4,
            hypoxia_mmhg: 20.0,
        };
        assert!(approx(mid.calculate_immune_evasion_deficit(), 0.5));
        let anoxic = TmeProfile {
            ph_level: 7.4,
            hypoxia_mmhg: 2.0,
        };
        assert_eq!(anoxic.calculate_immune_evasion_deficit(), 1.0);
    }
}
