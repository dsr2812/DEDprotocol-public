//! Apex compensatory matrix — maps a modeled tumor profile's deficit against
//! two evolutionary anchor polynomials and synthesizes a deterministic
//! candidate carrier specification.
//!
//! Theoretical computational model: the base-38 deficit polynomials
//! (`1·38` elephant TP53, `3 + 1·38` shark dielectric) are parsed and their
//! coefficients mapped onto the 144 Hz / 456 Hz carriers. The anchors are
//! structural definitions (see the constants below); the model asserts no
//! physical or biological reality.

use crate::core::{Digit, Term, ZeroFree38};
use crate::ded_engine::{DedResonanceEngine, GateSide, TmeProfile, TumorProfile};
use crate::universality::ScaleInvarianceBreakdown;

/// Elephant anchor carrier (THz) — telomere boundary.
pub const TELOMERE_BOUNDARY_THZ: f64 = 33.0;
/// Shark anchor carrier (THz) — MPTP gate.
pub const MPTP_RESET_THZ: f64 = 0.1;
/// 456 Hz carrier (Hz) — the 38 × 12 anchor.
pub const CARRIER_456_HZ: f64 = 456.0;
/// 144 Hz reference carrier (Hz).
pub const CARRIER_144_HZ: f64 = 144.0;

/// A pair of base carrier frequencies for the specification.
///
/// The reference pair (144 Hz gate / 456 Hz snap) is the framework's canonical
/// carrier set. Supplying any other pair applies the same deficit structure to
/// different base frequencies; this is a computational comparison arm, not a
/// claim that the harmonics are biologically required.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarrierSet {
    /// Gate-channel base frequency (Hz). Reference value: 144.0.
    pub gate_hz: f64,
    /// Snap-channel base frequency (Hz). Reference value: 456.0.
    pub snap_hz: f64,
}

/// Tolerance (Hz) for treating a carrier pair as the canonical reference. This
/// is a modeling choice; a window-width refinement is left to a future
/// frequency-offset sensitivity sweep.
pub const RESONANCE_TOLERANCE_HZ: f64 = 1e-9;

impl CarrierSet {
    /// The canonical reference pair: 144 Hz / 456 Hz.
    pub const REFERENCE: Self = Self {
        gate_hz: CARRIER_144_HZ,
        snap_hz: CARRIER_456_HZ,
    };

    /// An off-reference pair (137 Hz / 420 Hz): same structure, different
    /// bases.
    pub fn off_harmonic() -> Self {
        Self {
            gate_hz: 137.0,
            snap_hz: 420.0,
        }
    }

    /// True only when both bases match the canonical reference pair.
    pub fn is_reference(&self) -> bool {
        (self.gate_hz - CARRIER_144_HZ).abs() <= RESONANCE_TOLERANCE_HZ
            && (self.snap_hz - CARRIER_456_HZ).abs() <= RESONANCE_TOLERANCE_HZ
    }
}

/// Elephant TP53 deficit, defined as the canonical Base-38 polynomial `1·38`
/// (= 38).
///
/// # Structural definition (radix identity)
/// The elephant anchor is a pure multiple of the Base-38 radix: one place-1
/// unit with an implied-zero units place. Its base-38 expansion `1·38` is the
/// *radix identity* — the smallest nonzero integer that is exactly one radix.
/// It carries no units digit, so `coefficient_at(0) == 0` and it contributes
/// nothing to the 144 Hz (place-0) carrier modulation. This is a design
/// choice, not a quantity derived from measurement.
pub const ELEPHANT_TP53_POLY: &str = "1·38";

/// Shark dielectric deficit, defined as the canonical Base-38 polynomial
/// `3 + 1·38` (= 41).
///
/// # Structural definition (triadic coefficient)
/// The shark anchor is a coefficient 3 at the units place plus one place-1
/// unit: `3 + 1·38`. The units digit `3` is a framework coefficient (one per
/// correction vector ŠÀR·IM·UŠ₂), and the place-1 digit `1` is the same radix
/// unit as the elephant. This is a design choice, not a derived or measured
/// quantity.
pub const SHARK_DIELECTRIC_POLY: &str = "3 + 1·38";

/// Full-scale digit used to normalize deficit coefficients into 0..1 indices.
const DIGIT_NORM: f64 = 37.0;

/// A Base-38 deficit polynomial parsed from the canonical engine strings,
/// e.g. `"1·38"` (Elephant TP53) or `"3 + 1·38"` (Shark dielectric).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApexDeficit {
    name: String,
    terms: Vec<Term>,
}

