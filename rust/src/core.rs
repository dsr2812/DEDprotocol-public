//! Zero-free bijective arithmetic — the numeric substrate of the DED resonance engine.
//!
//! The encoding is implemented generically as [`BijectiveEncoding<B>`] for any
//! radix `B`, then specialized to base-38 via the [`ZeroFree38`] type alias.
//! This makes the radix an explicit, testable hyperparameter rather than a
//! fixed constant: determinism, reversibility, and non-nullness are properties
//! of the general construction, not of any particular base.
//!
//! Model rules implemented here:
//!   * Digits are nonzero, restricted to `1..=B-1`; zero is the Void and never stored.
//!   * A number is a sparse polynomial over place values `B^place`; an absent
//!     place is an implied zero (gap logic).
//!   * Coefficients above `B-1` are resolved by carry propagation. When a place
//!     resolves to a perfect multiple of `B` (remainder == 0) its coefficient is
//!     an implied zero and is omitted from the sparse vector.
//!   * In the 38-instantiation, digits are split across the 19/19 membrane
//!     (`is_above` 1..=19, `is_below` 20..=37) — a framework-specific semantic.
//!
//! This is a computational model of a theoretical number system. It makes no
//! claims about physical or biological reality.

use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU8;
use std::ops::{Add, Mul};

/// Default radix of the system. Valid digit set is `1..=37`.
///
/// Base-38 is a framework-specific hyperparameter — one specialization of the
/// general [`BijectiveEncoding<B>`]. No physical significance is claimed for
/// the radix itself.
pub const BASE: u32 = 38;

/// The 19/19 membrane gate. Digits `1..=GATE` sit above it, `GATE+1..=37` below.
/// This is a 38-instantiation semantic; it is not meaningful for other radices.
pub const GATE: u8 = 19;

/// A single bijective digit. Invariant: value is in `1..=255` (never the Void).
///
/// The per-radix ceiling (`B-1`) is enforced by [`BijectiveEncoding<B>`], not
/// by this type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(NonZeroU8);

impl Digit {
    /// Lowest valid digit.
    pub const MIN: u8 = 1;
    /// Highest representable digit. The ceiling `B-1` is enforced by the
    /// encoding's radix.
    pub const MAX: u8 = 255;

    /// Construct a digit, rejecting `0` (the Void). The upper bound `255` is
    /// inherent to `u8`.
    #[inline]
    pub fn new(value: u8) -> Option<Digit> {
        NonZeroU8::new(value).map(Digit)
    }

    /// Const constructor for compile-time digit constants.
    ///
    /// # Safety
    /// `value` must be in `1..=255`. A value of 0 violates the type invariant
    /// and produces an invalid number.
    #[inline]
    pub const unsafe fn new_unchecked(value: u8) -> Digit {
        Digit(unsafe { NonZeroU8::new_unchecked(value) })
    }

    /// The digit's numeric value, `1..=255`.
    #[inline]
    pub const fn get(self) -> u8 {
        self.0.get()
    }

    /// True for digits on the upper side of the 19/19 membrane (`1..=19`).
    /// Only meaningful in the 38-instantiation.
    #[inline]
    pub const fn is_above(self) -> bool {
        self.get() <= GATE
    }

    /// True for digits below the 19/19 membrane (`20..=37`).
    /// Only meaningful in the 38-instantiation.
    #[inline]
    pub const fn is_below(self) -> bool {
        self.get() > GATE
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digit({})", self.get())
    }
}

/// A term of the sparse polynomial: `digit * B^place`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Term {
    pub place: u32,
    pub digit: Digit,
}

/// A zero-free bijective number in radix `B`, digits `1..=B-1`.
///
/// Stored as terms sorted ascending by `place` with no duplicate places and no
/// implied-zero coefficients. The empty vector is the value zero (the Void).
///
/// `B` must satisfy `2 <= B <= 256`; the radix is validated at compile time on
/// first construction.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BijectiveEncoding<const B: u32> {
    terms: Vec<Term>,
}

/// The DED engine's specific instantiation: zero-free base-38.
pub type ZeroFree38 = BijectiveEncoding<38>;

