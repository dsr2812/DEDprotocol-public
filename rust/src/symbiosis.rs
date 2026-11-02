//! DED symbiosis inverter — maps a candidate's dissonance signature to a
//! phase-locked reference frequency, expressed as a zero-free Base-38
//! polynomial. Mirrors the audit CSV's candidate shape.
//! Theoretical computational model; asserts no physical or biological reality.

use crate::core::ZeroFree38;
use crate::ded_engine::{
    DedResonanceEngine, GateSide, TumorProfile, GATE_MV, HEALTHY_FD, MEMBRANE_SPAN_MV,
    REFERENCE_144_HZ, TRIBONACCI_VOID_FD,
};
use std::path::Path;

/// A candidate stage surfaced by the audit CSV, ready for inversion.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbiosisCandidate {
    /// Stage identifier (mirrors the audit CSV).
    pub id: String,
    /// Dissonance index (0 = phase-locked, 1 = full void).
    pub dissonance_score: f64,
    /// Source emission frequency (model input).
    pub source_freq_hz: f64,
    /// Coupling factor to the reference lattice (model input).
    pub coupling_factor: f64,
}

/// Maps candidate dissonance to a reference polynomial.
#[derive(Debug, Clone, Copy)]
pub struct SymbiosisInverter {
    master_clock_hz: f64,
}

/// One inverted candidate: beacon frequency plus its Base-38 vector.
#[derive(Debug, Clone)]
pub struct InversionReport {
    pub candidate: SymbiosisCandidate,
    pub beacon_hz: f64,
    pub vector: ZeroFree38,
}

impl SymbiosisInverter {
    pub fn new(master_clock_hz: f64) -> Self {
        assert!(master_clock_hz > 0.0, "master clock must be positive");
        Self { master_clock_hz }
    }

    /// Inverter bound to the canonical 144 Hz reference.
    pub fn with_master_clock() -> Self {
        Self::new(REFERENCE_144_HZ)
    }

    pub fn master_clock_hz(&self) -> f64 {
        self.master_clock_hz
    }

    /// Nearest 144 Hz reference harmonic above the source emission
    /// (`n = round(f / clock)`, clamped to n >= 1).
    pub fn anchor_harmonic(&self, freq_hz: f64) -> f64 {
        let n = (freq_hz / self.master_clock_hz).round().max(1.0);
        n * self.master_clock_hz
    }

    /// Mirror of the source frequency across its reference anchor:
    /// `2 * anchor - f`. A candidate locked exactly on the anchor maps to
    /// itself.
    pub fn mirrored_freq(&self, freq_hz: f64) -> f64 {
        2.0 * self.anchor_harmonic(freq_hz) - freq_hz
    }

    /// Synthetic `TumorProfile` induced by the candidate: dissonance maps the
    /// membrane from the 19/19 gate toward depolarization and the geometry
    /// from the healthy reference toward the Tribonacci limit.
    fn profile_for(&self, candidate: &SymbiosisCandidate) -> TumorProfile {
        let d = candidate.dissonance_score.clamp(0.0, 1.0);
        TumorProfile {
            rmp_mv: GATE_MV + d * MEMBRANE_SPAN_MV,
            fractal_dim: HEALTHY_FD + d * (TRIBONACCI_VOID_FD - HEALTHY_FD),
            lacunarity_pct: candidate.dissonance_score * 141.0,
            nltl_burst_hz: candidate.source_freq_hz,
        }
    }

    /// The reference frequency. The source emission is mirrored across the
    /// nearest 144 Hz reference harmonic, then mapped through the 19/19 gate:
    /// the inversion amplitude is gated by the membrane deviation (gate factor
    /// `1 + dissonance`) and scaled by coupling (`2 - coupling`).
    pub fn beacon_hz(&self, candidate: &SymbiosisCandidate) -> f64 {
        let engine = DedResonanceEngine::with_master_clock(self.profile_for(candidate));
        let gate_factor = 1.0 + engine.topology().gate_deviation().abs() / MEMBRANE_SPAN_MV;
        let coupling_gain = 2.0 - candidate.coupling_factor.clamp(0.0, 2.0);

        let anchor = self.anchor_harmonic(candidate.source_freq_hz);
        let mirrored = 2.0 * anchor - candidate.source_freq_hz;
        let beacon = anchor + (mirrored - anchor) * gate_factor * coupling_gain;

        beacon.max(REFERENCE_144_HZ)
    }

    /// Base-38 polynomial of the reference frequency, canonicalized by the
    /// ZeroFree38 normalize engine: digits in `1..=37`, ascending places,
    /// every coefficient resolving to a perfect multiple of 38 collapsed to
    /// an implied zero (omitted from the sparse vector).
    pub fn invert_polarity(&self, candidate: &SymbiosisCandidate) -> ZeroFree38 {
        let beacon = self.beacon_hz(candidate).round().max(REFERENCE_144_HZ);
        ZeroFree38::from_u128(beacon as u128)
    }

    /// Full report for a single candidate.
    pub fn invert(&self, candidate: &SymbiosisCandidate) -> InversionReport {
        InversionReport {
            candidate: candidate.clone(),
            beacon_hz: self.beacon_hz(candidate),
            vector: self.invert_polarity(candidate),
        }
    }
}