fn parse_poly_term(chunk: &str) -> Option<(u32, u8)> {
    let chunk = chunk.trim();
    if let Some((digit, place)) = chunk.split_once("·38^") {
        return Some((place.trim().parse().ok()?, digit.trim().parse().ok()?));
    }
    if let Some(digit) = chunk.strip_suffix("·38") {
        return Some((1, digit.trim().parse().ok()?));
    }
    Some((0, chunk.parse().ok()?))
}

impl ApexDeficit {
    /// Build a deficit from a canonical Base-38 polynomial string
    /// (`"d"`, `"d·38"`, `"d·38^p"`, terms joined by `" + "`, or `"∅"`).
    pub fn from_poly(name: &str, poly: &str) -> Result<Self, String> {
        let trimmed = poly.trim();
        if trimmed.is_empty() || trimmed == "∅" {
            return Ok(Self {
                name: name.to_string(),
                terms: Vec::new(),
            });
        }
        let mut terms = Vec::new();
        for chunk in trimmed.split('+') {
            let (place, digit) = parse_poly_term(chunk)
                .ok_or_else(|| format!("unparseable term '{chunk}' in '{poly}'"))?;
            let digit = Digit::new(digit)
                .filter(|d| (d.get() as u32) < crate::core::BASE)
                .ok_or_else(|| format!("digit {digit} out of 1..=37 in '{poly}'"))?;
            terms.push(Term { place, digit });
        }
        terms.sort_unstable_by_key(|t| t.place);
        for w in terms.windows(2) {
            if w[0].place == w[1].place {
                return Err(format!("duplicate place {} in '{poly}'", w[0].place));
            }
        }
        Ok(Self {
            name: name.to_string(),
            terms,
        })
    }

    /// Build a deficit directly from an integer value via the normalize engine.
    pub fn from_value(name: &str, value: u128) -> Self {
        let zero_free = ZeroFree38::from_u128(value);
        Self {
            name: name.to_string(),
            terms: zero_free.terms().to_vec(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// Integer value of the polynomial.
    pub fn value(&self) -> u128 {
        self.terms.iter().fold(0u128, |acc, t| {
            acc + 38u128.pow(t.place) * t.digit.get() as u128
        })
    }

    /// Coefficient at a place value, `0` if the place is an implied zero.
    pub fn coefficient_at(&self, place: u32) -> u8 {
        self.terms
            .iter()
            .find(|t| t.place == place)
            .map(|t| t.digit.get())
            .unwrap_or(0)
    }

    /// Number of nonzero terms.
    pub fn digit_count(&self) -> usize {
        self.terms.len()
    }

    /// Canonical Base-38 string (re-encoded through the normalize engine).
    pub fn to_zero_free38(&self) -> ZeroFree38 {
        ZeroFree38::from_u128(self.value())
    }
}

impl std::fmt::Display for ApexDeficit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            write!(f, "∅")
        } else {
            write!(f, "{}", self.to_zero_free38())
        }
    }
}

/// A rectangular pulse train driving one compensation axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseTrain {
    pub frequency_hz: f64,
    /// Fraction of each cycle spent at full amplitude, `0..=1`.
    pub duty_cycle: f64,
    pub pulses_per_burst: u32,
    /// Normalized field amplitude, `0..=1` (0 = no compensation needed).
    pub amplitude: f64,
}

/// The full synthesized field: modulated carriers, pulse trains, and the final
/// DED ŠÀR-IM-UŠ₂ correction frequencies.
#[derive(Debug, Clone)]
pub struct CompensatoryPayload {
    pub tumor: TumorProfile,
    pub elephant_deficit: ApexDeficit,
    pub shark_deficit: ApexDeficit,
    pub gate_deviation_mv: f64,
    pub gate_side: GateSide,
    pub void_proximity: f64,
    pub lattice_lock: f64,
    pub im: f64,
    /// Combined 144 Hz carrier modulation from place-0 deficit coefficients.
    pub m144: f64,
    /// Combined 456 Hz carrier modulation from place-1 deficit coefficients,
    /// scaled by the universality desynchronization coupling.
    pub m456: f64,
    pub carrier_144_hz: f64,
    pub carrier_456_hz: f64,
    /// Universality-layer desynchronization coupling (`0..=1`) applied to the
    /// 456 Hz UŠ₂ carrier amplitude. Zero = perfectly synchronized scales
    /// (carrier emitted at base deficit modulation); nonzero = broken
    /// scale-invariance (from [`ScaleInvarianceBreakdown::desync_factor`])
    /// amplifies the carrier modulation.
    pub scale_desync: f64,
    /// TME coupling block, present only when synthesized through the
    /// tumor-microenvironment path (acidic cloak + hypoxic evasion).
    pub tme: Option<TmeCorrection>,
    /// Elephant ratchet carrier: 33 THz, deficit-modulated.
    pub elephant_ratchet_hz: f64,
    /// Shark anchor carrier: 0.1 THz (MPTP reset), deficit-modulated.
    pub shark_mptp_hz: f64,
    pub snap_pulse: PulseTrain,
    pub gate_pulse: PulseTrain,
    pub tumor_ush2: ZeroFree38,
    pub tumor_ush2_value: u128,
    /// Final compensated UŠ₂ (tumor UŠ₂ + Elephant + Shark deficits).
    pub ush2: ZeroFree38,
    pub ush2_value: u128,
    /// ŠÀR destructive-interference frequency.
    pub sar_hz: f64,
    /// Final IM carrier frequency (the 19/19 gate push).
    pub im_hz: f64,
}

