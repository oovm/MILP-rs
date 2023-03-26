use super::*;

impl From<bool> for MixedValue {
    fn from(value: bool) -> Self {
        MixedValue::Boolean(value)
    }
}

macro_rules! impl_from_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MixedValue {
                fn from(value: $t) -> Self {
                    MixedValue::Integer(value.into())
                }
            }
            impl From<&$t> for MixedValue {
                fn from(value: &$t) -> Self {
                    MixedValue::Integer(value.clone().into())
                }
            }
        )*
    };
}

macro_rules! impl_from_dec {
    ($($t:ty),*) => {
        $(
            impl From<$t> for MixedValue {
                fn from(value: $t) -> Self {
                    MixedValue::Decimal(value.into())
                }
            }
            impl From<&$t> for MixedValue {
                fn from(value: &$t) -> Self {
                    MixedValue::Decimal(value.clone().into())
                }
            }
        )*
    };
}

impl_from_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_from_int!(BigInt, BigUint);
impl_from_dec!(f32, f64);
