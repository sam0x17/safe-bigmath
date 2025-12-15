use quoth::{
    Parsable, ParsableExt, ParseStream, Span, Spanned,
    parsable::{Exact, Nothing},
};
use rug::ops::NegAssign;

use crate::SafeInt;

extern crate alloc;
use alloc::vec::Vec;

/// Parsed representation of a fixed-scale decimal literal.
///
/// Useful for inspecting the raw scaled value and source span after parsing.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ParsableExt)]
pub struct ParsedSafeDec<const D: usize> {
    /// The scaled integer backing the parsed decimal.
    pub raw: SafeInt,
    /// Number of decimal places parsed (should equal `D` when valid).
    pub decimals: usize,
    /// Source span of the parsed literal.
    pub span: Span,
}

impl<const D: usize> Spanned for ParsedSafeDec<D> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<const D: usize> Parsable for ParsedSafeDec<D> {
    fn parse(stream: &mut ParseStream) -> quoth::Result<Self> {
        let start_position = stream.position;
        let is_neg = if stream.next_char()? == '-' {
            stream.consume(1)?;
            true
        } else {
            false
        };
        let mut major_digits = Vec::new();
        major_digits.push(stream.parse_digit()?);
        while let Ok(digit) = stream.parse_digit() {
            major_digits.push(digit);
        }
        stream.parse_value(Exact::from("."))?;
        let mut minor_digits = Vec::new();
        minor_digits.push(stream.parse_digit()?);
        while let Ok(digit) = stream.parse_digit() {
            minor_digits.push(digit);
        }
        while minor_digits.len() < D {
            minor_digits.push(0);
        }
        if minor_digits.len() > D {
            return Err(quoth::Error::new(
                stream.current_span(),
                "unexpected extra digit",
            ));
        }

        let num_minor_digits = minor_digits.len();

        let mut digits = Vec::new();
        digits.append(&mut major_digits);
        digits.append(&mut minor_digits);
        let mut raw = SafeInt::from(0);
        for &d in &digits {
            raw *= 10;
            raw += d;
        }
        if is_neg {
            raw.neg_assign();
        }
        stream.parse::<Nothing>()?;
        Ok(ParsedSafeDec {
            raw,
            decimals: num_minor_digits,
            span: Span::new(stream.source().clone(), start_position..stream.position),
        })
    }
}

/// Parsed representation of an integer literal.
///
/// Useful for inspecting the parsed value and source span.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ParsableExt)]
pub struct ParsedSafeInt {
    /// Parsed integer value.
    pub value: SafeInt,
    /// Source span of the parsed literal.
    pub span: Span,
}

impl Spanned for ParsedSafeInt {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parsable for ParsedSafeInt {
    fn parse(stream: &mut ParseStream) -> quoth::Result<Self> {
        let start_position = stream.position;
        let is_neg = if stream.next_char()? == '-' {
            stream.consume(1)?;
            true
        } else {
            false
        };
        let mut digits = Vec::new();
        digits.push(stream.parse_digit()?);
        while let Ok(digit) = stream.parse_digit() {
            digits.push(digit);
        }
        let mut raw = SafeInt::from(0);
        for &d in &digits {
            raw *= 10;
            raw += d;
        }
        if is_neg {
            raw.neg_assign();
        }
        stream.parse::<Nothing>()?;
        Ok(ParsedSafeInt {
            value: raw,
            span: Span::new(stream.source().clone(), start_position..stream.position),
        })
    }
}

#[test]
fn test_parse_safe_dec_valid_same_digits() {
    let mut stream = ParseStream::from("-3487834.885");
    let parsed = stream.parse::<ParsedSafeDec<3>>().unwrap();
    assert_eq!(parsed.raw, -3487834885i64);
}

#[test]
fn test_parse_safe_dec_invalid_extra_char() {
    let mut stream = ParseStream::from("-3487834.885a");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
    let mut stream = ParseStream::from("-3487a834.885");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
    let mut stream = ParseStream::from("-a3487834.885");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
    let mut stream = ParseStream::from("a-3487834.885");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
}

#[test]
fn test_parse_safe_dec_valid_less_digits_a() {
    let mut stream = ParseStream::from("7488793498789.9");
    let parsed = stream.parse::<ParsedSafeDec<7>>().unwrap();
    assert_eq!(parsed.raw, 74887934987899000000u128);
}

#[test]
fn test_parse_safe_dec_valid_less_digits_b() {
    let mut stream = ParseStream::from("123.456");
    let parsed = stream.parse::<ParsedSafeDec<6>>().unwrap();
    assert_eq!(parsed.raw, 123456000);
}

#[test]
fn test_parse_safe_dec_invalid_more_digits() {
    let mut stream = ParseStream::from("7488793498789.00487347878");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
}

#[test]
fn test_parse_safe_dec_invalid_no_dot() {
    let mut stream = ParseStream::from("748879349878900487347878");
    stream.parse::<ParsedSafeDec<44>>().unwrap_err();
}

#[test]
fn test_parse_safe_dec_invalid_no_minor_digits() {
    let mut stream = ParseStream::from("7488793498789.");
    stream.parse::<ParsedSafeDec<3>>().unwrap_err();
    let mut stream = ParseStream::from("7488793498789.");
    stream.parse::<ParsedSafeDec<0>>().unwrap_err();
}

#[test]
fn test_parse_safe_int_valid_positive() {
    assert_eq!(
        ParseStream::from("123456")
            .parse::<ParsedSafeInt>()
            .unwrap()
            .value,
        123456
    );
    assert_eq!(
        *ParseStream::from(
            "112233445566778829879879823749798798982893947293749823798729387293849234"
        )
        .parse::<ParsedSafeInt>()
        .unwrap()
        .value
        .raw(),
        "112233445566778829879879823749798798982893947293749823798729387293849234"
            .parse::<rug::Integer>()
            .unwrap()
    );
    assert_eq!(
        ParseStream::from("0")
            .parse::<ParsedSafeInt>()
            .unwrap()
            .value,
        0
    );
}

#[test]
fn test_parse_safe_int_valid_negative() {
    assert_eq!(
        ParseStream::from("-0")
            .parse::<ParsedSafeInt>()
            .unwrap()
            .value,
        0
    );
    assert_eq!(
        ParseStream::from("-1")
            .parse::<ParsedSafeInt>()
            .unwrap()
            .value,
        -1
    );
    assert_eq!(
        ParseStream::from("-98439874987948333035")
            .parse::<ParsedSafeInt>()
            .unwrap()
            .value,
        -98439874987948333035i128
    );
}

#[test]
fn test_parse_safe_int_invalid_decimal() {
    ParseStream::from("123.456")
        .parse::<ParsedSafeInt>()
        .unwrap_err();
}

#[test]
fn test_parse_safe_int_invalid_non_alpha() {
    ParseStream::from("123a456")
        .parse::<ParsedSafeInt>()
        .unwrap_err();
}
