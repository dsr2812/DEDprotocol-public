pub mod apex_synthesizer;
pub mod core;
pub mod ded_engine;
pub mod interferential_modulation;
pub mod sensitivity;
pub mod symbiosis;
pub mod universality;

pub use apex_synthesizer::*;
pub use core::*;
pub use ded_engine::*;
pub use interferential_modulation::*;
pub use sensitivity::*;
pub use symbiosis::*;
pub use universality::*;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
/// Exposes the symbiosis inverter to the Python data pipeline.
#[pyclass(name = "SymbiosisInverter", module = "ded_ark_engine")]
#[derive(Clone)]
pub struct PySymbiosisInverter {
    inner: SymbiosisInverter,
}

/// One inverted row as surfaced to Python:
/// `(candidate_id, raw_beacon_u128, base38_polynomial_string)`.
pub type CsvRow = (String, String, String);

fn invert_candidate(inverter: &SymbiosisInverter, candidate: &SymbiosisCandidate) -> CsvRow {
    let beacon = inverter.beacon_hz(candidate).round();
    let vector = inverter.invert_polarity(candidate);
    (
        candidate.id.clone(),
        (beacon as u128).to_string(),
        vector.to_string(),
    )
}

fn process_candidates_file(inverter: &SymbiosisInverter, csv_path: &str) -> PyResult<Vec<CsvRow>> {
    let candidates = parse_candidates_csv(Path::new(csv_path)).map_err(PyValueError::new_err)?;
    Ok(candidates
        .iter()
        .map(|c| invert_candidate(inverter, c))
        .collect())
}

#[pymethods]
impl PySymbiosisInverter {
    #[new]
    fn new() -> Self {
        Self {
            inner: SymbiosisInverter::with_master_clock(),
        }
    }

    /// Invert a single candidate supplied inline.
    /// Returns `(candidate_id, raw_beacon_u128, base38_polynomial_string)`.
    #[pyo3(signature = (id, dissonance_score, source_freq_hz, coupling_factor))]
    fn invert(
        &self,
        id: String,
        dissonance_score: f64,
        source_freq_hz: f64,
        coupling_factor: f64,
    ) -> (String, u128, String) {
        let candidate = SymbiosisCandidate {
            id,
            dissonance_score,
            source_freq_hz,
            coupling_factor,
        };
        let beacon = self.inner.beacon_hz(&candidate).round();
        let vector = self.inner.invert_polarity(&candidate);
        (candidate.id, beacon as u128, vector.to_string())
    }

    /// Read the Python audit's `symbiosis_candidates.csv`, run every row
    /// through `invert_polarity()`, and return
    /// `(candidate_id, raw_beacon_u128, base38_polynomial_string)` rows.
    fn process_csv(&self, csv_path: String) -> PyResult<Vec<CsvRow>> {
        process_candidates_file(&self.inner, &csv_path)
    }
}

/// Module-level entrypoint: `ded_ark_engine.process_csv("path/to.csv")`.
#[pyfunction]
fn process_csv(csv_path: String) -> PyResult<Vec<CsvRow>> {
    process_candidates_file(&SymbiosisInverter::with_master_clock(), &csv_path)
}

/// Encode a non-negative integer as a canonical zero-free Base-38 polynomial
/// via the normalize engine (implied zeros omitted).
#[pyfunction]
fn to_base38(value: u128) -> String {
    ZeroFree38::from_u128(value).to_string()
}

/// Desynchronization coupling between a mock local and global scale factor,
/// `|local − global| / global` clamped to `0.0..=1.0`. Mirrors
/// [`ScaleInvarianceBreakdown::desync_factor`] so the Python pipeline derives
/// the same clamped value the engine will apply to the 456 Hz carrier.
#[pyfunction]
fn desync_factor(local_scale: f64, global_scale: f64) -> f64 {
    ScaleInvarianceBreakdown::new(local_scale, global_scale).desync_factor()
}
/// Exposes the Apex Compensatory Matrix to the Python data pipeline.
#[pyclass(name = "ApexSynthesizer", module = "ded_ark_engine")]
#[derive(Clone)]
pub struct PyApexSynthesizer {
    inner: ApexSynthesizer,
}

