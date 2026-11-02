#!/usr/bin/env python3
"""Iron Bridge — load the compiled Rust engine into Python.

Builds the Rust crate if needed, loads the PyO3 extension module, runs the
synthetic candidate CSV through the Base-38 inversion, and prints a reference
payload. Theoretical computational model; no clinical claim.

Run from anywhere; the script locates the repo root itself.
"""

import importlib.machinery
import importlib.util
import os
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
RUST_DIR = ROOT / "rust"
TARGET = RUST_DIR / "target" / "release"
CSV = ROOT / "test_data" / "symbiosis_candidates.csv"
MODULE_NAME = "ded_ark_engine"
PY_STAMP = TARGET / ".pyo3-python-version"

SOURCES = [
    RUST_DIR / "Cargo.toml",
    RUST_DIR / "Cargo.lock",
    RUST_DIR / "src" / "lib.rs",
    RUST_DIR / "src" / "main.rs",
    RUST_DIR / "src" / "core.rs",
    RUST_DIR / "src" / "ded_engine.rs",
    RUST_DIR / "src" / "symbiosis.rs",
    RUST_DIR / "src" / "apex_synthesizer.rs",
    RUST_DIR / "src" / "universality.rs",
    RUST_DIR / "src" / "sensitivity.rs",
    RUST_DIR / "src" / "interferential_modulation.rs",
]


def _so_candidates():
    stem = MODULE_NAME
    for suffix in (".so", ".dylib", ".pyd"):
        yield TARGET / f"lib{stem}{suffix}"
        yield TARGET / f"{stem}{suffix}"


def _find_so() -> pathlib.Path:
    for path in _so_candidates():
        if path.exists():
            return path
    # Linux default, used only for the error message before a build.
    return TARGET / f"lib{MODULE_NAME}.so"


def _stale() -> bool:
    so = _find_so()
    if not so.exists():
        return True
    try:
        built_with = PY_STAMP.read_text(encoding="utf-8").strip()
    except OSError:
        return True
    if built_with != sys.version.split()[0]:
        return True
    so_mtime = so.stat().st_mtime
    return any(p.stat().st_mtime > so_mtime for p in SOURCES)


def ensure_built() -> pathlib.Path:
    """Compile the extension module against the running interpreter if stale."""
    env = dict(os.environ)
    env["PYO3_PYTHON"] = sys.executable
    if sys.version_info >= (3, 14):
        env.setdefault("PYO3_USE_ABI3_FORWARD_COMPATIBILITY", "1")
    if _stale():
        subprocess.run(
            ["cargo", "build", "--release"],
            cwd=RUST_DIR,
            env=env,
            check=True,
        )
        TARGET.mkdir(parents=True, exist_ok=True)
        PY_STAMP.write_text(sys.version.split()[0] + "\n", encoding="utf-8")
    return _find_so()


def load_engine():
    so = ensure_built()
    spec = importlib.util.spec_from_file_location(MODULE_NAME, so)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not build import spec for {so}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> int:
    engine = load_engine()
    print(f"iron bridge: {MODULE_NAME} v{engine.__version__} loaded")

    if not CSV.exists():
        print(f"iron bridge: demo CSV missing — {CSV}", file=sys.stderr)
        return 1

    rows = engine.process_csv(str(CSV))
    print("+============================================================+")
    print("|  PYTHON IRON BRIDGE — Base-38 Inversion Vectors           |")
    print(f"|  source: {CSV.name}")
    print("+============================================================+")
    for candidate_id, beacon, vector in rows:
        print()
        print(f"  {candidate_id}")
        print(f"    raw beacon : {beacon}")
        print(f"    US2 vector : {vector}")
    print()
    print("+============================================================+")
    print(f"  {len(rows)} vector(s) extracted from the synthetic demo.")
    print("+============================================================+")

    print()
    print("+============================================================+")
    print("|  PYTHON IRON BRIDGE — Sensitivity & Bifurcation Audit     |")
    print("+============================================================+")
    print(engine.run_sensitivity_audit())

    print()
    print("+============================================================+")
    print("|  PYTHON IRON BRIDGE — reference payload                   |")
    print("+============================================================+")
    payload = engine.ApexSynthesizer().synthesize(-7.0, 1.79, 41.0, 39.0)
    for key in (
        "carrier_144_hz",
        "carrier_456_hz",
        "snap_amplitude",
        "gate_amplitude",
        "ush2_value",
    ):
        print(f"  {key:20s} {payload[key]}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
