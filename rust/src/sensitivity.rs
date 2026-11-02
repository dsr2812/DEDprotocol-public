//! Parameter-sensitivity and bifurcation analysis for the DED engine.
//!
//! Demonstrates that the model is a continuous mathematical heuristic with a
//! single, physically motivated phase transition — the 19/19 gate — rather than
//! a brittle numeric script. Two sweeps are provided:
//!
//!   * `sweep_rmp`: the resting membrane potential across the gate. The
//!     gate-mediated terms are continuous in the input and reach a minimum
//!     exactly at the bifurcation point; the structural payload (carriers,
//!     deficit polynomials, entropic-reversion polynomial) is invariant to
//!     membrane-potential perturbation.
//!   * `sweep_fractal_dim`: the boundary fractal dimension across the
//!     healthy reference (1.68) to Tribonacci limit (1.839) window. The
//!     void-proximity mapping saturates below the healthy reference and above
//!     the attractor limit, and is monotone non-decreasing between them.
//!
//! This is a computational model of a theoretical system; it asserts no
//! physical or biological reality.

use crate::apex_synthesizer::ApexSynthesizer;
use crate::ded_engine::{
    DedResonanceEngine, GateSide, TumorProfile, GATE_MV, HEALTHY_FD, MEMBRANE_SPAN_MV,
    TRIBONACCI_VOID_FD,
};

/// A single sweep point: the payload's dependence on one input parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct SensitivityPoint {
    pub rmp_mv: f64,
    pub fractal_dim: f64,
    pub gate_deviation_mv: f64,
    pub gate_side: GateSide,
    pub im: f64,
    pub void_proximity: f64,
    pub lattice_lock: f64,
    pub pulses_per_burst: u32,
    pub snap_amplitude: f64,
    pub carrier_456_hz: f64,
    pub ush2_value: u128,
}

fn point(profile: TumorProfile) -> SensitivityPoint {
    let engine = DedResonanceEngine::with_master_clock(profile);
    let audit = engine.audit();
    let payload = ApexSynthesizer::new().synthesize_compensatory_field(profile);
    SensitivityPoint {
        rmp_mv: profile.rmp_mv,
        fractal_dim: profile.fractal_dim,
        gate_deviation_mv: audit.gate_deviation_mv,
        gate_side: audit.gate_side,
        im: audit.im,
        void_proximity: audit.void_proximity,
        lattice_lock: audit.lattice_lock,
        pulses_per_burst: payload.gate_pulse.pulses_per_burst,
        snap_amplitude: payload.snap_pulse.amplitude,
        carrier_456_hz: payload.carrier_456_hz,
        ush2_value: payload.ush2_value,
    }
}

/// Sweep the resting membrane potential from `from_mv` to `to_mv` inclusive,
/// in `step_mv` steps, holding the remaining profile fields fixed.
pub fn sweep_rmp(
    from_mv: f64,
    to_mv: f64,
    step_mv: f64,
    base: TumorProfile,
) -> Vec<SensitivityPoint> {
    let mut points = Vec::new();
    let mut v = from_mv;
    while v <= to_mv + f64::EPSILON {
        let mut p = base;
        p.rmp_mv = v;
        points.push(point(p));
        v += step_mv;
    }
    points
}

/// Sweep the boundary fractal dimension from `from` to `to` inclusive, in
/// `step` steps, holding the remaining profile fields fixed.
pub fn sweep_fractal_dim(
    from: f64,
    to: f64,
    step: f64,
    base: TumorProfile,
) -> Vec<SensitivityPoint> {
    let mut points = Vec::new();
    let mut fd = from;
    while fd <= to + f64::EPSILON {
        let mut p = base;
        p.fractal_dim = fd;
        points.push(point(p));
        fd += step;
    }
    points
}

