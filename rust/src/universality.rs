//! Scale-invariant extension of the Resonant Biophysical Debt Correction
//! (RBDC) computational heuristic.
//!
//! Exercises the non-null bijective sparse polynomial encoding ([`ZeroFree38`])
//! against three physical systems characterized by scale-invariant order:
//!
//! 1. [`EpithelialTessellation`] — two-dimensional Voronoi/honeycomb packing.
//! 2. [`ScaleInvarianceBreakdown`] — multi-scale (local/global) metric coupling.
//! 3. [`ChromatinContactMatrix`] — one-dimensional polymer folding topology.
//!
//! A fourth component, [`StateSpaceComparison`], formalizes the
//! information-theoretic contrast between the discrete bijective state space
//! (base-38) and standard continuous (base-10) representations.
//!
//! Percent-valued features (packing deficits, discrepancy basis points) are
//! additionally encoded through the second instantiation of the same
//! substrate, [`PercentEncoding`] (base-100), whose digits read directly as
//! percent digits. [`ScaleInvarianceBreakdown::desync_factor`] exports the
//! broken-scale coupling that the Apex synthesizer feeds into the 456 Hz UŠ₂
//! carrier.
//!
//! This module demonstrates the *formal* properties of the encoding —
//! determinism, reversibility, and non-nullness — on model systems. It is a
//! computational heuristic; it asserts no physical or biological reality.

use crate::core::{PercentEncoding, ZeroFree38, BASE};
use std::f64::consts::PI;

/// Constant `2·√3`, the denominator of the honeycomb packing fraction.
pub const TWO_ROOT_THREE: f64 = 3.464_101_615_137_755;

/// Ideal hexagonal (Voronoi) packing efficiency: the area fraction of the
/// plane covered by a honeycomb lattice, `π/(2·√3) ≈ 0.9069`.
pub const HEXAGONAL_PACKING_EFFICIENCY: f64 = PI / TWO_ROOT_THREE;

/// Quantization resolution for packing deficits, in basis points (0.01%).
pub const PACKING_BASIS_POINTS: u128 = 10_000;

/// Quantization resolution for metric expansion discrepancies, in basis points.
pub const DISCREPANCY_BASIS_POINTS: u128 = 10_000;

/// Quantization resolution for the polymer compaction signature (×10⁻³).
pub const COMPACTION_SCALE: u128 = 1_000;

/// Percent-scale bijective radix: 100. A feature quantized to integer basis
/// points (1/100 of a percent) encodes with place 0 holding the fractional
/// percent and place 1 the whole percent: 1269 bp → `69 + 12·100` (12.69%),
/// 8621 bp → `21 + 86·100` (86.21%). The base-100 form is a second
/// instantiation of the generic zero-free encoding ([`PercentEncoding`]); it
/// carries the same determinism, reversibility, and non-nullness guarantees
/// as the canonical base-38 form.
pub const PERCENT_SCALE_RADIX: u32 = 100;

/// Ceil-log of `n + 1` in `base`: the fixed-width symbol count required to
/// represent `n` in a positional system of radix `base` (zero permitted).
fn ceil_log(base: f64, n: u128) -> u32 {
    let n_plus_one = n as f64 + 1.0;
    if n_plus_one <= 1.0 {
        1
    } else {
        (n_plus_one.log(base).ceil().max(1.0)) as u32
    }
}

/// Count of `'0'` characters in the decimal (base-10) representation of `n`.
/// These are zero-placeholder symbols: padding digits that carry no
/// information but occupy state space.
fn decimal_zero_placeholders(n: u128) -> u32 {
    n.to_string().chars().filter(|c| *c == '0').count() as u32
}

/// Packing-efficiency deficit of a tissue matrix relative to the ideal
/// hexagonal (Voronoi) tessellation of the epithelial plane.
///
/// Healthy epithelia approximate a honeycomb lattice. Malignant transformation
/// degrades coordination order and opens the packing geometry, lowering the
/// measured efficiency below the hexagonal ideal. The deficit is quantized to
/// basis points and mapped into the zero-free base-38 polynomial: the healthy
/// state (zero deficit) encodes to the empty polynomial `∅`, while any ordered
/// departure from the ideal lattice produces a nonzero polynomial — a
/// reversible displacement, never a collapse to zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EpithelialTessellation {
    measured_efficiency: f64,
}

