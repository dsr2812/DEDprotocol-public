# The base-38 cross-witness — three canonical forms

**Run 2026-10-10.** This note records a value-level check between DED's `ZeroFree38` encoding
and a tabulated bijective base-38 implementation. A third, balanced base-38 lane is discussed
separately. The other implementations and the witness program are external and are **not** part
of this repository.

## What was checked

The same integers, encoded by DED and by a bijective base-38 implementation, folded back to
values:

```
n,bijective_digits,ded_terms,value_agrees
1,"[1]","1·38^0",true
37,"[37]","37·38^0",true
38,"[38]","1·38^1",true
39,"[1,1]","1·38^0 + 1·38^1",true
75,"[1,37]","37·38^0 + 1·38^1",true
76,"[1,38]","2·38^1",true
77,"[2,1]","1·38^0 + 2·38^1",true
1443,"[37,37]","37·38^0 + 37·38^1",true
1444,"[37,38]","1·38^2",true
1445,"[38,1]","1·38^0 + 1·38^2",true
1000000,"[18,8,19,30]","30·38^0 + 19·38^1 + 8·38^2 + 18·38^3",true
3010936384,"[37,37,37,37,37,38]","1·38^6",true
```

**Value-level: they agree on every sample. Form-level: they are different encodings, and the
difference is named:**

| | bijective base-38 | DED `ZeroFree38` |
|---|---|---|
| digits | **1..=38** (true bijective) | **1..=37**, zero omitted (sparse) |
| 38 | `[38]` — one digit | `1·38` — place 1 |
| 76 | `[1,38]` | `2·38` |

## What this means (and does not)

- It does **not** mean the systems are the same. DED's README calls the encoding "zero-free
  sparse base-38"; in strict terms it is a **zero-free variant of base-38**: standard radix
  place values, but the zero digit is never stored — the Void is the empty polynomial. That is
  a correct, invertible encoding. Bijective base-38 is true bijective numeration (digits
  1..=38, `(n−1) mod 38 + 1`).
- Both are exact. Neither lies. The canonical *strings* differ, so any data exchanged between
  the systems must cross at the **value** (integer) layer or through a named translation —
  never by assuming the digit forms match.
- A third lane — an external **balanced base-38** implementation with signed digits −18..=19 —
  is a third canonical form. It reports a Lean 4 certificate (159 theorem/lemma declarations,
  0 sorries), but the certificate source is **not included in this repository** and was not
  re-run for this release. Three dialects, one arithmetic.

## What follows for data exchange

If two implementations ever share actual numbers, the bridge should exchange values (or use a
named translation), and keep this table as the record of why the forms differ.

## Reproducing the DED side

The DED side of the table is covered by the public Rust tests:

```bash
cd rust
cargo test --release
# expect: 95 passed, 0 failed
```

The cross-lane comparison was an external one-off. Its receipt is this table, not a program
bundled here. Do not describe the external certificate as if it were part of this repository.

(On hosts with Python 3.14, the PyO3 0.23 build may need
`PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` — a build flag, not an algorithm change.)