/// Serialize a [`CompensatoryPayload`] into the dictionary consumed by the
/// Python data pipeline.
fn payload_to_dict<'py>(
    py: Python<'py>,
    payload: CompensatoryPayload,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("rmp_mv", payload.tumor.rmp_mv)?;
    dict.set_item("gate_deviation_mv", payload.gate_deviation_mv)?;
    dict.set_item("gate_side", format!("{:?}", payload.gate_side))?;
    dict.set_item("void_proximity", payload.void_proximity)?;
    dict.set_item("lattice_lock", payload.lattice_lock)?;
    dict.set_item("im", payload.im)?;
    dict.set_item("m144", payload.m144)?;
    dict.set_item("m456", payload.m456)?;
    dict.set_item("carrier_144_hz", payload.carrier_144_hz)?;
    dict.set_item("carrier_456_hz", payload.carrier_456_hz)?;
    dict.set_item("scale_desync", payload.scale_desync)?;
    dict.set_item("elephant_ratchet_hz", payload.elephant_ratchet_hz)?;
    dict.set_item("shark_mptp_hz", payload.shark_mptp_hz)?;
    dict.set_item("snap_frequency_hz", payload.snap_pulse.frequency_hz)?;
    dict.set_item("snap_duty_cycle", payload.snap_pulse.duty_cycle)?;
    dict.set_item("snap_pulses", payload.snap_pulse.pulses_per_burst)?;
    dict.set_item("snap_amplitude", payload.snap_pulse.amplitude)?;
    dict.set_item("gate_frequency_hz", payload.gate_pulse.frequency_hz)?;
    dict.set_item("gate_duty_cycle", payload.gate_pulse.duty_cycle)?;
    dict.set_item("gate_pulses", payload.gate_pulse.pulses_per_burst)?;
    dict.set_item("gate_amplitude", payload.gate_pulse.amplitude)?;
    dict.set_item("sar_hz", payload.sar_hz)?;
    dict.set_item("im_hz", payload.im_hz)?;
    dict.set_item("tumor_ush2", payload.tumor_ush2.to_string())?;
    dict.set_item("tumor_ush2_value", payload.tumor_ush2_value)?;
    dict.set_item("ush2", payload.ush2.to_string())?;
    dict.set_item("ush2_value", payload.ush2_value)?;
    if let Some(tme) = &payload.tme {
        dict.set_item("tme_active", true)?;
        dict.set_item("tme_dielectric_attenuation", tme.dielectric_attenuation)?;
        dict.set_item("tme_base_carrier_144_hz", tme.base_carrier_144_hz)?;
        dict.set_item("tme_boosted_carrier_144_hz", tme.boosted_carrier_144_hz)?;
        dict.set_item("tme_immune_evasion_deficit", tme.immune_evasion_deficit)?;
        dict.set_item(
            "tme_immune_evasion_polynomial",
            tme.immune_evasion_polynomial.to_string(),
        )?;
        dict.set_item("tme_sar_hif_target_hz", tme.sar_hif_target_hz)?;
    } else {
        dict.set_item("tme_active", false)?;
    }
    Ok(dict)
}

#[pymethods]
impl PyApexSynthesizer {
    #[new]
    fn new() -> Self {
        Self {
            inner: ApexSynthesizer::new(),
        }
    }

    /// Parse a canonical Base-38 deficit polynomial (e.g. `"3 + 1·38"`) into
    /// its integer value.
    #[staticmethod]
    fn parse_deficit(poly: String) -> PyResult<u128> {
        Ok(ApexDeficit::from_poly("deficit", &poly)
            .map_err(PyValueError::new_err)?
            .value())
    }