/// Percent-scaled instantiation: zero-free base-100.
///
/// Used for percent-valued features (packing deficits, discrepancy basis
/// points) where place 0 holds the fractional percent and place 1 the whole
/// percent: 1269 basis points → `69 + 12·100` (12.69%).
pub type PercentEncoding = BijectiveEncoding<100>;

/// Arithmetic failure modes of [`BijectiveEncoding`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Num38Error {
    /// A carry (or convolution) exceeded the `u32` place-value space.
    PlaceOverflow { left: u32, right: u32 },
}

impl fmt::Display for Num38Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num38Error::PlaceOverflow { left, right } => {
                write!(f, "place-value overflow at {left} + {right}")
            }
        }
    }
}

impl std::error::Error for Num38Error {}

impl<const B: u32> BijectiveEncoding<B> {
    /// Compile-time radix invariant: `2 <= B <= 256` so digits fit `u8`.
    const RADIX_OK: () = assert!(
        B >= 2 && B <= 256,
        "bijective radix must satisfy 2 <= B <= 256"
    );

    /// Force evaluation of [`Self::RADIX_OK`] at the use site.
    #[inline]
    fn check_radix() {
        assert_eq!(Self::RADIX_OK, ());
    }

    /// The additive identity: no terms, the Void.
    pub const ZERO: Self = Self { terms: Vec::new() };

    /// A number equal to the Void (zero).
    #[inline]
    pub fn zero() -> Self {
        Self::ZERO
    }

    /// True when this number is the Void (zero).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// The sparse terms, ascending by place.
    #[inline]
    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// Highest occupied place, if any.
    #[inline]
    pub fn max_place(&self) -> Option<u32> {
        self.terms.last().map(|t| t.place)
    }

    /// Number of nonzero digits currently occupying place values.
    #[inline]
    pub fn digit_count(&self) -> usize {
        self.terms.len()
    }

    /// `(count_above_gate, count_below_gate)` across all occupied places.
    #[inline]
    pub fn gate_tally(&self) -> (usize, usize) {
        let above = self.terms.iter().filter(|t| t.digit.is_above()).count();
        (above, self.terms.len() - above)
    }

    /// Convert from an ordinary `u128`. Any value is representable; zero maps
    /// to the empty (Void) form.
    pub fn from_u128(value: u128) -> Self {
        Self::check_radix();
        let mut value = value;
        let mut terms = Vec::new();
        let mut place = 0u32;
        while value > 0 {
            let digit = (value % B as u128) as u8;
            if digit != 0 {
                terms.push(Term {
                    place,
                    digit: Digit::new(digit).expect("remainder is in 1..=B-1"),
                });
            }
            value /= B as u128;
            place += 1;
        }
        Self { terms }
    }

    /// Convert from a `u64`.
    #[inline]
    pub fn from_u64(value: u64) -> Self {
        Self::from_u128(value as u128)
    }

    /// Evaluate to `u128`, or `None` if the value exceeds `u128::MAX`.
    pub fn to_u128(&self) -> Option<u128> {
        let mut acc: u128 = 0;
        for t in &self.terms {
            let pow = B as u128;
            let pow = pow.checked_pow(t.place)?;
            acc = acc.checked_add(pow.checked_mul(t.digit.get() as u128)?)?;
        }
        Some(acc)
    }

    /// Evaluate to `f64` for telemetry. Unbounded exponents saturate naturally.
    pub fn to_f64(&self) -> f64 {
        self.terms
            .iter()
            .map(|t| t.digit.get() as f64 * (B as f64).powf(t.place as f64))
            .sum()
    }