/// TME-coupled correction block attached to a synthesized payload.
#[derive(Debug, Clone)]
pub struct TmeCorrection {
    /// Acidic-cloak dielectric shielding factor, `0.0..=1.0`.
    pub dielectric_attenuation: f64,
    /// 144 Hz grounding carrier before the acidic-cloak amplitude boost (Hz).
    pub base_carrier_144_hz: f64,
    /// 144 Hz grounding carrier after the acidic-cloak amplitude boost (Hz).
    pub boosted_carrier_144_hz: f64,
    /// Hypoxic immune-evasion drive, `0.0..=1.0`.
    pub immune_evasion_deficit: f64,
    /// The immune-evasion drive as a zero-free Base-38 polynomial.
    pub immune_evasion_polynomial: ZeroFree38,
    /// ŠÀR (Phase-Extraction) frequency before the HIF-1α modulation (Hz).
    pub base_sar_hz: f64,
    /// ŠÀR frequency modulated to target the HIF-1α-stabilized burst (Hz).
    pub sar_hif_target_hz: f64,
}

/// Binds the Apex anchor constants and synthesizes compensatory fields.
///
/// The two anchor polynomials — [`ELEPHANT_TP53_POLY`] (`1·38`, radix identity)
/// and [`SHARK_DIELECTRIC_POLY`] (`3 + 1·38`, triadic coefficient on the
/// radix) — are structural definitions of the framework's Base-38 deficit
/// structure. The synthesizer parses them and maps their coefficients onto the
/// 144 Hz / 456 Hz carriers. Neither anchor is a measured quantity; both are
/// documented design choices.
#[derive(Debug, Clone, Copy, Default)]
pub struct ApexSynthesizer;

impl ApexSynthesizer {
    pub const fn new() -> Self {
        Self
    }

    /// Synthesize the compensatory field for a tumor profile.
    ///
    /// * Void-stretched geometry (FD ~1.79) snaps back via the Shark's
    ///   `3 + 1·38` dielectric pulse train.
    /// * Depolarized membrane (-7 mV) is pushed through the 19/19 Gate by the
    ///   144 Hz gate field.
    /// * The parsed Base-38 deficit coefficients modulate the 144 Hz and
    ///   456 Hz carriers directly (`m144` from place-0, `m456` from place-1).
    ///
    /// The deficit anchors come from [`ELEPHANT_TP53_POLY`] and
    /// [`SHARK_DIELECTRIC_POLY`]; see those constants for their axiomatic
    /// definitions.
    ///
    /// Equivalent to [`Self::synthesize_compensatory_field_with_desync`] with
    /// zero desynchronization coupling (the canonical, uncoupled payload).
    pub fn synthesize_compensatory_field(
        &self,
        tumor_profile: TumorProfile,
    ) -> CompensatoryPayload {
        self.synthesize_compensatory_field_with_desync(tumor_profile, 0.0)
    }

    /// Synthesize the compensatory field with universality-layer coupling.
    ///
    /// The `desync` term (clamped to `0..=1`) is the
    /// [`ScaleInvarianceBreakdown::desync_factor`] fed back from the
    /// scale-invariant extension: a broken scale-invariance amplifies the
    /// modulation applied to the 456 Hz UŠ₂ carrier amplitude
    /// (`m456_eff = m456_base · (1 + desync)`). At `desync == 0` the payload
    /// is bit-identical to the canonical [`Self::synthesize_compensatory_field`];
    /// at `desync == 1` the deficit modulation is doubled.
    pub fn synthesize_compensatory_field_with_desync(
        &self,
        tumor_profile: TumorProfile,
        desync: f64,
    ) -> CompensatoryPayload {
        self.synthesize_compensatory_field_with_desync_refs(
            tumor_profile,
            desync,
            CarrierSet::REFERENCE,
        )
    }

