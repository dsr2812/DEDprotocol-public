# Supplementary Document S2 — Frequency Provenance: Compiled-Database Heuristic Anchors

*Provenance appendix to the RBDC Framework technical report draft (`manuscript/technical_report_draft.md`, §6.3).*

**Status.** Formal provenance lock for the alternative bio-electromagnetic frequency references used by the computational model.

## 1. Purpose

This document records, without ambiguity, the provenance class of four resonance frequencies that appear in the framework's synthesis source: **456.383 Hz**, **1,873.477 Hz**, **2,221.323 Hz**, and **6,350.333 Hz**. It exists so that reviewers can distinguish at a glance between:

1. **Independently verified inputs** — peer-reviewed, directly measured biophysical anchors (resting membrane potential, fractal dimension; see technical report draft §6.3).
2. **Synthesis-source heuristic anchors** — the four frequencies below, which are **not** peer-reviewed oncological constants.

Nothing in this document asserts a physical, therapeutic, or clinical effect for any frequency. The framework's claims are limited to determinism, reproducibility, and the generation of testable payload specifications (technical report draft §6.3).

## 2. Definition: Compiled-Database Heuristic Anchors

The four frequencies are formally defined as **Compiled-Database Heuristic Anchors**:

> A **Compiled-Database Heuristic Anchor** is a scalar constant drawn from a compiled repository of bio-electromagnetic frequency observations — i.e., aggregated, historically compiled lists of resonant frequencies (including CAFL/scalar-wave-style databases and related alternative bio-electromagnetic frequency repositories) — rather than from a single peer-reviewed primary study in oncology. Such anchors are retained by the framework exclusively as **synthetic empirical baselines** that initialize its computational mappings. They carry **no independent measurement** attached by this framework, and the framework **does not claim clinical or biological validation** for them.

| Frequency | Definition in this framework | Provenance class |
|-----------|------------------------------|------------------|
| 456.383 Hz | Reference resonance from the synthesis source; the engine's integer harmonic approximation is 456 Hz (12 × 38) | Compiled-Database Heuristic Anchor (synthesis-source input) |
| 1,873.477 Hz | Additional reference frequency from the same compiled repository | Compiled-Database Heuristic Anchor (synthesis-source input) |
| 2,221.323 Hz | Additional reference frequency from the same compiled repository | Compiled-Database Heuristic Anchor (synthesis-source input) |
| 6,350.333 Hz | Additional reference frequency from the same compiled repository | Compiled-Database Heuristic Anchor (synthesis-source input) |

## 3. Explicit provenance statement

- The values 456.383 Hz, 1,873.477 Hz, 2,221.323 Hz, and 6,350.333 Hz are derived from **alternative, non-mainstream bio-electromagnetic frequency repositories** (historical compiled resonant-frequency databases, including CAFL/scalar-wave-style listings), not from peer-reviewed oncological literature.
- They are **NOT peer-reviewed oncological constants**. No claim is made that they have been validated as anti-neoplastic or therapeutic frequencies in any controlled clinical or preclinical study.
- Their role in the computational model is bounded: they serve as **synthetic empirical baselines used to initialize the UŠ₂ (Entropic Reversion) polynomial mapping** in the computational heuristic. In particular, 456.383 Hz (approximated as the integer harmonic 456.0 Hz = 12 × 38) seeds the UŠ₂ snap-gain mapping; the remaining anchors are retained in the compiled-database provenance record for reproducibility of the synthesis source's frequency set.
- The mapping of these frequencies to biological effect is **unmeasured** by this framework.

## 4. Consequence for the engine

Because these anchors are explicitly synthetic and non-validated, the engine is engineered so that **no falsifiable claim depends on their therapeutic efficacy**. The engine's falsifiable specifications (carriers, duty cycles, pulse counts, amplitudes) are derived from two active inputs — resting membrane potential and nuclear fractal dimension — while lacunarity is carried but unused in the canonical payload, and the characteristic burst frequency sets only the 39 Hz ŠÀR carrier, which the in vitro protocol does not test. The compiled-database anchors act purely as dimensionless seeding constants of the UŠ₂ polynomial mapping (see `in_vitro_falsifiability_protocol.md`). This separation of provenance classes is deliberate and structural (§6.3 of the technical report draft).

## 5. Relationship to the falsifiability protocol

The in vitro falsifiability protocol (`in_vitro_falsifiability_protocol.md`) is designed to test the *seed-frequency specificity* claim of the integer 456 Hz seed; the computed 456-channel carrier is 480.6 Hz. A null result there would **not support** the resonant-window claim of the framework's synthesis source at the stated power; it would **not** be explained away by this provenance note. This document governs only how the four anchors are *labelled* — it does not weaken or alter the protocol's decision criteria.