    /// Add another number, carrying through gaps. Returns `Err` only if a final
    /// carry would overflow the `u32` place space.
    pub fn checked_add(&self, other: &Self) -> Result<Self, Num38Error> {
        let mut coeffs = Vec::with_capacity(self.terms.len() + other.terms.len());
        let (mut i, mut j) = (0usize, 0usize);
        while i < self.terms.len() && j < other.terms.len() {
            let a = &self.terms[i];
            let b = &other.terms[j];
            match a.place.cmp(&b.place) {
                Ordering::Less => {
                    coeffs.push((a.place, a.digit.get() as u64));
                    i += 1;
                }
                Ordering::Greater => {
                    coeffs.push((b.place, b.digit.get() as u64));
                    j += 1;
                }
                Ordering::Equal => {
                    coeffs.push((a.place, a.digit.get() as u64 + b.digit.get() as u64));
                    i += 1;
                    j += 1;
                }
            }
        }
        coeffs.extend(
            self.terms[i..]
                .iter()
                .map(|t| (t.place, t.digit.get() as u64)),
        );
        coeffs.extend(
            other.terms[j..]
                .iter()
                .map(|t| (t.place, t.digit.get() as u64)),
        );
        Self::normalize(coeffs)
    }

    /// Multiply by another number via polynomial convolution, then normalize.
    pub fn checked_mul(&self, other: &Self) -> Result<Self, Num38Error> {
        if self.is_zero() || other.is_zero() {
            return Ok(Self::ZERO);
        }
        let mut coeffs = Vec::with_capacity(self.terms.len().saturating_mul(other.terms.len()));
        for a in &self.terms {
            for b in &other.terms {
                let place = a
                    .place
                    .checked_add(b.place)
                    .ok_or(Num38Error::PlaceOverflow {
                        left: a.place,
                        right: b.place,
                    })?;
                coeffs.push((place, a.digit.get() as u64 * b.digit.get() as u64));
            }
        }
        Self::normalize(coeffs)
    }

    /// The normalize engine: sorts and merges raw coefficient terms, then
    /// propagates carries upward. Any place whose coefficient resolves to a
    /// perfect multiple of `B` (remainder 0) becomes an implied zero and is
    /// omitted. Output is always canonical: ascending, gap-free of zeros,
    /// digits in `1..=B-1`.
    fn normalize(mut coeffs: Vec<(u32, u64)>) -> Result<Self, Num38Error> {
        coeffs.sort_unstable_by_key(|&(place, _)| place);
        let mut out: Vec<Term> = Vec::with_capacity(coeffs.len());
        let mut carry: u64 = 0;
        let mut idx = 0usize;
        let mut place = coeffs.first().map(|&(p, _)| p).unwrap_or(0);
        loop {
            let mut total = carry;
            while idx < coeffs.len() && coeffs[idx].0 == place {
                total += coeffs[idx].1;
                idx += 1;
            }
            carry = total / B as u64;
            let digit = (total % B as u64) as u8;
            if digit != 0 {
                out.push(Term {
                    place,
                    digit: Digit::new(digit).expect("remainder is in 1..=B-1"),
                });
            }
            if idx == coeffs.len() && carry == 0 {
                break;
            }
            place = place.checked_add(1).ok_or(Num38Error::PlaceOverflow {
                left: place,
                right: 1,
            })?;
        }
        Ok(Self { terms: out })
    }
}

impl<const B: u32> From<u128> for BijectiveEncoding<B> {
    #[inline]
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl<const B: u32> From<u64> for BijectiveEncoding<B> {
    #[inline]
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl<const B: u32> Add for BijectiveEncoding<B> {
    type Output = Result<Self, Num38Error>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(&rhs)
    }
}

impl<const B: u32> Mul for BijectiveEncoding<B> {
    type Output = Result<Self, Num38Error>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(&rhs)
    }
}

impl<const B: u32> fmt::Display for BijectiveEncoding<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "∅");
        }
        for (i, t) in self.terms.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            match t.place {
                0 => write!(f, "{}", t.digit.get())?,
                1 => write!(f, "{}·{B}", t.digit.get())?,
                p => write!(f, "{}·{B}^{p}", t.digit.get())?,
            }
        }
        Ok(())
    }
}

impl<const B: u32> Ord for BijectiveEncoding<B> {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut a = self.terms.iter().rev();
        let mut b = other.terms.iter().rev();
        loop {
            match (a.next(), b.next()) {
                (Some(ta), Some(tb)) => match ta.place.cmp(&tb.place) {
                    Ordering::Equal => match ta.digit.cmp(&tb.digit) {
                        Ordering::Equal => continue,
                        o => return o,
                    },
                    o => return o,
                },
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
            }
        }
    }
}