impl EpithelialTessellation {
    /// Construct from a measured packing efficiency, clamped to `0.0..=1.0`.
    pub fn new(measured_efficiency: f64) -> Self {
        Self {
            measured_efficiency: measured_efficiency.clamp(0.0, 1.0),
        }
    }

    /// The measured packing efficiency.
    pub fn measured_efficiency(&self) -> f64 {
        self.measured_efficiency
    }

    /// The hexagonal (Voronoi) ideal packing efficiency.
    #[inline]
    pub fn hexagonal_ideal() -> f64 {
        HEXAGONAL_PACKING_EFFICIENCY
    }

    /// Packing efficiency deficit: the ideal minus the measured efficiency,
    /// clamped at zero (a tissue cannot exceed the hexagonal ideal).
    pub fn packing_deficit(&self) -> f64 {
        (HEXAGONAL_PACKING_EFFICIENCY - self.measured_efficiency).max(0.0)
    }

    /// The deficit quantized to integer basis points.
    pub fn deficit_basis_points(&self) -> u128 {
        (self.packing_deficit() * PACKING_BASIS_POINTS as f64).round() as u128
    }

    /// The deficit mapped into the zero-free base-38 polynomial. Zero deficit
    /// maps to the empty polynomial `∅`; any packing-order departure produces
    /// a nonzero, reversible polynomial state.
    pub fn deficit_polynomial(&self) -> ZeroFree38 {
        ZeroFree38::from_u128(self.deficit_basis_points())
    }

    /// The deficit mapped into the zero-free base-100 polynomial — the
    /// percent-scaled form. Identical integer semantics to
    /// [`Self::deficit_polynomial`] (`deficit_basis_points()`), but read
    /// directly in percent digits: for a 12.69% deficit the place-1 digit is
    /// `12` (whole percent) and the place-0 digit is `69` (fractional
    /// percent).
    pub fn deficit_polynomial_percent(&self) -> PercentEncoding {
        PercentEncoding::from_u128(self.deficit_basis_points())
    }
}

/// Metric expansion discrepancy between the local and global scale factors of
/// a growing tissue.
///
/// This is the framework's analog of the Hubble tension: two independent
/// determinations of an expansion rate that disagree. The local scale factor
/// (cell-cycle proliferation rate) is measured against the global scale factor
/// (angiogenic vascularization of the tissue). A vanishing discrepancy
/// indicates that scale invariance holds between the micro (cell) and macro
/// (tissue) regimes; a nonzero discrepancy is an ordered deviation — a broken
/// scale-invariance — mapped to a compensatory polynomial that quantifies the
/// re-synchronization required to re-couple the two scales.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleInvarianceBreakdown {
    local_scale_factor: f64,
    global_scale_factor: f64,
}

impl ScaleInvarianceBreakdown {
    /// Construct from the local (proliferation) and global (vascularization)
    /// scale factors. Both are clamped away from zero to keep the discrepancy
    /// well-defined.
    pub fn new(local_scale_factor: f64, global_scale_factor: f64) -> Self {
        Self {
            local_scale_factor: local_scale_factor.max(f64::EPSILON),
            global_scale_factor: global_scale_factor.max(f64::EPSILON),
        }
    }

    pub fn local_scale_factor(&self) -> f64 {
        self.local_scale_factor
    }

    pub fn global_scale_factor(&self) -> f64 {
        self.global_scale_factor
    }

    /// Relative discrepancy between the two expansion determinations,
    /// `|local − global| / global`, dimensionless.
    pub fn metric_expansion_discrepancy(&self) -> f64 {
        (self.local_scale_factor - self.global_scale_factor).abs() / self.global_scale_factor
    }

    /// Factor by which the local scale must contract (or dilate) to match the
    /// global scale: `global / local`. Unity indicates perfect synchrony.
    pub fn resynchronization_factor(&self) -> f64 {
        self.global_scale_factor / self.local_scale_factor
    }

    /// The discrepancy quantized to integer basis points.
    pub fn discrepancy_basis_points(&self) -> u128 {
        (self.metric_expansion_discrepancy() * DISCREPANCY_BASIS_POINTS as f64).round() as u128
    }

    /// The compensatory zero-free base-38 vector for re-synchronization. A
    /// synchronous state (discrepancy zero) maps to `∅`; any broken
    /// scale-invariance maps to a nonzero polynomial.
    pub fn compensatory_vector(&self) -> ZeroFree38 {
        ZeroFree38::from_u128(self.discrepancy_basis_points())
    }

