# Words and their licenses

**The language policy of this repository.** Written 2026-10-10; revised 2026-11-02.

## The one rule

> **A word is licensed by what was measured or proven — never by what the model hopes.**

Two layers live in this repository. They never trade words.

| layer | what it is | what licenses it |
|---|---|---|
| **the arithmetic layer** | exact base-38 numeration, polynomial state encoding, carry laws | public Rust tests + a value-level cross-witness; a separate external project also reports a Lean 4 certificate (not bundled) |
| **the biophysical layer** | the RBDC framework, defect-resolution model, candidate specifications | hypotheses and paper references only — **no clinical evidence exists or is claimed** |

## Licensed word list

| word | allowed for | never for |
|---|---|---|
| **verified / proven / certified** | the *arithmetic substrate* (digit laws, exactness, round-trips). Always name what was verified and where the certificate lives. | the biology, the model, any outcome. Never “verified treatment”, “proven mechanism”. |
| **exact** | numbers computed in the exact lanes (no floating-point rounding, residuals printed). | measurements of tissue — those carry uncertainty and must say so. |
| **deterministic** | code behaviour: same input → same output, reproducible. | anything about a patient. Deterministic ≠ true. |
| **computed / generated specification** | what the engine outputs: a candidate, a target, a falsifiable spec. | a prescription, a protocol, a treatment plan. |
| **model / modeled / framework / hypothesis / candidate / proposed** | the biophysics — every claim about cells, tissue, resonance, geometry. | — (these are the correct words; use them). |
| **falsifiable** | the specs, proudly: they can be shown wrong. | — |
| **efficacy, treats, cures, heals, therapy, clinical, patients, dose** | **nothing in this repository.** These words are not used to make a claim; when they appear, it is only to refuse the claim. | everywhere else. |

## The two-layer sentence

Read every sentence that mixes the layers and check it against this shape:

> *The arithmetic is exact and tested; the biophysical model is theoretical; no clinical claims
> are made.*

Applied to this repository's own README:

```diff
- This is a **theoretical / computational framework**: it generates deterministic,
- falsifiable specifications from measured order parameters. It makes no claim of
- clinical efficacy.
+ This is a **theoretical / computational framework**: it generates deterministic,
+ falsifiable specifications from measured order parameters. Its arithmetic is tested in
+ this repository and cross-witnessed against an external bijective base-38 lane (not bundled);
+ a separate external Lean certificate covers the balanced lane. The proof covers the numbers,
+ never the biology. It makes no claim of clinical efficacy.
```

## Why this is safer, not looser

The arithmetic receipt lets the project say **more, precisely** about the math — and keeps every
restriction on the medical words exactly where it was. The boundary is the project's strongest
asset and its strictest fence: it marks precisely where certainty stops. Everything beyond that
line remains *modeled*, *hypothesized*, *candidate* — the honest words of a researcher who is not
a clinician.