    /// [`Self::synthesize_compensatory_field_with_desync`] onto arbitrary
    /// carrier references. Identical deficit structure, off-harmonic bases.
    pub fn synthesize_compensatory_field_with_desync_refs(
        &self,
        tumor_profile: TumorProfile,
        desync: f64,
        refs: CarrierSet,
    ) -> CompensatoryPayload {
        let desync = desync.clamp(0.0, 1.0);
        let engine = DedResonanceEngine::with_master_clock(tumor_profile);
        let audit = engine.audit();
        let elephant = ApexDeficit::from_poly("Elephant TP53", ELEPHANT_TP53_POLY)
            .expect("canonical elephant polynomial");
        let shark = ApexDeficit::from_poly("Shark Dielectric", SHARK_DIELECTRIC_POLY)
            .expect("canonical shark polynomial");

        let m144 = (elephant.coefficient_at(0) + shark.coefficient_at(0)) as f64 / DIGIT_NORM;
        let m456_base = (elephant.coefficient_at(1) + shark.coefficient_at(1)) as f64 / DIGIT_NORM;
        let m456 = m456_base * (1.0 + desync);
        let carrier_144 = refs.gate_hz * (1.0 + m144);
        let carrier_456 = refs.snap_hz * (1.0 + m456);

        let m456_shark = shark.coefficient_at(1) as f64 / DIGIT_NORM;
        let m144_shark = shark.coefficient_at(0) as f64 / DIGIT_NORM;
        let snap_pulse = PulseTrain {
            frequency_hz: refs.snap_hz * (1.0 + m456_shark),
            duty_cycle: shark.coefficient_at(0) as f64 / 38.0,
            pulses_per_burst: shark.digit_count() as u32,
            amplitude: audit.void_proximity,
        };
        let gate_pulse = PulseTrain {
            frequency_hz: refs.gate_hz * (1.0 + audit.im),
            duty_cycle: audit.im,
            pulses_per_burst: audit.gate_deviation_mv.abs().round() as u32,
            amplitude: audit.im,
        };

        let ush2_value = audit.ush2_value + elephant.value() + shark.value();
        CompensatoryPayload {
            tumor: tumor_profile,
            elephant_deficit: elephant,
            shark_deficit: shark,
            gate_deviation_mv: audit.gate_deviation_mv,
            gate_side: audit.gate_side,
            void_proximity: audit.void_proximity,
            lattice_lock: audit.lattice_lock,
            im: audit.im,
            m144,
            m456,
            carrier_144_hz: carrier_144,
            carrier_456_hz: carrier_456,
            scale_desync: desync,
            elephant_ratchet_hz: TELOMERE_BOUNDARY_THZ * 1e12 * (1.0 + 1.0 / DIGIT_NORM),
            shark_mptp_hz: MPTP_RESET_THZ * 1e12 * (1.0 + m144_shark),
            snap_pulse,
            gate_pulse,
            tumor_ush2: audit.ush2.clone(),
            tumor_ush2_value: audit.ush2_value,
            ush2: ZeroFree38::from_u128(ush2_value),
            ush2_value,
            sar_hz: audit.shar_hz,
            im_hz: gate_pulse.frequency_hz,
            tme: None,
        }
    }

    /// Synthesize the field with the coupling read directly from a
    /// [`ScaleInvarianceBreakdown`]: the universality discrepancy is fed into
    /// the 456 Hz carrier as [`ScaleInvarianceBreakdown::desync_factor`].
    pub fn synthesize_with_universal_desync(
        &self,
        tumor_profile: TumorProfile,
        breakdown: ScaleInvarianceBreakdown,
    ) -> CompensatoryPayload {
        self.synthesize_compensatory_field_with_desync(tumor_profile, breakdown.desync_factor())
    }

