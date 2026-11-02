# Formal arithmetic — exact scope and receipts

**Purpose.** State precisely what the external Lean 4 certificate covers, what DED's public Rust
implementation covers, and how to read a result that quotes a base-38 string.

This page exists so neither the paper nor a reviewer can accidentally turn a real proof into an
overclaim.

---

## 1. The two lanes

| Lane | What it covers | Receipt |
|---|---|---|
| **Lean 4 — balanced base-38** (external) | The **balanced base-38** arithmetic: signed digits `−18..=19`; encoding/decoding, carry, transposition, arithmetic laws. | A separate external project reports **PASS**, 0 errors, `FAILURES: 0` (Lean 4.33.1); **159 theorem/lemma declarations, 0 sorries**. The certificate source is **not included in this repository**, and it was not re-run for this release. |
| **Rust — `ZeroFree38`** (this repository) | DED’s **zero-free sparse base-38** dialect: digits `1..=37`, sparse canonical form, round-trip, add/mul/carry, gate logic. | `cd rust && cargo test --release` → **95 passed, 0 failed** in this public snapshot. The full development tree (not included) has 169 tests. |
| **Cross-witness** | The DED zero-free dialect and the tabulated bijective base-38 implementation agree at the **value** layer on the witness sample; their digit strings differ. The balanced lane is covered separately. | [`BASE38_CROSS_WITNESS.md`](BASE38_CROSS_WITNESS.md). The other implementations are external. |

The Lean certificate and the Rust suite are **two lanes**, not one execution. The Lean proof does
not compile into the DED engine, and the DED engine does not call Lean at runtime.

## 1b. Parity run — the external lane reproduces the DED values

In the engine, `UŠ₂ = round(456 × (1 + ρ))` is derived from the defect proximity `ρ`; the
compensation law then adds the two anchor deficits (`ush2 = tumor + 38 + 41`).

A separate parity harness — **not included in this repository** — ran the DED numbers through a
Rust port of the balanced-lane definitions:

- **480 engine profiles** (RMP × FD × lacunarity × burst);
- 16 canonical values and 7 add/multiply pairs;
- **2,903 checks, 0 mismatches**;
- the compensation law `ush2 = tumor + 38 + 41` checked in both lanes.

The harness executes the Rust port, **not Lean**. The digit forms differ (zero-free sparse vs
balanced signed); the **integer values agree**. This is a value-layer parity report, not a
runnable public receipt, and not a full engine swap. The engine still uses `ZeroFree38`; a swap
would change the canonical strings and public types, not the numbers.

## 2. What the external Lean certificate proves

It proves properties of the **balanced** base-38 arithmetic family, including:

- encoding and decoding are inverses on the representable domain;
- the digit membrane/carry rules;
- transposition and arithmetic identities covered by the 159 declarations;
- the runtime smoke test (transposition 324/342 caught, single-digit 340/342) reports
  `FAILURES: 0`.

It does **not** prove:

- the exact `ZeroFree38` zero-free sparse dialect used by DED;
- the biological model, the carrier frequencies, or the PE-IM-ER payload;
- any clinical, safety, or efficacy claim.

## 3. What changes if you swap dialects

If DED used the balanced dialect instead of `ZeroFree38`:

- **integer values** stay the same on the cross-witness sample (and the value-level mapping is
  the intended bridge);
- **polynomial strings** change, because the canonical digit forms differ.

So any paper result quoted as an integer/frequency is dialect-invariant. Any result quoted as a
base-38 string must name the dialect. DED’s strings are **zero-free sparse** (`ZeroFree38`).

## 4. Wording for the paper

Use:

> A separate external project reports a Lean 4 certificate for the balanced base-38 dialect
> (159 theorem/lemma declarations, 0 sorries). DED uses a zero-free sparse dialect; the DED and
> tabulated bijective forms are cross-witnessed at the value layer, and the public DED
> implementation is covered by 95 Rust tests (169 in the full development tree, not included).
> The balanced lane is covered by the external certificate, not by the cross-witness. The
> certificate covers arithmetic, not biology, and its source is not bundled with this release.

Do not use:

- “The DED engine is formally verified.”
- “The results are proven.”
- “The number system proves the biology.”
- “159 theorems prove the carrier protocol.”
- “The Lean certificate is in this repository.”

## 5. Reproduce

```bash
# Rust lane — the lane that ships here
cd rust
PYO3_PYTHON=python3 cargo test --release
# expect: 95 passed, 0 failed

# CLI smoke
./target/release/ded_ark_cli --help
./target/release/ded_ark_cli --version
```

The Lean lane is **external and not bundled**. Before relying on it, re-run the external
project's `verify.sh` and quote its current count. The Rust lane is re-run in CI on every push.

## 6. A separate zero-free deposit

A separate external Lean project studies zero-free complete sequences. It is **not** the
159-declaration balanced certificate and is **not** included here. Do not merge the two claims
in the paper.