/// Synthetic demo candidates for the built-in CLI audit. They are not
/// biological data.
pub fn mock_candidates() -> Vec<SymbiosisCandidate> {
    vec![
        SymbiosisCandidate {
            id: "Demo organism A / Stage 1".to_string(),
            dissonance_score: 0.58,
            source_freq_hz: 144.0,
            coupling_factor: 0.42,
        },
        SymbiosisCandidate {
            id: "Demo organism B / Stage 2".to_string(),
            dissonance_score: 0.33,
            source_freq_hz: 456.0,
            coupling_factor: 0.67,
        },
        SymbiosisCandidate {
            id: "Demo organism C / Stage 3".to_string(),
            dissonance_score: 0.69,
            source_freq_hz: 39.0,
            coupling_factor: 0.31,
        },
    ]
}

fn fmt_freq(hz: f64) -> String {
    if hz >= 1.0e6 {
        format!("{hz:.3e} Hz")
    } else if hz >= 1000.0 {
        format!("{hz:.1} Hz")
    } else {
        format!("{hz:.2} Hz")
    }
}

/// Parse the Python audit's `symbiosis_candidates.csv` into candidates.
///
/// Expected columns: `organism`, `stage`, `continuous_score`,
/// `target_frequency_hz`. The dissonance score is derived as
/// `1 - continuous_score` and the continuous score doubles as the host
/// coupling factor. Missing or malformed numeric cells are rejected.
pub fn parse_candidates_csv(path: &Path) -> Result<Vec<SymbiosisCandidate>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| format!("failed to open {}: {e}", path.display()))?;

    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let col = |name: &str| -> Result<usize, String> {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("column '{name}' missing in {}", path.display()))
    };
    let org = col("organism")?;
    let stage = col("stage")?;
    let continuous = col("continuous_score")?;
    let freq = col("target_frequency_hz")?;

    let mut out = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let parse = |idx: usize| -> Result<f64, String> {
            record
                .get(idx)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("bad numeric cell in record {:?}", record))
        };
        let continuous: f64 = parse(continuous)?;
        let freq_hz: f64 = parse(freq)?;
        out.push(SymbiosisCandidate {
            id: format!(
                "{} / {}",
                record.get(org).unwrap_or_default(),
                record.get(stage).unwrap_or_default()
            ),
            dissonance_score: (1.0 - continuous).clamp(0.0, 1.0),
            source_freq_hz: freq_hz,
            coupling_factor: continuous,
        });
    }
    Ok(out)
}

/// Run the inversion audit on an arbitrary candidate set and print the boxed
/// terminal telemetry. Returns the per-candidate reports.
pub fn run_inversion_audit_for(candidates: &[SymbiosisCandidate]) -> Vec<InversionReport> {
    let inverter = SymbiosisInverter::with_master_clock();
    let reports: Vec<InversionReport> = candidates.iter().map(|c| inverter.invert(c)).collect();

    println!("+============================================================+");
    println!("|  DED SYMBIOSIS INVERSION — SAR→US2 POLARITY FLIP          |");
    println!("+============================================================+");
    for (i, r) in reports.iter().enumerate() {
        let engine = DedResonanceEngine::with_master_clock(inverter.profile_for(&r.candidate));
        let dev = engine.topology().gate_deviation();
        let side = match engine.topology().gate_side() {
            GateSide::Above => "ABOVE",
            GateSide::Below => "BELOW",
        };
        let anchor = inverter.anchor_harmonic(r.candidate.source_freq_hz);
        let n = anchor / inverter.master_clock_hz();
        let mirror = inverter.mirrored_freq(r.candidate.source_freq_hz);
        println!();
        println!("  [{}/{}] {}", i + 1, reports.len(), r.candidate.id);
        println!(
            "    dissonance : {:.4} | source   : {} | coupling : {:.2}",
            r.candidate.dissonance_score,
            fmt_freq(r.candidate.source_freq_hz),
            r.candidate.coupling_factor
        );
        println!(
            "    lattice    : {} (n={n:.0}) | mirror : {}",
            fmt_freq(anchor),
            fmt_freq(mirror)
        );
        println!("    gate       : {side} 19/19 ({dev:+.2} mV deviation)");
        println!("    beacon     : {}", fmt_freq(r.beacon_hz));
        println!(
            "    US2 vector : {}   (= {})",
            r.vector,
            r.vector.to_u128().unwrap_or(0)
        );
    }
    println!();
    println!("+============================================================+");
    println!(
        "  {} candidate(s) phase-inverted through the 19/19 gate.",
        reports.len()
    );
    println!("+============================================================+");
    reports
}