    /// Synthesize the canonical (desync-uncoupled) compensatory field for a
    /// tumor profile and return the DED ŠÀR-IM-UŠ₂ payload as a dictionary.
    #[pyo3(signature = (rmp_mv, fractal_dim, lacunarity_pct, nltl_burst_hz))]
    fn synthesize<'py>(
        &self,
        py: Python<'py>,
        rmp_mv: f64,
        fractal_dim: f64,
        lacunarity_pct: f64,
        nltl_burst_hz: f64,
    ) -> PyResult<Bound<'py, PyDict>> {
        let profile = TumorProfile {
            rmp_mv,
            fractal_dim,
            lacunarity_pct,
            nltl_burst_hz,
        };
        let payload = self.inner.synthesize_compensatory_field(profile);
        payload_to_dict(py, payload)
    }

    /// Synthesize the compensatory field with explicit universality-layer
    /// desynchronization coupling. `desync` (clamped to `0.0..=1.0` in the
    /// engine) amplifies the modulation applied to the 456 Hz UŠ₂ carrier
    /// amplitude: `m456_eff = m456_base · (1 + desync)`.
    #[pyo3(signature = (rmp_mv, fractal_dim, lacunarity_pct, nltl_burst_hz, desync))]
    fn synthesize_with_desync<'py>(
        &self,
        py: Python<'py>,
        rmp_mv: f64,
        fractal_dim: f64,
        lacunarity_pct: f64,
        nltl_burst_hz: f64,
        desync: f64,
    ) -> PyResult<Bound<'py, PyDict>> {
        let profile = TumorProfile {
            rmp_mv,
            fractal_dim,
            lacunarity_pct,
            nltl_burst_hz,
        };
        let payload = self
            .inner
            .synthesize_compensatory_field_with_desync(profile, desync);
        payload_to_dict(py, payload)
    }

    /// Synthesize the compensatory field with the universality coupling read
    /// directly from a mock local/global scale-invariance discrepancy: a
    /// `ScaleInvarianceBreakdown` is built from `(local_scale, global_scale)`
    /// and its clamped `desync_factor` is fed into the 456 Hz UŠ₂ carrier
    /// amplitude. This is the engine's `synthesize_with_universal_desync` path.
    #[pyo3(signature = (rmp_mv, fractal_dim, lacunarity_pct, nltl_burst_hz, local_scale, global_scale))]
    #[allow(clippy::too_many_arguments)]
    fn synthesize_with_universal_desync<'py>(
        &self,
        py: Python<'py>,
        rmp_mv: f64,
        fractal_dim: f64,
        lacunarity_pct: f64,
        nltl_burst_hz: f64,
        local_scale: f64,
        global_scale: f64,
    ) -> PyResult<Bound<'py, PyDict>> {
        let profile = TumorProfile {
            rmp_mv,
            fractal_dim,
            lacunarity_pct,
            nltl_burst_hz,
        };
        let breakdown = ScaleInvarianceBreakdown::new(local_scale, global_scale);
        let payload = self
            .inner
            .synthesize_with_universal_desync(profile, breakdown);
        payload_to_dict(py, payload)
    }

    /// Synthesize the TME-coupled payload for a single profile: the universality
    /// desync (from `local_scale`/`global_scale`), the acidic-cloak 144 Hz
    /// amplitude boost (from `ph_level`), and the hypoxic ŠÀR modulation (from
    /// `hypoxia_mmhg`) are all composed in one pass.
    #[pyo3(signature = (rmp_mv, fractal_dim, lacunarity_pct, nltl_burst_hz, ph_level, hypoxia_mmhg, local_scale, global_scale))]
    #[allow(clippy::too_many_arguments)]
    fn synthesize_with_tme<'py>(
        &self,
        py: Python<'py>,
        rmp_mv: f64,
        fractal_dim: f64,
        lacunarity_pct: f64,
        nltl_burst_hz: f64,
        ph_level: f64,
        hypoxia_mmhg: f64,
        local_scale: f64,
        global_scale: f64,
    ) -> PyResult<Bound<'py, PyDict>> {
        let profile = TumorProfile {
            rmp_mv,
            fractal_dim,
            lacunarity_pct,
            nltl_burst_hz,
        };
        let tme = TmeProfile {
            ph_level,
            hypoxia_mmhg,
        };
        let desync = ScaleInvarianceBreakdown::new(local_scale, global_scale).desync_factor();
        let payload = self
            .inner
            .synthesize_compensatory_field_with_tme(profile, tme, desync);
        payload_to_dict(py, payload)
    }
}

/// Module-level entrypoint: `ded_ark_engine.run_sensitivity_audit()`.
#[pyfunction(name = "run_sensitivity_audit")]
fn py_run_sensitivity_audit() -> String {
    sensitivity::run_sensitivity_audit()
}

/// The PyO3 extension module. Importable in Python as `ded_ark_engine`.
#[pymodule]
fn ded_ark_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySymbiosisInverter>()?;
    m.add_class::<PyApexSynthesizer>()?;
    m.add_function(wrap_pyfunction!(process_csv, m)?)?;
    m.add_function(wrap_pyfunction!(to_base38, m)?)?;
    m.add_function(wrap_pyfunction!(desync_factor, m)?)?;
    m.add_function(wrap_pyfunction!(py_run_sensitivity_audit, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