    /// Synthesize the compensatory field with tumor-microenvironment coupling.
    ///
    /// Composes the universality desync coupling (456 Hz UŠ₂ carrier) with the
    /// two TME channels:
    ///
    /// * **Acidic cloak** — the pH-driven dielectric attenuation
    ///   ([`TmeProfile::calculate_dielectric_attenuation`]) raises the
    ///   amplitude of the 144 Hz grounding carrier so the field can penetrate
    ///   the acidified interstitial matrix:
    ///   `m144_eff = m144 · (1 + attenuation)`.
    /// * **Hypoxic immune evasion** — the HIF-1α/PD-L1 evasion deficit
    ///   ([`TmeProfile::calculate_immune_evasion_deficit`]) modulates the
    ///   ŠÀR (Phase-Extraction) channel to target the stabilized hypoxic
    ///   burst: `sar_hif_target = sar · (1 + evasion)`.
    ///
    /// The pre-TME values are retained in [`TmeCorrection`] so the base and
    /// corrected channels are both auditable.
    pub fn synthesize_compensatory_field_with_tme(
        &self,
        tumor_profile: TumorProfile,
        tme: TmeProfile,
        desync: f64,
    ) -> CompensatoryPayload {
        self.synthesize_compensatory_field_with_tme_refs(
            tumor_profile,
            tme,
            desync,
            CarrierSet::REFERENCE,
        )
    }

    /// [`Self::synthesize_compensatory_field_with_tme`] onto arbitrary carrier
    /// references.
    pub fn synthesize_compensatory_field_with_tme_refs(
        &self,
        tumor_profile: TumorProfile,
        tme: TmeProfile,
        desync: f64,
        refs: CarrierSet,
    ) -> CompensatoryPayload {
        let mut payload =
            self.synthesize_compensatory_field_with_desync_refs(tumor_profile, desync, refs);

        let attenuation = tme.calculate_dielectric_attenuation();
        let base_carrier_144 = payload.carrier_144_hz;
        let m144_eff = payload.m144 * (1.0 + attenuation);
        let boosted_carrier_144 = refs.gate_hz * (1.0 + m144_eff);

        let evasion = tme.calculate_immune_evasion_deficit();
        let base_sar = payload.sar_hz;
        let sar_hif_target = base_sar * (1.0 + evasion);

        payload.m144 = m144_eff;
        payload.carrier_144_hz = boosted_carrier_144;
        payload.sar_hz = sar_hif_target;
        payload.tme = Some(TmeCorrection {
            dielectric_attenuation: attenuation,
            base_carrier_144_hz: base_carrier_144,
            boosted_carrier_144_hz: boosted_carrier_144,
            immune_evasion_deficit: evasion,
            immune_evasion_polynomial: tme.immune_evasion_polynomial(),
            base_sar_hz: base_sar,
            sar_hif_target_hz: sar_hif_target,
        });
        payload
    }
}

