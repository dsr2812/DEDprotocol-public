//! End-to-end framework validation: the malignant reference profile through the
//! resonance engine and the Apex compensatory synthesizer.
//!
//! This is an internal-consistency test of the computational model. It feeds
//! the framework's canonical malignant reference profile (RMP −7.0 mV vs. the
//! −23.0 mV grounded reference; fractal dimension 1.79 vs. 1.68; 41%
//! lacunarity; 39 Hz NLTL burst) through the pipeline and verifies the
//! deterministic payload it must produce. It does not assert biological or
//! biological reality.

use ded_ark_engine::{
    ApexDeficit, ApexSynthesizer, DedResonanceEngine, ScaleInvarianceBreakdown, TmeProfile,
    TopologyCalculator, TumorProfile,
};

#[test]
fn test_reference_profile_liver_csc_and_glioblastoma() {
    println!("\n⚕️ INITIATING REFERENCE PROFILE TEST ⚕️");
    println!("Target: Liver Cancer Stem Cell (CSC) + Glioblastoma Morphology");

    // 1. INGEST THE MALIGNANT REFERENCE PROFILE
    // Depolarized membrane: RMP −7.0 mV (grounded reference −23.0 mV).
    // Nuclear fractal dimension 1.79 (phi-lattice reference 1.68).
    // Lacunarity 41%. NLTL bioelectric burst peaking at 39 Hz.
    let malignant_profile = TumorProfile {
        rmp_mv: -7.0,
        fractal_dim: 1.79,
        lacunarity_pct: 41.0,
        nltl_burst_hz: 39.0,
    };

    // 2. CALCULATE THE TOPOLOGICAL DEFICIT (Bipartite Dielectric Bifurcation)
    let topology = TopologyCalculator::new(malignant_profile);

    // PROOF 1: The cell has crossed the Bipartite Dielectric Bifurcation (−19.0 mV).
    // −7.0 is greater than −19.0, placing it in the depolarized "above" regime.
    let gate_dev = topology.gate_deviation();
    assert!(
        gate_dev > 0.0,
        "Cell must be depolarized past the −19 mV gate"
    );
    println!(
        "✅ PROOF 1: Gate Deviation is {:.2} mV. Cell is in the depolarized regime.",
        gate_dev
    );

    // PROOF 2: The cell is deep inside the Maximum Entropy Topological Attractor.
    // 1.79 is close to the 1.839 attractor limit.
    let void_prox = topology.void_proximity();
    assert!(
        void_prox > 0.5,
        "Cell must be deep in the Maximum Entropy Topological Attractor"
    );
    println!(
        "✅ PROOF 2: Void Proximity is {:.2}%. The nucleus is stretched toward the attractor limit.",
        void_prox * 100.0
    );

    // 3. CALCULATE THE RAW RESONANCE VECTORS (ŠÀR-IM-UŠ₂)
    let engine = DedResonanceEngine::new(malignant_profile, 144.0);
    let shar = engine.calculate_shar();
    let im = engine.calculate_im();
    let ush2 = engine.calculate_ush2();

    // PROOF 3: The ŠÀR extraction identifies the 39 Hz NLTL burst for destructive interference.
    assert!(
        (shar - 39.0).abs() < 0.01,
        "ŠÀR must target the 39 Hz NLTL burst"
    );
    println!(
        "✅ PROOF 3: ŠÀR Frequency locked at {:.2} Hz. Destructive interference specified.",
        shar
    );
    println!(
        "   -> IM modulation index: {:.4} | raw UŠ₂ polynomial: {} (= {})",
        im,
        ush2,
        ush2.to_u128().unwrap_or(0)
    );

    // 4. SYNTHESIZE THE APEX COMPENSATORY FIELD (Elephant & Shark anchors)
    let synthesizer = ApexSynthesizer::new();

    // Map the human deficit against the Elephant (TP53 = 1·38) and the Shark
    // (dielectric = 3 + 1·38) topological anchors.
    let elephant_deficit =
        ApexDeficit::from_poly("Elephant TP53", "1·38").expect("canonical elephant polynomial");
    let shark_deficit =
        ApexDeficit::from_poly("Shark Dielectric", "3 + 1·38").expect("canonical shark polynomial");
    assert_eq!(elephant_deficit.value(), 38);
    assert_eq!(shark_deficit.value(), 41);

    let payload = synthesizer.synthesize_compensatory_field(malignant_profile);
    assert_eq!(payload.elephant_deficit, elephant_deficit);
    assert_eq!(payload.shark_deficit, shark_deficit);

    // PROOF 4: The engine generates a non-zero, mathematically valid Base-38 payload
    // with the exact carrier modulations specified by the model.
    assert!(
        payload.ush2_value > 0,
        "Compensatory UŠ₂ must be a positive, non-zero presence"
    );
    println!("✅ PROOF 4: Apex Payload Synthesized.");
    println!(
        "   -> Carrier 1 (144 Hz reference) Modulation: {:.2} Hz  (m144 = {:.4})",
        payload.carrier_144_hz, payload.m144
    );
    println!(
        "   -> Carrier 2 (456 Hz lattice) Modulation: {:.2} Hz  (m456 = {:.4})",
        payload.carrier_456_hz, payload.m456
    );
    println!(
        "   -> Snap Train (Shark Dielectric) Amplitude: {:.2}",
        payload.snap_pulse.amplitude
    );
    println!(
        "   -> Gate Push (Elephant Genomic) Amplitude: {:.2}",
        payload.gate_pulse.amplitude
    );
    println!(
        "   -> Final Compensated UŠ₂ Polynomial: {} (= {})",
        payload.ush2, payload.ush2_value
    );

    assert_eq!(payload.ush2_value, 850);
    assert_eq!(payload.ush2.to_string(), "14 + 22·38");

    println!("\n⚕️ VALIDATION COMPLETE. THE MATH HOLDS. ⚕️");
}

