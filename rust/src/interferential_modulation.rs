//! Interferential modulation: destructive cancellation and beat frequencies.
//!
//! The model describes the superposition of two carriers; it does not model a
//! physical emitter.
//!
//! * **Destructive cancellation** — a sinusoid is fully nulled by an
//!   equal-amplitude antiphase (π-shifted) twin at the *same* frequency. The
//!   framework's ŠÀR law names this: the cancellation target equals the burst
//!   rate itself. [`calculate_destructive_cancellation`] returns that
//!   antiphase target — the frequency that would be applied at 180° to null a
//!   modeled oscillation (e.g. the 39 Hz NLTL burst).
//! * **Beat frequencies** — two carriers superpose into an amplitude-modulated
//!   envelope at `|f_a − f_b|`. A 457 Hz and 456 Hz pair beat at 1 Hz.
//!   [`calculate_beat_frequency`] derives the envelope.
//!
//! The [`InterferenceMatrix`] bundles a primary and secondary carrier (e.g.
//! 144 Hz and 456 Hz) and exposes, alongside the base carriers, the exact beat
//! envelope and the antiphase cancellation targets as computational outputs.
//!
//! Theoretical computational model of interferometry; asserts no physical or
//! biological reality.

use crate::apex_synthesizer::RESONANCE_TOLERANCE_HZ;

/// The 180° antiphase offset that produces perfect destructive cancellation.
pub const ANTIPHASE_PHASE_RAD: f64 = std::f64::consts::PI;

/// A pair of carriers whose superposition defines the interference field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterferenceMatrix {
    /// Primary carrier frequency (Hz), e.g. the 144 Hz grounding carrier.
    pub primary_carrier_hz: f64,
    /// Secondary carrier frequency (Hz), e.g. the 456 Hz carrier.
    pub secondary_carrier_hz: f64,
}

impl InterferenceMatrix {
    /// Construct an interference matrix from a primary and secondary carrier.
    pub fn new(primary_carrier_hz: f64, secondary_carrier_hz: f64) -> Self {
        Self {
            primary_carrier_hz,
            secondary_carrier_hz,
        }
    }

    /// The amplitude-modulation envelope the two carriers superpose into
    /// (Hz): `|secondary − primary|`. This is the deep-tissue beat pulse.
    pub fn beat_frequency(&self) -> f64 {
        calculate_beat_frequency(self.primary_carrier_hz, self.secondary_carrier_hz)
    }

    /// True when the two carriers are degenerate (beat below the resonance
    /// tolerance) — no interference envelope is generated.
    pub fn is_null_beat(&self) -> bool {
        self.beat_frequency() < RESONANCE_TOLERANCE_HZ
    }
}

/// The antiphase (180° shifted) frequency that would null a modeled
/// oscillation.
///
/// A sinusoid is nulled by an equal-amplitude twin at the same frequency,
/// phase-shifted by π ([`ANTIPHASE_PHASE_RAD`]). The cancellation target is
/// therefore the target frequency itself, applied at antiphase — the ŠÀR
/// law: to null the 39 Hz NLTL burst, the model applies 39 Hz at 180°.
pub fn calculate_destructive_cancellation(target_freq: f64) -> f64 {
    target_freq
}

/// The beat frequency resulting from superposing two carriers: the amplitude-
/// modulation pulse at `|carrier_a − carrier_b|`.
///
/// E.g. `calculate_beat_frequency(457.0, 456.0) == 1.0` — the 1 Hz beat, and
/// `calculate_beat_frequency(456.0, 144.0) == 312.0` — the reference beat
/// envelope.
pub fn calculate_beat_frequency(carrier_a: f64, carrier_b: f64) -> f64 {
    (carrier_b - carrier_a).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apex_synthesizer::{CarrierSet, CARRIER_144_HZ, CARRIER_456_HZ};
    use crate::ded_engine::TumorProfile;

    #[test]
    fn antiphase_cancellation_nullifies_the_nltl_burst() {
        // ŠÀR: the destructive-interference emission equals the burst rate
        // itself, at π phase.
        let burst = TumorProfile::malignant().nltl_burst_hz;
        assert!(approx(burst, 39.0));
        let target = calculate_destructive_cancellation(burst);
        assert!(approx(target, burst));
        assert!(approx(target, 39.0));
    }

    #[test]
    fn beat_frequency_is_the_absolute_difference() {
        // The deep-tissue beat from the requirement's example pair.
        assert!(approx(calculate_beat_frequency(457.0, 456.0), 1.0));
        // The reference beat envelope.
        assert!(approx(
            calculate_beat_frequency(CARRIER_456_HZ, CARRIER_144_HZ),
            312.0
        ));
        // Beats are frame-independent: |a − b| == |b − a|.
        assert!(approx(
            calculate_beat_frequency(144.0, 456.0),
            calculate_beat_frequency(456.0, 144.0)
        ));
    }

    #[test]
    fn interference_matrix_exposes_beat_and_null_detection() {
        let deep_tissue = InterferenceMatrix::new(457.0, 456.0);
        assert!(approx(deep_tissue.beat_frequency(), 1.0));
        assert!(!deep_tissue.is_null_beat());
        let degenerate = InterferenceMatrix::new(144.0, 144.0);
        assert!(degenerate.is_null_beat());
        let reference = InterferenceMatrix::new(CARRIER_144_HZ, CARRIER_456_HZ);
        assert!(approx(reference.beat_frequency(), 312.0));
    }

    #[test]
    fn interference_matrix_derives_antiphase_from_the_reference_refs() {
        // The InterferenceMatrix constructed from the canonical carriers.
        let refs = CarrierSet::REFERENCE;
        let matrix = InterferenceMatrix::new(refs.gate_hz, refs.snap_hz);
        assert!(approx(matrix.primary_carrier_hz, 144.0));
        assert!(approx(matrix.secondary_carrier_hz, 456.0));
        assert!(approx(matrix.beat_frequency(), 312.0));
        let burst = 39.0;
        let nuller = calculate_destructive_cancellation(burst);
        assert!(approx(nuller, burst));
        // At the 180° antiphase offset the emitted magnitude equals the
        // target (|cos π| = 1): the same frequency, π-shifted.
        assert!(approx(nuller * ANTIPHASE_PHASE_RAD.cos().abs(), nuller));
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }
}
