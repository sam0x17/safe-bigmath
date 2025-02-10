use core::str::FromStr;

use quoth::Parsable;

use crate::{parsing::ParsedSafeDec, SafeInt};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SafeDec<const D: usize>(SafeInt);

impl<const D: usize> SafeDec<D> {
    pub const ZERO: SafeDec<D> = SafeDec::<D>(SafeInt::ZERO);
}

impl<const D: usize> FromStr for SafeDec<D> {
    type Err = quoth::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = quoth::ParseStream::from(s);
        let parsed = ParsedSafeDec::<D>::parse(&mut stream)?;
        Ok(SafeDec::<D>(parsed.raw))
    }
}

#[test]
fn test_safe_dec_from_str() {
    let parsed = "123.456".parse::<SafeDec<3>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(123456));
    let parsed = "123.456".parse::<SafeDec<4>>().unwrap();
    assert_eq!(parsed.0, SafeInt::from(1234560));
}