#[test]
fn test_universal_desync_coupling_end_to_end() {
    // Phase-2 ingestion path: a real-world profile (RMP −7.0 mV, FD 1.79,
    // lacunarity 41%, NLTL burst 39 Hz) pushed through the universality-layer
    // desync coupling used by the Desync Bridge pipeline.
    let profile = TumorProfile {
        rmp_mv: -7.0,
        fractal_dim: 1.79,
        lacunarity_pct: 41.0,
        nltl_burst_hz: 39.0,
    };

    let synth = ApexSynthesizer::new();
    let base = synth.synthesize_compensatory_field(profile);

    // Mock local/global scale factors (local proliferation 1.62 vs global
    // vascularization 0.87): a broken scale-invariance → nonzero desync.
    let breakdown = ScaleInvarianceBreakdown::new(1.62, 0.87);
    let desync = breakdown.desync_factor();
    assert!((desync - 0.8621).abs() < 1e-4);

    let coupled = synth.synthesize_with_universal_desync(profile, breakdown);
    assert_eq!(coupled.scale_desync, desync);
    assert!((coupled.m456 - base.m456 * (1.0 + desync)).abs() < 1e-9);
    assert!(coupled.carrier_456_hz > base.carrier_456_hz);
    assert_eq!(coupled.ush2_value, base.ush2_value);
    assert_eq!(coupled.ush2.to_string(), "14 + 22·38");

    // A synchronized microenvironment (local == global) couples to zero and
    // reproduces the canonical, desync-uncoupled payload bit-for-bit.
    let synced =
        synth.synthesize_with_universal_desync(profile, ScaleInvarianceBreakdown::new(1.0, 1.0));
    assert_eq!(synced.scale_desync, 0.0);
    assert_eq!(synced.carrier_456_hz, base.carrier_456_hz);

    println!("✅ UNIVERSAL DESYNC COUPLING: m456 amplified by (1 + {desync:.4}).");
}

#[test]
fn test_tme_matrix_modulates_payload_end_to_end() {
    // Phase-3 ingestion path: the malignant reference profile inside a
    // Warburg-acidic, hypoxic microenvironment (pH 6.8, 8 mmHg).
    let profile = TumorProfile {
        rmp_mv: -7.0,
        fractal_dim: 1.79,
        lacunarity_pct: 41.0,
        nltl_burst_hz: 39.0,
    };
    let tme = TmeProfile {
        ph_level: 6.8,
        hypoxia_mmhg: 8.0,
    };

    let synth = ApexSynthesizer::new();
    let base = synth.synthesize_compensatory_field(profile);
    let coupled = synth.synthesize_compensatory_field_with_tme(profile, tme, 0.0);

    let correction = coupled.tme.as_ref().expect("TME correction present");
    assert!((correction.dielectric_attenuation - 1.0).abs() < 1e-9);
    assert!((correction.immune_evasion_deficit - 1.0).abs() < 1e-9);
    assert_eq!(correction.immune_evasion_polynomial.to_u128(), Some(10_000));

    // Acidic cloak shatters the lactic-acid matrix via a boosted 144 Hz
    // grounding carrier; hypoxia re-targets the ŠÀR channel at HIF-1α.
    assert!(coupled.carrier_144_hz > base.carrier_144_hz);
    assert!(coupled.sar_hz > base.sar_hz);

    println!(
        "✅ TME MATRIX: 144 Hz carrier {:.2} -> {:.2} Hz, ŠÀR {:.2} -> {:.2} Hz.",
        correction.base_carrier_144_hz,
        correction.boosted_carrier_144_hz,
        correction.base_sar_hz,
        correction.sar_hif_target_hz
    );
}
