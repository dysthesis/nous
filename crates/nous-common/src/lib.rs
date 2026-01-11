#![no_std]

/// Adds two 64-bit signed integers.
#[inline]
#[must_use]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Returns a friendly greeting message.
#[must_use]
pub fn greeting() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod test {
    use super::{add, greeting};

    mod prop {
        use super::add;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn addition_commutes(x in -10_000i64..10_000, y in -10_000i64..10_000) {
                prop_assert_eq!(add(x, y), add(y, x));
            }
        }
    }

    #[test]
    fn greets_someone() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
