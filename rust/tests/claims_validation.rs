//! Claims-level validation for the technical report draft's core statements.
//!
//! These tests lock the boundaries the paper actually relies on:
//! - the zero-free Base-38 round trip at the canonical sample points;
//! - the exact −19 mV gate and the 1.68 → 1.839 fractal clamp;
//! - the Apex payload invariants (positive carriers, bounded duties/amplitudes,
//!   elephant + shark compensation);
//! - determinism;
//! - CSV rejection of missing/malformed data;
//! - the CLI end-to-end on the reference candidates file.

use ded_ark_engine::{
    parse_candidates_csv, ApexSynthesizer, GateSide, TopologyCalculator, TumorProfile, ZeroFree38,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn temp_csv(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("ded_claims_{}_{}.csv", std::process::id(), name));
    fs::write(&path, body).expect("write temp csv");
    path
}

#[test]
fn base38_roundtrip_boundaries() {
    let values = [
        0u128,
        1,
        2,
        37,
        38,
        39,
        75,
        76,
        77,
        144,
        1443,
        1444,
        1445,
        1_000_000,
        3_010_936_384,
    ];
    for v in values {
        let z = ZeroFree38::from_u128(v);
        assert_eq!(z.to_u128(), Some(v), "round-trip failed for {v}");
    }
}

#[test]
fn gate_threshold_is_exact() {
    let mut p = TumorProfile::malignant();

    p.rmp_mv = -19.0;
    let t = TopologyCalculator::new(p);
    assert_eq!(t.gate_deviation(), 0.0);
    assert_eq!(t.gate_side(), GateSide::Below);

    p.rmp_mv = -18.9;
    assert_eq!(TopologyCalculator::new(p).gate_side(), GateSide::Above);

    p.rmp_mv = -19.1;
    assert_eq!(TopologyCalculator::new(p).gate_side(), GateSide::Below);
}

#[test]
fn void_proximity_clamps_at_the_documented_limits() {
    let mut p = TumorProfile::malignant();

    p.fractal_dim = 1.68;
    assert!(TopologyCalculator::new(p).void_proximity().abs() < 1e-12);

    p.fractal_dim = 1.839;
    assert!((TopologyCalculator::new(p).void_proximity() - 1.0).abs() < 1e-12);

    p.fractal_dim = 1.60;
    assert_eq!(TopologyCalculator::new(p).void_proximity(), 0.0);

    p.fractal_dim = 1.90;
    assert_eq!(TopologyCalculator::new(p).void_proximity(), 1.0);

    p.fractal_dim = 1.75;
    let vp = TopologyCalculator::new(p).void_proximity();
    assert!(
        vp > 0.0 && vp < 1.0,
        "expected interior proximity, got {vp}"
    );
}

#[test]
fn apex_payload_invariants_hold() {
    let payload = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());

    assert!(payload.carrier_144_hz > 0.0);
    assert!(payload.carrier_456_hz > 0.0);
    assert!(payload.sar_hz > 0.0);
    assert!(payload.im_hz > 0.0);

    for (name, train) in [("snap", &payload.snap_pulse), ("gate", &payload.gate_pulse)] {
        assert!(
            (0.0..=1.0).contains(&train.duty_cycle),
            "{name} duty out of range: {}",
            train.duty_cycle
        );
        assert!(
            (0.0..=1.0).contains(&train.amplitude),
            "{name} amplitude out of range: {}",
            train.amplitude
        );
        assert!(
            train.frequency_hz > 0.0,
            "{name} frequency must be positive"
        );
        assert!(
            train.pulses_per_burst > 0,
            "{name} needs at least one pulse"
        );
    }

    assert_eq!(payload.elephant_deficit.value(), 38);
    assert_eq!(payload.shark_deficit.value(), 41);
    assert_eq!(
        payload.ush2_value,
        payload.tumor_ush2_value + 38 + 41,
        "compensated UŠ₂ must equal tumor + elephant + shark"
    );
}

#[test]
fn apex_payload_is_deterministic() {
    let a = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
    let b = ApexSynthesizer::new().synthesize_compensatory_field(TumorProfile::malignant());
    assert_eq!(format!("{a}"), format!("{b}"));
    assert_eq!(a.ush2_value, b.ush2_value);
}

#[test]
fn csv_parser_accepts_the_reference_shape() {
    let path = temp_csv(
        "good",
        "organism,stage,continuous_score,target_frequency_hz\n\
         Demo organism A,Stage 1,0.29,39.0\n",
    );
    let rows = parse_candidates_csv(&path).expect("reference-shaped CSV parses");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].source_freq_hz, 39.0);
    let _ = fs::remove_file(path);
}

#[test]
fn csv_parser_rejects_missing_column() {
    let path = temp_csv(
        "missing",
        "organism,stage,continuous_score\nDemo organism A,Stage 1,0.29\n",
    );
    let err = parse_candidates_csv(&path).expect_err("missing column must fail");
    assert!(
        err.contains("target_frequency_hz"),
        "error should name the missing column: {err}"
    );
    let _ = fs::remove_file(path);
}

#[test]
fn csv_parser_rejects_malformed_number() {
    let path = temp_csv(
        "bad",
        "organism,stage,continuous_score,target_frequency_hz\n\
         Demo organism A,Stage 1,not-a-number,39.0\n",
    );
    assert!(parse_candidates_csv(&path).is_err());
    let _ = fs::remove_file(path);
}

#[test]
fn cli_runs_on_the_reference_candidates_file() {
    let csv = Path::new(env!("CARGO_MANIFEST_DIR")).join("../test_data/symbiosis_candidates.csv");
    assert!(csv.exists(), "reference CSV not found at {}", csv.display());

    let out = Command::new(env!("CARGO_BIN_EXE_ded_ark_cli"))
        .arg(&csv)
        .output()
        .expect("run ded_ark_cli");

    assert!(
        out.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("US2 vector"),
        "CLI output did not carry the UŠ₂ vector: {stdout}"
    );
}