impl<const B: u32> PartialOrd for BijectiveEncoding<B> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn d(value: u8) -> Digit {
        unsafe { Digit::new_unchecked(value) }
    }

    fn num(terms: &[(u32, u8)]) -> ZeroFree38 {
        ZeroFree38 {
            terms: terms
                .iter()
                .map(|&(place, value)| Term {
                    place,
                    digit: d(value),
                })
                .collect(),
        }
    }

    #[test]
    fn digit_bounds() {
        assert_eq!(Digit::new(0), None);
        assert_eq!(Digit::new(1).map(|x| x.get()), Some(1));
        assert_eq!(Digit::new(37).map(|x| x.get()), Some(37));
        assert_eq!(Digit::new(38).map(|x| x.get()), Some(38));
        assert_eq!(Digit::new(255).map(|x| x.get()), Some(255));
    }

    /// Prove that determinism, reversibility, and non-nullness hold for ANY
    /// bijective radix — the base-38 choice is a hyperparameter, not a law.
    fn assert_radix_properties<const B: u32>() {
        assert!(B >= 2 && B <= 256, "sweep precondition");
        let samples = [
            0u128,
            1,
            (B as u128) - 1,
            B as u128,
            B as u128 + 1,
            37,
            850,
            3_400,
            38u128.pow(5) - 1,
            u64::MAX as u128,
        ];
        for v in samples {
            let n1 = BijectiveEncoding::<B>::from_u128(v);
            let n2 = BijectiveEncoding::<B>::from_u128(v);
            assert_eq!(n1, n2, "determinism failed for radix {B}, value {v}");
            assert_eq!(
                n1.to_u128(),
                Some(v),
                "reversibility failed for radix {B}, value {v}"
            );
            assert!(
                n1.terms().iter().all(|t| {
                    let d = t.digit.get();
                    d >= 1 && (d as u32) < B
                }),
                "digit outside 1..=B-1 for radix {B}, value {v}"
            );
            assert_eq!(
                n1.is_zero(),
                v == 0,
                "non-nullness failed for radix {B}, value {v}"
            );
        }
        let pairs = [(1u128, 1), (B as u128, 1), (37, 37), (850, 1_443)];
        for (x, y) in pairs {
            assert_eq!(
                (BijectiveEncoding::<B>::from_u128(x) + BijectiveEncoding::<B>::from_u128(y))
                    .unwrap()
                    .to_u128(),
                Some(x + y),
                "add property failed for radix {B}, {x} + {y}"
            );
            assert_eq!(
                (BijectiveEncoding::<B>::from_u128(x) * BijectiveEncoding::<B>::from_u128(y))
                    .unwrap()
                    .to_u128(),
                Some(x * y),
                "mul property failed for radix {B}, {x} * {y}"
            );
        }
    }

    #[test]
    fn test_radix_sweep() {
        assert_radix_properties::<2>();
        assert_radix_properties::<10>();
        assert_radix_properties::<37>();
        assert_radix_properties::<38>();
        assert_radix_properties::<41>();
    }

    #[test]
    fn gate_split() {
        assert!(d(1).is_above());
        assert!(d(19).is_above());
        assert!(!d(19).is_below());
        assert!(d(20).is_below());
        assert!(!d(20).is_above());
        assert!(d(37).is_below());
    }

    #[test]
    fn roundtrip_u128() {
        let samples = [
            0u128,
            1,
            19,
            37,
            38,
            39,
            74,
            75,
            1_443,
            1_444,
            38u128.pow(5) - 1,
            u64::MAX as u128,
            u128::MAX,
        ];
        for v in samples {
            assert_eq!(
                ZeroFree38::from_u128(v).to_u128(),
                Some(v),
                "roundtrip failed for {v}"
            );
        }
    }

    #[test]
    fn thirty_eight_is_a_single_term() {
        let n = ZeroFree38::from_u128(38);
        assert_eq!(
            n.terms(),
            &[Term {
                place: 1,
                digit: d(1)
            }]
        );
        assert_eq!(n.to_string(), "1·38");
    }

    #[test]
    fn one_thousand_four_forty_four_has_only_one_term() {
        let n = ZeroFree38::from_u128(38 * 38);
        assert_eq!(
            n.terms(),
            &[Term {
                place: 2,
                digit: d(1)
            }]
        );
    }

    #[test]
    fn add_carries_and_omits_implied_zeros() {
        assert_eq!((num(&[(0, 37)]) + num(&[(0, 1)])).unwrap(), num(&[(1, 1)]));
        assert_eq!(
            (num(&[(0, 37)]) + num(&[(0, 36)])).unwrap(),
            num(&[(0, 35), (1, 1)])
        );
        assert_eq!(
            (ZeroFree38::from_u128(1443) + ZeroFree38::from_u128(1)).unwrap(),
            num(&[(2, 1)])
        );
    }

    #[test]
    fn add_property_matches_u128() {
        let pairs = [(1, 37u128), (38, 38), (74, 1), (1443, 37), (700, 888)];
        for (x, y) in pairs {
            let got = (ZeroFree38::from_u128(x) + ZeroFree38::from_u128(y))
                .unwrap()
                .to_u128();
            assert_eq!(got, Some(x + y), "add mismatch for {x} + {y}");
        }
    }

    #[test]
    fn mul_convolution_and_carry() {
        assert_eq!(
            (num(&[(0, 7)]) * num(&[(0, 7)])).unwrap(),
            num(&[(0, 11), (1, 1)])
        );
        assert_eq!(
            (num(&[(0, 37)]) * num(&[(0, 37)])).unwrap(),
            num(&[(0, 1), (1, 36)])
        );
        assert_eq!(
            (num(&[(0, 1), (1, 1)]) * num(&[(0, 37)])).unwrap(),
            num(&[(0, 37), (1, 37)])
        );
    }

    #[test]
    fn mul_property_matches_u128() {
        let pairs = [(7u128, 7), (37, 37), (38, 38), (1443, 39), (12345, 678)];
        for (x, y) in pairs {
            let got = (ZeroFree38::from_u128(x) * ZeroFree38::from_u128(y))
                .unwrap()
                .to_u128();
            assert_eq!(got, Some(x * y), "mul mismatch for {x} * {y}");
        }
    }

    #[test]
    fn implied_zero_on_perfect_multiple() {
        assert_eq!((num(&[(0, 38)]) + num(&[(0, 38)])).unwrap(), num(&[(1, 2)]));
        assert_eq!(
            (num(&[(0, 38)]) + num(&[(0, 38)])).unwrap().digit_count(),
            1
        );
    }

    #[test]
    fn ordering() {
        let a = num(&[(1, 1)]);
        let b = num(&[(0, 37)]);
        let c = num(&[(0, 37), (1, 1)]);
        assert!(ZeroFree38::ZERO < b);
        assert!(a > b);
        assert!(a < c);
        assert_eq!(ZeroFree38::ZERO.cmp(&ZeroFree38::ZERO), Ordering::Equal);
        assert_eq!(a.cmp(&num(&[(1, 1)])), Ordering::Equal);
        assert!(b < num(&[(0, 37), (1, 2)]));
    }

    #[test]
    fn to_u128_detects_overflow() {
        let huge = ZeroFree38 {
            terms: vec![Term {
                place: 30,
                digit: d(37),
            }],
        };
        assert!(huge.to_u128().is_none());
    }

    #[test]
    fn place_overflow_is_reported() {
        let top = ZeroFree38 {
            terms: vec![Term {
                place: u32::MAX,
                digit: d(1),
            }],
        };
        assert_eq!(
            top.checked_mul(&num(&[(1, 2)])),
            Err(Num38Error::PlaceOverflow {
                left: u32::MAX,
                right: 1,
            })
        );
    }

    #[test]
    fn gate_tally() {
        let n = num(&[(0, 7), (1, 19), (2, 37), (3, 21)]);
        assert_eq!(n.gate_tally(), (2, 2));
    }
}