    /// The compensatory vector in the zero-free base-100 (percent-scaled)
    /// form. Identical integer semantics to [`Self::compensatory_vector`]
    /// (`discrepancy_basis_points()`); the base-100 digits read directly as
    /// percent digits (place 1 = whole percent, place 0 = fractional percent).
    pub fn compensatory_vector_percent(&self) -> PercentEncoding {
        PercentEncoding::from_u128(self.discrepancy_basis_points())
    }

    /// Desynchronization coupling fed back into the resonance engine: the
    /// metric expansion discrepancy clamped to `0.0..=1.0`.
    ///
    /// This is the universality→engine coupling term. A broken scale
    /// invariance (nonzero discrepancy) yields a nonzero `desync_factor`, which
    /// the Apex synthesizer uses to modulate the 456 Hz UŠ₂ carrier amplitude;
    /// a perfectly synchronized state (discrepancy zero) yields `0.0` and
    /// leaves the carrier untouched. Unity saturates the modulation at full
    /// desynchronization.
    pub fn desync_factor(&self) -> f64 {
        self.metric_expansion_discrepancy().min(1.0)
    }
}

/// A chromatin contact matrix (Hi-C style): `contacts[i][j]` is the normalized
/// contact frequency between loci `i` and `j`.
///
/// The folding topology is reduced to a single scalar invariant — the
/// contact-order-weighted span `Σ_{i<j} c_ij · (j − i)` — and encoded into the
/// zero-free base-38 polynomial.
///
/// ## Non-null mapping and zero-state singularities
///
/// Standard continuous (base-10) real-number representations admit `0.0` as a
/// valid state. A fully disconnected matrix (zero contact weight) produces a
/// division by zero when a normalized compaction index is required — a
/// zero-state singularity, the model's analog of apoptotic collapse. The
/// bijective encoding prevents this in two ways:
///
/// * Every representable polynomial state is nonzero: digit zero is excluded
///   by construction, so no internal placeholder zero can arise.
/// * The raw compaction signature (not a normalized ratio) is encoded, so the
///   only degenerate input — the fully disconnected matrix — maps to the
///   empty polynomial `∅`, which the model identifies explicitly as the
///   boundary state via [`Self::has_zero_state_singularity`] rather than
///   silently propagating `NaN`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChromatinContactMatrix {
    contacts: Vec<Vec<f64>>,
}

impl ChromatinContactMatrix {
    /// Construct from a square contact matrix. Rejects non-square input.
    pub fn new(contacts: Vec<Vec<f64>>) -> Result<Self, String> {
        let n = contacts.len();
        if n == 0 {
            return Err("contact matrix must be non-empty".to_string());
        }
        if contacts.iter().any(|row| row.len() != n) {
            return Err("contact matrix must be square".to_string());
        }
        Ok(Self { contacts })
    }

    /// Number of loci (matrix dimension).
    pub fn dimension(&self) -> usize {
        self.contacts.len()
    }

    /// Iterate the strict upper-triangle pairs `(i, j, c_ij)`.
    fn pairs(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        let n = self.dimension();
        (0..n).flat_map(move |i| (i + 1..n).map(move |j| (i, j, self.contacts[i][j])))
    }

    /// Total contact weight across all locus pairs, `Σ_{i<j} c_ij`.
    pub fn total_contact_weight(&self) -> f64 {
        self.pairs().map(|(_, _, c)| c).sum()
    }

    /// Contact-order-weighted span, `Σ_{i<j} c_ij · (j − i)`: a scalar
    /// compaction signature of the folding topology. Higher values indicate
    /// longer-range (more compacted) interactions.
    pub fn contact_order(&self) -> f64 {
        self.pairs().map(|(i, j, c)| c * (j - i) as f64).sum()
    }

    /// Mean contact-order: `contact_order / total_contact_weight`. Returns
    /// `None` when the matrix is fully disconnected — the zero-state
    /// singularity, where a continuous normalized ratio is undefined.
    pub fn compaction_index(&self) -> Option<f64> {
        let weight = self.total_contact_weight();
        (weight > 0.0).then(|| self.contact_order() / weight)
    }

    /// True when the matrix carries no inter-locus contacts (the degenerate,
    /// fully disconnected state).
    pub fn has_zero_state_singularity(&self) -> bool {
        self.total_contact_weight() == 0.0
    }