/// Run the full sensitivity audit and return a human-readable report.
pub fn run_sensitivity_audit() -> String {
    let base = TumorProfile::malignant();

    let rmp_pts = sweep_rmp(-30.0, -5.0, 0.5, base);
    let fd_pts = sweep_fractal_dim(1.60, 1.90, 0.005, base);

    let max_im_delta = rmp_pts
        .windows(2)
        .map(|w| (w[1].im - w[0].im).abs())
        .fold(0.0, f64::max);
    let flip_below = rmp_pts
        .iter()
        .filter(|p| p.gate_side == GateSide::Below)
        .map(|p| p.rmp_mv)
        .fold(f64::NEG_INFINITY, f64::max);
    let flip_above = rmp_pts
        .iter()
        .filter(|p| p.gate_side == GateSide::Above)
        .map(|p| p.rmp_mv)
        .fold(f64::INFINITY, f64::min);

    let structural_stable = rmp_pts.windows(2).all(|w| {
        (w[0].carrier_456_hz - w[1].carrier_456_hz).abs() < 1e-9
            && w[0].ush2_value == w[1].ush2_value
            && (w[0].snap_amplitude - w[1].snap_amplitude).abs() < 1e-9
    });

    let at_gate = rmp_pts
        .iter()
        .find(|p| (p.rmp_mv - GATE_MV).abs() < 1e-9)
        .map(|p| (p.gate_side, p.im, p.pulses_per_burst));

    let below_lock = fd_pts
        .iter()
        .filter(|p| p.fractal_dim < HEALTHY_FD)
        .all(|p| p.void_proximity == 0.0);
    let at_limit = fd_pts
        .iter()
        .filter(|p| p.fractal_dim >= TRIBONACCI_VOID_FD)
        .all(|p| p.void_proximity == 1.0);
    let fd_monotone = fd_pts.windows(2).all(|w| {
        w[1].ush2_value >= w[0].ush2_value && w[1].void_proximity + 1e-12 >= w[0].void_proximity
    });

    let ref_pt = &fd_pts[fd_pts
        .iter()
        .position(|p| (p.fractal_dim - 1.79).abs() < 1e-9)
        .unwrap_or(0)];

    let mut out = String::new();
    out.push_str("+==========================================================+\n");
    out.push_str("|  DED SENSITIVITY & BIFURCATION AUDIT                    |\n");
    out.push_str("+==========================================================+\n");
    out.push_str(&format!(
        "  RMP sweep        : -30.0 .. -5.0 mV (step 0.5)  [{} pts]\n",
        rmp_pts.len()
    ));
    out.push_str(&format!("    gate flip     : Below -> Above between {flip_below:.1} mV and {flip_above:.1} mV (gate = {GATE_MV:.0} mV)\n"));
    out.push_str(&format!(
        "    continuity    : max |ΔIM| = {max_im_delta:.4} (expected {} = step / membrane span)\n",
        0.5 / MEMBRANE_SPAN_MV
    ));
    out.push_str(&format!(
        "    at gate       : side = {:?}, IM = {:.0}, pulses = {}\n",
        at_gate.map(|g| g.0),
        at_gate.map(|g| g.1).unwrap_or(-1.0),
        at_gate.map(|g| g.2).unwrap_or(0)
    ));
    out.push_str(&format!(
        "    structural    : carrier_456 / ush2 / snap amplitude invariant: {structural_stable}\n"
    ));
    out.push_str(&format!(
        "  FD sweep         : 1.60 .. 1.90 (step 0.005)  [{} pts]\n",
        fd_pts.len()
    ));
    out.push_str(&format!(
        "    full lock     : void proximity == 0 below {HEALTHY_FD}: {below_lock}\n"
    ));
    out.push_str(&format!(
        "    saturation    : void proximity == 1 at/above {TRIBONACCI_VOID_FD}: {at_limit}\n"
    ));
    out.push_str(&format!(
        "    monotone      : ush2_value non-decreasing across window: {fd_monotone}\n"
    ));
    out.push_str(&format!(
        "  Malignant ref    : FD 1.79 -> void proximity {:.4}, ush2_value {}\n",
        ref_pt.void_proximity, ref_pt.ush2_value
    ));
    out.push_str("+==========================================================+\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apex_synthesizer::ApexSynthesizer;

    #[test]
    fn rmp_gate_flip_is_exact() {
        let pts = sweep_rmp(-30.0, -5.0, 0.5, TumorProfile::malignant());
        for p in &pts {
            assert!(
                (p.gate_deviation_mv - (p.rmp_mv - GATE_MV)).abs() < 1e-9,
                "deviation must be rmp - gate"
            );
        }
        assert!(
            pts.iter()
                .filter(|p| p.rmp_mv <= GATE_MV)
                .all(|p| p.gate_side == GateSide::Below),
            "all points at or below the gate are Below"
        );
        assert!(
            pts.iter()
                .filter(|p| p.rmp_mv > GATE_MV)
                .all(|p| p.gate_side == GateSide::Above),
            "all points strictly above the gate are Above"
        );
    }

    #[test]
    fn rmp_sweep_is_continuous_across_gate() {
        let pts = sweep_rmp(-30.0, -5.0, 0.5, TumorProfile::malignant());
        let max_im_delta = pts
            .windows(2)
            .map(|w| (w[1].im - w[0].im).abs())
            .fold(0.0, f64::max);
        assert!(
            (max_im_delta - 0.5 / MEMBRANE_SPAN_MV).abs() < 1e-12,
            "IM is linear in |deviation| with slope 1/span"
        );
        let at_gate = pts
            .iter()
            .find(|p| (p.rmp_mv - GATE_MV).abs() < 1e-9)
            .unwrap();
        assert_eq!(at_gate.im, 0.0, "IM dips to exactly zero at the gate");
        assert_eq!(
            at_gate.pulses_per_burst, 0,
            "gate pulse is null at the gate"
        );
    }

    #[test]
    fn rmp_does_not_disturb_structural_payload() {
        let pts = sweep_rmp(-30.0, -5.0, 0.5, TumorProfile::malignant());
        let first = &pts[0];
        assert!(
            pts.iter()
                .all(|p| (p.carrier_456_hz - first.carrier_456_hz).abs() < 1e-9),
            "456 Hz carrier is invariant to RMP perturbation"
        );
        assert!(
            pts.iter().all(|p| p.ush2_value == first.ush2_value),
            "entropic-reversion polynomial is invariant to RMP perturbation"
        );
        assert!(
            pts.iter()
                .all(|p| (p.snap_amplitude - first.snap_amplitude).abs() < 1e-9),
            "snap train amplitude is invariant to RMP perturbation"
        );
    }

    #[test]
    fn pulses_per_burst_is_quantized_gate_deviation() {
        let pts = sweep_rmp(-30.0, -5.0, 0.5, TumorProfile::malignant());
        for p in &pts {
            assert_eq!(
                p.pulses_per_burst,
                p.gate_deviation_mv.abs().round() as u32,
                "pulse count is the quantized gate deviation magnitude"
            );
        }
    }

    #[test]
    fn fd_below_phi_lattice_is_fully_locked() {
        let pts = sweep_fractal_dim(1.60, 1.675, 0.005, TumorProfile::malignant());
        assert!(!pts.is_empty());
        assert!(
            pts.iter()
                .all(|p| p.void_proximity == 0.0 && p.lattice_lock == 1.0),
            "healthy-range FD perturbation produces no void proximity"
        );
    }

    #[test]
    fn fd_saturates_at_void_limit() {
        let pts = sweep_fractal_dim(1.84, 1.90, 0.005, TumorProfile::malignant());
        assert!(!pts.is_empty());
        assert!(
            pts.iter()
                .all(|p| p.void_proximity == 1.0 && p.lattice_lock == 0.0),
            "FD at or above the attractor limit saturates the void proximity"
        );
    }

    #[test]
    fn fd_sweep_is_monotone_and_quantized() {
        let pts = sweep_fractal_dim(1.60, 1.90, 0.005, TumorProfile::malignant());
        for w in pts.windows(2) {
            assert!(
                w[1].void_proximity + 1e-12 >= w[0].void_proximity,
                "void proximity monotone non-decreasing"
            );
            assert!(
                w[1].snap_amplitude + 1e-12 >= w[0].snap_amplitude,
                "snap amplitude monotone non-decreasing"
            );
            assert!(
                w[1].ush2_value >= w[0].ush2_value,
                "ush2_value monotone non-decreasing"
            );
        }
    }

    #[test]
    fn malignant_reference_is_reproduced() {
        let payload =
            ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
        let expected_ush2 = (456.0
            * (1.0 + (1.79 - HEALTHY_FD) / (TRIBONACCI_VOID_FD - HEALTHY_FD)))
            .round() as u128
            + 38
            + 41;
        assert_eq!(payload.ush2_value, expected_ush2);
        assert_eq!(payload.ush2_value, 850);
        assert!(
            (payload.snap_pulse.amplitude
                - (1.79 - HEALTHY_FD) / (TRIBONACCI_VOID_FD - HEALTHY_FD))
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn audit_report_is_deterministic() {
        let a = run_sensitivity_audit();
        let b = run_sensitivity_audit();
        assert_eq!(a, b);
        assert!(a.contains("gate flip"));
        assert!(a.contains("continuity"));
    }
}
