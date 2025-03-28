use super::*;
use std::convert::TryInto;

pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}

impl<T> TryAsRef<T> for T {
    fn try_as_ref(&self) -> Option<&T> {
        Some(self)
    }
}

macro_rules! impl_term_try_as_ref {
    ($to:ident) => {
        impl TryAsRef<$to> for Term {
            fn try_as_ref(&self) -> Option<&$to> {
                match *self {
                    Term::$to(ref x) => Some(x),
                    _ => None,
                }
            }
        }
    };
}
impl_term_try_as_ref!(Atom);
impl_term_try_as_ref!(FixInteger);
impl_term_try_as_ref!(BigInteger);
impl_term_try_as_ref!(Float);
impl_term_try_as_ref!(Pid);
impl_term_try_as_ref!(Port);
impl_term_try_as_ref!(Reference);
impl_term_try_as_ref!(ExternalFun);
impl_term_try_as_ref!(InternalFun);
impl_term_try_as_ref!(Binary);
impl_term_try_as_ref!(BitBinary);
impl_term_try_as_ref!(List);
impl_term_try_as_ref!(ImproperList);
impl_term_try_as_ref!(Tuple);
impl_term_try_as_ref!(Map);
impl_term_try_as_ref!(ByteList);

macro_rules! impl_term_try_into {
    ($to:ident) => {
        impl TryInto<$to> for Term {
            type Error = Self;

            fn try_into(self) -> Result<$to, Self>
            where
                Self: Sized,
            {
                match self {
                    Term::$to(x) => Ok(x),
                    _ => Err(self),
                }
            }
        }
    };
}
macro_rules! impl_term_try_into_boxed {
    ($to:ident) => {
        impl TryInto<$to> for Term {
            type Error = Self;

            fn try_into(self) -> Result<$to, Self>
            where
                Self: Sized,
            {
                match self {
                    Term::$to(x) => Ok(*x),
                    _ => Err(self),
                }
            }
        }
    };
}
impl_term_try_into!(Atom);
impl_term_try_into!(FixInteger);
impl_term_try_into!(BigInteger);
impl_term_try_into!(Float);
impl_term_try_into!(Pid);
impl_term_try_into!(Port);
impl_term_try_into_boxed!(Reference);
impl_term_try_into_boxed!(ExternalFun);
impl_term_try_into_boxed!(InternalFun);
impl_term_try_into!(Binary);
impl_term_try_into!(BitBinary);
impl_term_try_into!(List);
impl_term_try_into!(ImproperList);
impl_term_try_into!(Tuple);
impl_term_try_into!(Map);
impl_term_try_into!(ByteList);

pub trait AsOption {
    fn as_option(&self) -> Option<&Self>;
}
impl AsOption for bool {
    fn as_option(&self) -> Option<&Self> {
        if *self {
            Some(self)
        } else {
            None
        }
    }
}

impl num_traits::ToPrimitive for FixInteger {
    fn to_i64(&self) -> Option<i64> {
        Some(i64::from(self.value))
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.value as u64)
    }
    fn to_f64(&self) -> Option<f64> {
        Some(f64::from(self.value))
    }
}
impl num_traits::ToPrimitive for BigInteger {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }
}
impl num_traits::ToPrimitive for Float {
    fn to_i64(&self) -> Option<i64> {
        None
    }
    fn to_u64(&self) -> Option<u64> {
        None
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.value)
    }
}
impl num_traits::ToPrimitive for Term {
    fn to_i64(&self) -> Option<i64> {
        match *self {
            Term::FixInteger(ref x) => x.to_i64(),
            Term::BigInteger(ref x) => x.to_i64(),
            _ => None,
        }
    }
    fn to_u64(&self) -> Option<u64> {
        match *self {
            Term::FixInteger(ref x) => x.to_u64(),
            Term::BigInteger(ref x) => x.to_u64(),
            _ => None,
        }
    }
    fn to_f64(&self) -> Option<f64> {
        match *self {
            Term::FixInteger(ref x) => x.to_f64(),
            Term::BigInteger(ref x) => x.to_f64(),
            Term::Float(ref x) => x.to_f64(),
            _ => None,
        }
    }
}

impl num_bigint::ToBigInt for FixInteger {
    fn to_bigint(&self) -> Option<num_bigint::BigInt> {
        Some(BigInteger::from(self).value)
    }
}
impl num_bigint::ToBigInt for BigInteger {
    fn to_bigint(&self) -> Option<num_bigint::BigInt> {
        Some(self.value.clone())
    }
}
impl num_bigint::ToBigInt for Term {
    fn to_bigint(&self) -> Option<num_bigint::BigInt> {
        match *self {
            Term::FixInteger(ref x) => x.to_bigint(),
            Term::BigInteger(ref x) => x.to_bigint(),
            _ => None,
        }
    }
}

impl num_bigint::ToBigUint for FixInteger {
    fn to_biguint(&self) -> Option<num_bigint::BigUint> {
        BigInteger::from(self).value.to_biguint()
    }
}
impl num_bigint::ToBigUint for BigInteger {
    fn to_biguint(&self) -> Option<num_bigint::BigUint> {
        self.value.to_biguint()
    }
}
impl num_bigint::ToBigUint for Term {
    fn to_biguint(&self) -> Option<num_bigint::BigUint> {
        match *self {
            Term::FixInteger(ref x) => x.to_biguint(),
            Term::BigInteger(ref x) => x.to_biguint(),
            _ => None,
        }
    }
}