    /// Encode the folding topology as a zero-free base-38 polynomial: the
    /// contact-order signature, quantized at [`COMPACTION_SCALE`] resolution.
    /// The mapping is deterministic and reversible; no representable state is
    /// zero unless the signature itself is zero.
    pub fn encode_folding_topology(&self) -> ZeroFree38 {
        let signature = (self.contact_order() * COMPACTION_SCALE as f64).round() as u128;
        ZeroFree38::from_u128(signature)
    }
}

/// Information-theoretic comparison of the discrete bijective state space
/// (base-38) against standard continuous (base-10) representations.
///
/// ## Why a non-null bijective state space enables topological reversion
///
/// * **No zero state, hence no annihilation.** In continuous base-10
///   arithmetic, `0.0` is a valid representable state. A state that passes
///   through zero loses invertibility — division by zero, `NaN` propagation,
///   and the collapse of the state to nothingness (the model's analog of
///   apoptosis/necrosis). The bijective base-38 encoding excludes zero as a
///   digit and therefore as an internal state: every representable value is a
///   nonzero lattice point. Correction is always a *reversion* — a path in
///   the discrete lattice from a displaced state back to the reference —
///   never an annihilation. [`Self::zero_state_collapse_representable`]
///   returns `true` for base-10 and is structurally `false` for base-38.
///
/// * **No zero placeholders, hence full symbol informativeness.** Fixed-width
///   base-10 representations pad with zero digits (`3400` carries two zero
///   placeholders); those symbols occupy state space but encode no
///   information. The sparse bijective form omits implied-zero places
///   entirely, so every symbol in [`Self::encoded`] is an informative digit in
///   `1..=37`.
///
/// * **Higher per-symbol information capacity.** A base-38 symbol carries
///   `log₂(38) ≈ 5.25` bits versus `log₂(10) ≈ 3.32` for base-10, so the
///   same information occupies fewer symbols when the representational span is
///   held fixed.
///
/// These are structural properties of the encoding, demonstrated here as
/// computable quantities; they are not claims about physical or biological
/// efficacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateSpaceComparison {
    value: u128,
}

impl StateSpaceComparison {
    /// Construct a comparison for a given integer state.
    pub fn new(value: u128) -> Self {
        Self { value }
    }

    /// The integer state under comparison.
    pub fn value(&self) -> u128 {
        self.value
    }

    /// Information content of the state in bits: `log₂(value + 1)`.
    pub fn entropy_bits(&self) -> f64 {
        (self.value as f64 + 1.0).log2()
    }

    /// Fixed-width symbol count in base-10 (zero padding permitted).
    pub fn base10_fixed_symbols(&self) -> u32 {
        ceil_log(10.0, self.value)
    }

    /// Number of zero-placeholder symbols in the decimal representation.
    pub fn base10_zero_placeholders(&self) -> u32 {
        decimal_zero_placeholders(self.value)
    }

    /// Theoretical symbol count in base-38 (bijective, implied zeros
    /// permitted within the span).
    pub fn base38_theoretical_symbols(&self) -> u32 {
        ceil_log(BASE as f64, self.value)
    }

    /// Number of nonzero digits actually occupied in the sparse encoding
    /// (`≤` the theoretical span).
    pub fn base38_sparse_digits(&self) -> usize {
        self.encoded().digit_count()
    }

    /// Number of zero digits in the base-38 encoding. Structural invariant:
    /// always zero — digit zero is excluded by construction.
    pub fn base38_zero_digits(&self) -> usize {
        0
    }

    /// The value encoded as a zero-free base-38 polynomial.
    pub fn encoded(&self) -> ZeroFree38 {
        ZeroFree38::from_u128(self.value)
    }

    /// Whether the base-10 continuous state space admits the zero state
    /// (annihilation). Always `true` for base-10; structurally `false` for
    /// the bijective base-38 encoding.
    pub fn zero_state_collapse_representable(&self) -> bool {
        true
    }
}

/// Aggregate audit of the scale-invariant encoding across all three systems.
#[derive(Debug, Clone)]
pub struct UniversalityAudit {
    pub tessellation: EpithelialTessellation,
    pub scale_breakdown: ScaleInvarianceBreakdown,
    pub contact_map: ChromatinContactMatrix,
    pub state_comparison: StateSpaceComparison,
}