impl std::fmt::Display for CompensatoryPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let side = match self.gate_side {
            GateSide::Above => "ABOVE",
            GateSide::Below => "BELOW",
        };
        let snap = &self.snap_pulse;
        let gate = &self.gate_pulse;
        writeln!(
            f,
            "+============================================================+"
        )?;
        writeln!(
            f,
            "|  APEX COMPENSATORY MATRIX — DED ŠÀR-IM-UŠ₂ PAYLOAD       |"
        )?;
        writeln!(
            f,
            "+============================================================+"
        )?;
        writeln!(
            f,
            "  TUMOR: RMP {:+.1} mV | FD {:.3} | Burst {:.1} Hz",
            self.tumor.rmp_mv, self.tumor.fractal_dim, self.tumor.nltl_burst_hz
        )?;
        writeln!(
            f,
            "    gate deviation : {:.2} mV ({side} 19/19) | IM {:.4}",
            self.gate_deviation_mv, self.im
        )?;
        writeln!(
            f,
            "    void proximity : {:.3} | lattice lock : {:.3}",
            self.void_proximity, self.lattice_lock
        )?;
        writeln!(f)?;
        writeln!(f, "  APEX DEFICITS (Base-38):")?;
        writeln!(
            f,
            "    {:<18}: {}  (= {})",
            self.elephant_deficit.name(),
            self.elephant_deficit,
            self.elephant_deficit.value()
        )?;
        writeln!(
            f,
            "    {:<18}: {}  (= {})",
            self.shark_deficit.name(),
            self.shark_deficit,
            self.shark_deficit.value()
        )?;
        writeln!(f)?;
        writeln!(f, "  CARRIER MODULATION (deficit coefficients → indices):")?;
        writeln!(
            f,
            "    m144 = {:.4}  ->  144 Hz carrier = {:.2} Hz",
            self.m144, self.carrier_144_hz
        )?;
        writeln!(
            f,
            "    m456 = {:.4}  ->  456 Hz carrier = {:.2} Hz",
            self.m456, self.carrier_456_hz
        )?;
        writeln!(
            f,
            "    scale desync = {:.4}  (universality coupling on 456 Hz)",
            self.scale_desync
        )?;
        writeln!(
            f,
            "    elephant ratchet : {:.3e} Hz (33 THz × {:.4})",
            self.elephant_ratchet_hz,
            1.0 + 1.0 / DIGIT_NORM
        )?;
        writeln!(
            f,
            "    shark MPTP reset : {:.3e} Hz (0.1 THz × {:.4})",
            self.shark_mptp_hz,
            1.0 + self.shark_deficit.coefficient_at(0) as f64 / DIGIT_NORM
        )?;
        writeln!(f)?;
        writeln!(f, "  PULSE TRAINS:")?;
        writeln!(
            f,
            "    shark snap : {:>9.2} Hz | duty {:.2}% | {} pulses | amp {:.3}",
            snap.frequency_hz,
            snap.duty_cycle * 100.0,
            snap.pulses_per_burst,
            snap.amplitude
        )?;
        writeln!(
            f,
            "    gate push  : {:>9.2} Hz | duty {:.2}% | {} pulses | amp {:.3}",
            gate.frequency_hz,
            gate.duty_cycle * 100.0,
            gate.pulses_per_burst,
            gate.amplitude
        )?;
        writeln!(f)?;
        if let Some(tme) = &self.tme {
            writeln!(f, "  TUMOR MICROENVIRONMENT (TME) COUPLING:")?;
            writeln!(
                f,
                "    acidic cloak attenuation : {:.4}",
                tme.dielectric_attenuation
            )?;
            writeln!(
                f,
                "    144 Hz grounding carrier : {:.2} Hz -> {:.2} Hz (amplitude boost)",
                tme.base_carrier_144_hz, tme.boosted_carrier_144_hz
            )?;
            writeln!(
                f,
                "    immune evasion deficit  : {:.4}  ({})",
                tme.immune_evasion_deficit, tme.immune_evasion_polynomial
            )?;
            writeln!(
                f,
                "    ŠÀR HIF-1α target       : {:.2} Hz -> {:.2} Hz",
                tme.base_sar_hz, tme.sar_hif_target_hz
            )?;
            writeln!(f)?;
        }
        writeln!(f, "  FINAL DED PROTOCOL VECTORS:")?;
        writeln!(
            f,
            "    ŠÀR (destructive) : {:.2} Hz @ antiphase π",
            self.sar_hz
        )?;
        writeln!(f, "    IM  (gate carrier): {:.2} Hz", self.im_hz)?;
        writeln!(
            f,
            "    UŠ₂ (compensated) : {}   (= {})  [tumor {} + {} + {}]",
            self.ush2,
            self.ush2_value,
            self.tumor_ush2_value,
            self.elephant_deficit.value(),
            self.shark_deficit.value()
        )?;
        writeln!(
            f,
            "+============================================================+"
        )?;
        Ok(())
    }
}