/// Run the inversion audit on the three mock candidates and print the
/// Base-38 inversion vectors to the terminal. Returns the reports.
pub fn run_inversion_audit() -> Vec<InversionReport> {
    let mocks = mock_candidates();
    run_inversion_audit_for(&mocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Digit;

    fn term(place: u32, digit: u8) -> crate::core::Term {
        crate::core::Term {
            place,
            digit: unsafe { Digit::new_unchecked(digit) },
        }
    }

    fn assert_canonical(poly: &ZeroFree38) {
        let terms = poly.terms();
        assert!(!terms.is_empty());
        for w in terms.windows(2) {
            assert!(w[0].place < w[1].place, "places must be ascending");
        }
        for t in terms {
            assert!(
                t.digit.get() >= 1 && t.digit.get() <= 37,
                "digit out of range"
            );
        }
    }

    #[test]
    fn lattice_anchoring() {
        let inv = SymbiosisInverter::with_master_clock();
        assert_eq!(inv.anchor_harmonic(39.0), 144.0);
        assert_eq!(inv.anchor_harmonic(270.0), 288.0);
        assert_eq!(inv.anchor_harmonic(288.0), 288.0);
        assert_eq!(inv.mirrored_freq(39.0), 249.0);
        assert_eq!(inv.mirrored_freq(288.0), 288.0);
    }

    #[test]
    fn first_demo_inversion() {
        let inv = SymbiosisInverter::with_master_clock();
        let c = mock_candidates()[0].clone();
        let beacon = inv.beacon_hz(&c);
        assert!((beacon - 144.0).abs() < 1e-6, "beacon was {beacon}");

        let poly = inv.invert_polarity(&c);
        assert_eq!(poly.terms(), &[term(0, 30), term(1, 3)]);
        assert_eq!(poly.to_string(), "30 + 3·38");
        assert_eq!(poly.to_u128(), Some(144));
        assert_canonical(&poly);
    }

    #[test]
    fn second_demo_inversion() {
        let inv = SymbiosisInverter::with_master_clock();
        let c = mock_candidates()[1].clone();
        let beacon = inv.beacon_hz(&c);
        assert!((beacon - 389.5464).abs() < 1e-4, "beacon was {beacon}");

        let poly = inv.invert_polarity(&c);
        assert_eq!(poly.terms(), &[term(0, 10), term(1, 10)]);
        assert_eq!(poly.to_string(), "10 + 10·38");
        assert_eq!(poly.to_u128(), Some(390));
        assert_canonical(&poly);
    }

    #[test]
    fn third_demo_inversion() {
        let inv = SymbiosisInverter::with_master_clock();
        let c = mock_candidates()[2].clone();
        let beacon = inv.beacon_hz(&c);
        assert!((beacon - 443.8905).abs() < 1e-4, "beacon was {beacon}");

        let poly = inv.invert_polarity(&c);
        assert_eq!(poly.terms(), &[term(0, 26), term(1, 11)]);
        assert_eq!(poly.to_string(), "26 + 11·38");
        assert_eq!(poly.to_u128(), Some(444));
        assert_canonical(&poly);
    }

    #[test]
    fn perfect_lock_maps_to_anchor() {
        let inv = SymbiosisInverter::with_master_clock();
        let c = SymbiosisCandidate {
            id: "locked".to_string(),
            dissonance_score: 0.0,
            source_freq_hz: 288.0,
            coupling_factor: 1.0,
        };
        assert_eq!(inv.beacon_hz(&c), 288.0);
        assert_eq!(inv.invert_polarity(&c).to_u128(), Some(288));
    }

    #[test]
    fn audit_reports_all_three() {
        let reports = run_inversion_audit();
        assert_eq!(reports.len(), 3);
        for r in &reports {
            assert_eq!(
                r.vector.to_u128(),
                Some(r.beacon_hz.round() as u128),
                "vector must match beacon for {}",
                r.candidate.id
            );
        }
    }

    #[test]
    fn parses_audit_csv() {
        let mut dir = std::env::temp_dir();
        dir.push("ded_symbiosis_test.csv");
        std::fs::write(
            &dir,
            "organism,stage,continuous_score,grounding_score,triadic_state,action,target_frequency_hz\n\
             Demo organism A,Stage 1,0.6646,0.9989,\"IM\",Apply,144.0\n\
             Demo organism B,Stage 2,0.7774,0.999,\"IM\",Apply,288.0\n",
        )
        .unwrap();

        let candidates = parse_candidates_csv(&dir).unwrap();
        std::fs::remove_file(&dir).ok();
        assert_eq!(candidates.len(), 2);

        let first = &candidates[0];
        assert_eq!(first.id, "Demo organism A / Stage 1");
        assert!((first.dissonance_score - 0.3354).abs() < 1e-4);
        assert!((first.source_freq_hz - 144.0).abs() < 1e-9);
        assert!((first.coupling_factor - 0.6646).abs() < 1e-4);

        assert_eq!(candidates[1].id, "Demo organism B / Stage 2");
        assert!((candidates[1].source_freq_hz - 288.0).abs() < 1e-9);
    }

    #[test]
    fn csv_missing_column_errors() {
        let mut dir = std::env::temp_dir();
        dir.push("ded_symbiosis_bad.csv");
        std::fs::write(&dir, "organism,stage\nA,B\n").unwrap();
        let err = parse_candidates_csv(&dir).unwrap_err();
        std::fs::remove_file(&dir).ok();
        assert!(err.contains("continuous_score"));
    }
}