/// Run the universality audit on a mocked malignant tissue sample and print
/// the boxed telemetry to the terminal.
///
/// Mock sample: measured packing efficiency 0.780 (hexagonal ideal 0.907);
/// local proliferation scale 1.62 against global vascularization 0.87; a 4×4
/// contact matrix with a compacted (long-range) folding signature.
pub fn run_universality_audit() -> UniversalityAudit {
    let tessellation = EpithelialTessellation::new(0.780);
    let scale_breakdown = ScaleInvarianceBreakdown::new(1.62, 0.87);
    let contact_map = ChromatinContactMatrix::new(vec![
        vec![1.0, 0.8, 0.3, 0.1],
        vec![0.8, 1.0, 0.6, 0.2],
        vec![0.3, 0.6, 1.0, 0.7],
        vec![0.1, 0.2, 0.7, 1.0],
    ])
    .expect("mock contact matrix is square");

    let signature = contact_map.encode_folding_topology().to_u128().unwrap_or(0);
    let state_comparison = StateSpaceComparison::new(signature);

    let audit = UniversalityAudit {
        tessellation,
        scale_breakdown,
        contact_map,
        state_comparison,
    };
    println!("{audit}");
    audit
}

impl std::fmt::Display for UniversalityAudit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = &self.tessellation;
        let s = &self.scale_breakdown;
        let c = &self.contact_map;
        let q = &self.state_comparison;
        let compaction = c
            .compaction_index()
            .map(|v| format!("{v:.4}"))
            .unwrap_or_else(|| "undefined (zero-state singularity)".to_string());

        writeln!(
            f,
            "+============================================================+"
        )?;
        writeln!(
            f,
            "|  UNIVERSALITY AUDIT — SCALE-INVARIANT RBDC ENCODING       |"
        )?;
        writeln!(
            f,
            "+============================================================+"
        )?;
        writeln!(f, "  EPITHELIAL TESSELLATION (Voronoi / honeycomb packing)")?;
        writeln!(
            f,
            "    measured efficiency : {:.3}  (hexagonal ideal {:.3})",
            t.measured_efficiency(),
            EpithelialTessellation::hexagonal_ideal()
        )?;
        writeln!(
            f,
            "    packing deficit     : {:.4}  ({} basis points)",
            t.packing_deficit(),
            t.deficit_basis_points()
        )?;
        writeln!(f, "    deficit polynomial : {}", t.deficit_polynomial())?;
        writeln!(
            f,
            "    deficit (base-100) : {}   [% digits: whole/fractional]",
            t.deficit_polynomial_percent()
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  SCALE-INVARIANCE BREAKDOWN (metric expansion discrepancy)"
        )?;
        writeln!(
            f,
            "    local proliferation : {:.3}x | global vascularization: {:.3}x",
            s.local_scale_factor(),
            s.global_scale_factor()
        )?;
        writeln!(
            f,
            "    expansion discrepancy: {:.4}  ({} basis points)",
            s.metric_expansion_discrepancy(),
            s.discrepancy_basis_points()
        )?;
        writeln!(
            f,
            "    resync factor        : {:.4}x (global / local)",
            s.resynchronization_factor()
        )?;
        writeln!(f, "    compensatory vector : {}", s.compensatory_vector())?;
        writeln!(
            f,
            "    compensatory (base-100): {}",
            s.compensatory_vector_percent()
        )?;
        writeln!(
            f,
            "    desync coupling      : {:.4}  (→ 456 Hz carrier amplitude)",
            s.desync_factor()
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  POLYMER CONTACT TOPOLOGY (chromatin folding, {}×{})",
            c.dimension(),
            c.dimension()
        )?;
        writeln!(
            f,
            "    contact order       : {:.4} | compaction index: {}",
            c.contact_order(),
            compaction
        )?;
        writeln!(
            f,
            "    folding polynomial  : {}  (= {})",
            c.encode_folding_topology(),
            c.encode_folding_topology().to_u128().unwrap_or(0)
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  STATE-SPACE COMPARISON (bijective base-38 vs continuous base-10)"
        )?;
        writeln!(f, "    value              : {}", q.value())?;
        writeln!(
            f,
            "    information        : {:.2} bits  (log₂ of value + 1)",
            q.entropy_bits()
        )?;
        writeln!(
            f,
            "    base-10 fixed-width: {} symbols | {} zero placeholder(s) | zero state allowed",
            q.base10_fixed_symbols(),
            q.base10_zero_placeholders()
        )?;
        writeln!(
            f,
            "    base-38 bijective  : {} symbols (sparse {}) | {} zero digit(s) | zero state excluded",
            q.base38_theoretical_symbols(),
            q.base38_sparse_digits(),
            q.base38_zero_digits()
        )?;
        writeln!(
            f,
            "+============================================================+"
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn hexagonal_ideal_is_honeycomb() {
        assert!(approx(EpithelialTessellation::hexagonal_ideal(), 0.9069));
    }

    #[test]
    fn healthy_tessellation_has_no_deficit() {
        let healthy = EpithelialTessellation::new(EpithelialTessellation::hexagonal_ideal());
        assert_eq!(healthy.packing_deficit(), 0.0);
        assert_eq!(healthy.deficit_basis_points(), 0);
        assert!(healthy.deficit_polynomial().is_zero());
        assert_eq!(healthy.deficit_polynomial().to_string(), "∅");
    }

    #[test]
    fn malignant_tessellation_maps_deficit_to_polynomial() {
        let malignant = EpithelialTessellation::new(0.780);
        assert!(approx(malignant.packing_deficit(), 0.1269));
        assert_eq!(malignant.deficit_basis_points(), 1269);
        assert_eq!(malignant.deficit_polynomial().to_string(), "15 + 33·38");
        assert_eq!(malignant.deficit_polynomial().to_u128(), Some(1269));
    }

    #[test]
    fn lower_packing_raises_deficit_monotonically() {
        let a = EpithelialTessellation::new(0.85);
        let b = EpithelialTessellation::new(0.70);
        assert!(b.packing_deficit() > a.packing_deficit());
        assert!(b.deficit_basis_points() > a.deficit_basis_points());
    }

    #[test]
    fn synchronized_scales_have_no_discrepancy() {
        let synced = ScaleInvarianceBreakdown::new(1.0, 1.0);
        assert_eq!(synced.metric_expansion_discrepancy(), 0.0);
        assert_eq!(synced.discrepancy_basis_points(), 0);
        assert!(synced.compensatory_vector().is_zero());
        assert_eq!(synced.compensatory_vector().to_string(), "∅");
        assert!(approx(synced.resynchronization_factor(), 1.0));
    }

    #[test]
    fn broken_scale_invariance_maps_to_compensatory_vector() {
        let broken = ScaleInvarianceBreakdown::new(1.62, 0.87);
        assert!(approx(broken.metric_expansion_discrepancy(), 0.8621));
        assert_eq!(broken.discrepancy_basis_points(), 8621);
        assert_eq!(
            broken.compensatory_vector().to_string(),
            "33 + 36·38 + 5·38^2"
        );
        assert_eq!(broken.compensatory_vector().to_u128(), Some(8621));
        assert!(approx(broken.resynchronization_factor(), 0.5370));
    }

    #[test]
    fn percent_encoding_reads_percent_semantics() {
        let t = EpithelialTessellation::new(0.780);
        let p = t.deficit_polynomial_percent();
        assert_eq!(p.to_string(), "69 + 12·100");
        assert_eq!(p.to_u128(), Some(1269));
        assert_eq!(
            p.terms().iter().find(|x| x.place == 1).unwrap().digit.get(),
            12
        );
        assert_eq!(
            p.terms().iter().find(|x| x.place == 0).unwrap().digit.get(),
            69
        );

        let s = ScaleInvarianceBreakdown::new(1.62, 0.87);
        let q = s.compensatory_vector_percent();
        assert_eq!(q.to_string(), "21 + 86·100");
        assert_eq!(q.to_u128(), Some(8621));
        assert_eq!(
            q.terms().iter().find(|x| x.place == 1).unwrap().digit.get(),
            86
        );
        assert_eq!(
            q.terms().iter().find(|x| x.place == 0).unwrap().digit.get(),
            21
        );
    }

    #[test]
    fn percent_encoding_matches_base38_integer_semantics() {
        let t = EpithelialTessellation::new(0.780);
        assert_eq!(
            t.deficit_polynomial_percent().to_u128(),
            t.deficit_polynomial().to_u128()
        );
        let s = ScaleInvarianceBreakdown::new(1.62, 0.87);
        assert_eq!(
            s.compensatory_vector_percent().to_u128(),
            s.compensatory_vector().to_u128()
        );
    }

    #[test]
    fn percent_encoding_zero_is_void() {
        let healthy = EpithelialTessellation::new(EpithelialTessellation::hexagonal_ideal());
        assert!(healthy.deficit_polynomial_percent().is_zero());
        assert_eq!(healthy.deficit_polynomial_percent().to_string(), "∅");
        let synced = ScaleInvarianceBreakdown::new(1.0, 1.0);
        assert!(synced.compensatory_vector_percent().is_zero());
        assert_eq!(synced.compensatory_vector_percent().to_string(), "∅");
    }

    #[test]
    fn desync_factor_clamps_discrepancy() {
        let synced = ScaleInvarianceBreakdown::new(1.0, 1.0);
        assert_eq!(synced.desync_factor(), 0.0);
        let broken = ScaleInvarianceBreakdown::new(1.62, 0.87);
        assert!(approx(broken.desync_factor(), 0.8621));
        let saturated = ScaleInvarianceBreakdown::new(3.0, 1.0);
        assert!(approx(saturated.desync_factor(), 1.0));
    }

    #[test]
    fn contact_matrix_rejects_non_square() {
        assert!(ChromatinContactMatrix::new(vec![vec![1.0, 2.0], vec![3.0]]).is_err());
        assert!(ChromatinContactMatrix::new(vec![]).is_err());
    }

    #[test]
    fn folding_topology_encodes_compaction_signature() {
        let matrix = ChromatinContactMatrix::new(vec![
            vec![1.0, 0.8, 0.3, 0.1],
            vec![0.8, 1.0, 0.6, 0.2],
            vec![0.3, 0.6, 1.0, 0.7],
            vec![0.1, 0.2, 0.7, 1.0],
        ])
        .unwrap();

        assert!(approx(matrix.total_contact_weight(), 2.7));
        assert!(approx(matrix.contact_order(), 3.4));
        assert!(approx(matrix.compaction_index().unwrap(), 1.2593));

        let encoded = matrix.encode_folding_topology();
        assert_eq!(encoded.to_u128(), Some(3400));
        assert_eq!(encoded.to_string(), "18 + 13·38 + 2·38^2");
        assert!(!matrix.has_zero_state_singularity());
    }

    #[test]
    fn disconnected_matrix_is_zero_state_singularity() {
        let matrix = ChromatinContactMatrix::new(vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ])
        .unwrap();

        assert!(matrix.has_zero_state_singularity());
        assert!(matrix.compaction_index().is_none());
        assert!(matrix.encode_folding_topology().is_zero());
        assert_eq!(matrix.encode_folding_topology().to_string(), "∅");
    }

    #[test]
    fn folding_encoding_roundtrips() {
        let matrix = ChromatinContactMatrix::new(vec![vec![1.0, 0.5], vec![0.5, 1.0]]).unwrap();
        let encoded = matrix.encode_folding_topology();
        let reparsed = encoded.to_u128().unwrap();
        assert_eq!(ZeroFree38::from_u128(reparsed), encoded);
    }

    #[test]
    fn state_space_comparison_quantities() {
        let c = StateSpaceComparison::new(3400);
        assert_eq!(c.value(), 3400);
        assert!((c.entropy_bits() > 11.7) && (c.entropy_bits() < 11.8));
        assert_eq!(c.base10_fixed_symbols(), 4);
        assert_eq!(c.base10_zero_placeholders(), 2);
        assert_eq!(c.base38_theoretical_symbols(), 3);
        assert_eq!(c.base38_sparse_digits(), 3);
        assert_eq!(c.base38_zero_digits(), 0);
        assert!(c.zero_state_collapse_representable());
        assert_eq!(c.encoded().to_string(), "18 + 13·38 + 2·38^2");
    }

    #[test]
    fn bijective_encoding_never_contains_zero_digit() {
        for value in [38u128, 850, 1269, 3400, 8621, 53_016] {
            let encoded = StateSpaceComparison::new(value).encoded();
            assert!(
                encoded.terms().iter().all(|t| t.digit.get() != 0),
                "zero digit found for {value}"
            );
        }
    }

    #[test]
    fn universality_audit_runs() {
        let audit = run_universality_audit();
        assert_eq!(audit.tessellation.deficit_basis_points(), 1269);
        assert_eq!(audit.scale_breakdown.discrepancy_basis_points(), 8621);
        assert_eq!(
            audit.contact_map.encode_folding_topology().to_u128(),
            Some(3400)
        );
        assert_eq!(audit.state_comparison.value(), 3400);
    }
}