/// Malignant human reference cell through the synthesizer, with telemetry.
pub fn run_apex_audit() -> CompensatoryPayload {
    let payload = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
    println!("{payload}");
    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ded_engine::TRIBONACCI_VOID_FD;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn parses_deficit_polynomials() {
        let elephant = ApexDeficit::from_poly("Elephant", "1·38").unwrap();
        assert_eq!(elephant.value(), 38);
        assert_eq!(elephant.digit_count(), 1);
        assert_eq!(elephant.coefficient_at(0), 0);
        assert_eq!(elephant.coefficient_at(1), 1);

        let shark = ApexDeficit::from_poly("Shark", "3 + 1·38").unwrap();
        assert_eq!(shark.value(), 41);
        assert_eq!(shark.digit_count(), 2);
        assert_eq!(shark.coefficient_at(0), 3);
        assert_eq!(shark.coefficient_at(1), 1);

        let single = ApexDeficit::from_poly("Single", "5").unwrap();
        assert_eq!(single.value(), 5);
        assert_eq!(single.coefficient_at(0), 5);

        let full = ApexDeficit::from_poly("Full", "6 + 27·38 + 36·38^2").unwrap();
        assert_eq!(full.value(), 53_016);
        assert_eq!(full.coefficient_at(2), 36);
    }

    #[test]
    fn parses_void() {
        let void = ApexDeficit::from_poly("None", "∅").unwrap();
        assert_eq!(void.value(), 0);
        assert_eq!(void.digit_count(), 0);
        assert_eq!(void.to_string(), "∅");
    }

    #[test]
    fn from_value_roundtrips_through_display() {
        for v in [38u128, 41, 144, 771, 850, 2_099_999_999_864] {
            let deficit = ApexDeficit::from_value("test", v);
            let reparsed = ApexDeficit::from_poly("test", &deficit.to_string()).unwrap();
            assert_eq!(reparsed.value(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn rejects_malformed_polynomials() {
        assert!(ApexDeficit::from_poly("Bad", "38").is_err());
        assert!(ApexDeficit::from_poly("Bad", "1·38 + 1·38").is_err());
        assert!(ApexDeficit::from_poly("Bad", "x + 1·38").is_err());
    }

    #[test]
    fn anchor_axioms_parse_to_canonical_coefficients() {
        let elephant = ApexDeficit::from_poly("Elephant TP53", ELEPHANT_TP53_POLY).unwrap();
        assert_eq!(elephant.value(), 38);
        assert_eq!(elephant.coefficient_at(1), 1);
        assert_eq!(
            elephant.coefficient_at(0),
            0,
            "radix identity has no units digit"
        );
        assert_eq!(elephant.digit_count(), 1);

        let shark = ApexDeficit::from_poly("Shark Dielectric", SHARK_DIELECTRIC_POLY).unwrap();
        assert_eq!(shark.value(), 41);
        assert_eq!(shark.coefficient_at(0), 3, "triadic coefficient");
        assert_eq!(shark.coefficient_at(1), 1, "place-1 unit");
        assert_eq!(shark.digit_count(), 2);
    }

    #[test]
    fn malignant_compensatory_payload() {
        let payload =
            ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());

        assert_eq!(payload.elephant_deficit.value(), 38);
        assert_eq!(payload.shark_deficit.value(), 41);
        assert!(approx(payload.im, 0.75));
        assert!(approx(payload.m144, 3.0 / 37.0));
        assert!(approx(payload.m456, 2.0 / 37.0));
        assert!(approx(payload.scale_desync, 0.0));
        assert!(approx(payload.carrier_144_hz, 144.0 * 40.0 / 37.0));
        assert!(approx(payload.carrier_456_hz, 456.0 * 39.0 / 37.0));

        let snap = payload.snap_pulse;
        assert!(approx(snap.frequency_hz, 456.0 * 38.0 / 37.0));
        assert!(approx(snap.duty_cycle, 3.0 / 38.0));
        assert_eq!(snap.pulses_per_burst, 2);
        assert!(approx(snap.amplitude, 0.11 / (TRIBONACCI_VOID_FD - 1.68)));

        let gate = payload.gate_pulse;
        assert!(approx(gate.frequency_hz, 252.0));
        assert!(approx(gate.duty_cycle, 0.75));
        assert_eq!(gate.pulses_per_burst, 12);

        assert!(approx(payload.sar_hz, 39.0));
        assert!(approx(payload.im_hz, 252.0));

        assert_eq!(payload.tumor_ush2_value, 771);
        assert_eq!(payload.ush2_value, 850);
        assert_eq!(payload.ush2.to_string(), "14 + 22·38");

        assert!(approx(payload.elephant_ratchet_hz, 33.0e12 * 38.0 / 37.0));
        assert!(approx(payload.shark_mptp_hz, 0.1e12 * 40.0 / 37.0));
    }

    #[test]
    fn healthy_compensatory_payload_is_minimal() {
        let payload = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::healthy());
        assert!(approx(payload.void_proximity, 0.0));
        assert!(approx(payload.snap_pulse.amplitude, 0.0));
        assert!(approx(payload.im, 0.25));
        assert!(approx(payload.gate_pulse.frequency_hz, 180.0));
        assert_eq!(payload.gate_pulse.pulses_per_burst, 4);
        assert_eq!(payload.ush2_value, 456 + 38 + 41);
        assert_eq!(payload.ush2.to_string(), "3 + 14·38");
    }

    #[test]
    fn apex_audit_runs() {
        let payload = run_apex_audit();
        assert_eq!(payload.ush2_value, 850);
    }

    #[test]
    fn desync_zero_equals_canonical_payload() {
        let base = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        let zero = ApexSynthesizer::new()
            .synthesize_compensatory_field_with_desync(TumorProfile::malignant(), 0.0);
        assert!(approx(zero.m456, base.m456));
        assert!(approx(zero.carrier_456_hz, base.carrier_456_hz));
        assert!(approx(zero.scale_desync, base.scale_desync));
        assert_eq!(zero.ush2_value, base.ush2_value);
        assert!(approx(zero.sar_hz, base.sar_hz));
    }

    #[test]
    fn universal_desync_coupling_modulates_456_carrier() {
        let base = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        let breakdown = ScaleInvarianceBreakdown::new(1.62, 0.87);
        let coupled = ApexSynthesizer::new()
            .synthesize_with_universal_desync(TumorProfile::malignant(), breakdown);
        let desync = (1.62 - 0.87) / 0.87;

        assert!(approx(coupled.scale_desync, desync));
        assert!(approx(coupled.m456, base.m456 * (1.0 + desync)));
        assert!(coupled.carrier_456_hz > base.carrier_456_hz);

        assert!(approx(coupled.m144, base.m144));
        assert!(approx(coupled.im, base.im));
        assert_eq!(coupled.ush2_value, base.ush2_value);
        assert!(approx(coupled.sar_hz, base.sar_hz));
        assert_eq!(coupled.gate_pulse, base.gate_pulse);
    }

    #[test]
    fn desync_is_clamped_to_unit_interval() {
        let malignant = TumorProfile::malignant();
        let synth = ApexSynthesizer::new();
        let sat = synth.synthesize_compensatory_field_with_desync(malignant, 3.0);
        let one = synth.synthesize_compensatory_field_with_desync(malignant, 1.0);
        assert!(approx(sat.scale_desync, 1.0));
        assert!(approx(sat.carrier_456_hz, one.carrier_456_hz));

        let base = synth.synthesize_compensatory_field(malignant);
        let neg = synth.synthesize_compensatory_field_with_desync(malignant, -0.5);
        assert!(approx(neg.scale_desync, 0.0));
        assert!(approx(neg.carrier_456_hz, base.carrier_456_hz));
    }

    #[test]
    fn synchronized_scales_couple_to_zero() {
        let synced = ScaleInvarianceBreakdown::new(1.0, 1.0);
        let payload = ApexSynthesizer::new()
            .synthesize_with_universal_desync(TumorProfile::malignant(), synced);
        assert!(approx(payload.scale_desync, 0.0));
        assert!(approx(
            payload.carrier_456_hz,
            ApexSynthesizer::new()
                .synthesize_compensatory_field(TumorProfile::malignant())
                .carrier_456_hz
        ));
    }

    #[test]
    fn tme_boosts_144_carrier_and_sar_channel() {
        let base = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        let tme = TmeProfile::malignant();
        let payload = ApexSynthesizer::new().synthesize_compensatory_field_with_tme(
            TumorProfile::malignant(),
            tme,
            0.0,
        );

        let correction = payload.tme.as_ref().expect("TME correction present");
        assert!(approx(correction.dielectric_attenuation, 1.0));
        assert!(approx(correction.immune_evasion_deficit, 1.0));
        assert_eq!(correction.immune_evasion_polynomial.to_u128(), Some(10_000));

        // Acidic cloak raises the amplitude of the 144 Hz grounding carrier.
        assert!(payload.carrier_144_hz > base.carrier_144_hz);
        assert!(approx(
            correction.boosted_carrier_144_hz,
            payload.carrier_144_hz
        ));
        assert!(approx(payload.carrier_144_hz, 144.0 * (1.0 + payload.m144)));
        assert!(approx(payload.m144, base.m144 * 2.0));

        // Hypoxic evasion modulates the ŠÀR (Phase-Extraction) channel to
        // target the HIF-1α-stabilized burst.
        assert!(payload.sar_hz > base.sar_hz);
        assert!(approx(correction.sar_hif_target_hz, payload.sar_hz));
        assert!(approx(payload.sar_hz, base.sar_hz * 2.0));
        assert!(approx(base.sar_hz, 39.0));

        // The 456 Hz UŠ₂ channel is untouched at zero desync.
        assert!(approx(payload.carrier_456_hz, base.carrier_456_hz));
    }

    #[test]
    fn canonical_payload_has_no_tme_correction() {
        let payload =
            ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        assert!(payload.tme.is_none());
    }

    #[test]
    fn healthy_tme_leaves_payload_unchanged() {
        let base = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        let payload = ApexSynthesizer::new().synthesize_compensatory_field_with_tme(
            TumorProfile::malignant(),
            TmeProfile::healthy(),
            0.0,
        );
        let correction = payload.tme.as_ref().expect("TME correction present");
        assert_eq!(correction.dielectric_attenuation, 0.0);
        assert_eq!(correction.immune_evasion_deficit, 0.0);
        assert!(correction.immune_evasion_polynomial.is_zero());
        assert!(approx(payload.carrier_144_hz, base.carrier_144_hz));
        assert!(approx(payload.sar_hz, base.sar_hz));
    }
}
