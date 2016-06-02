use super::*;

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
    }
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

pub trait TryInto<T> {
    fn try_into(self) -> Result<T, Self> where Self: Sized;
}

impl<T> TryInto<T> for T {
    fn try_into(self) -> Result<T, Self>
        where Self: Sized
    {
        Ok(self)
    }
}

macro_rules! impl_term_try_into {
    ($to:ident) => {
        impl TryInto<$to> for Term {
            fn try_into(self) -> Result<$to, Self> where Self: Sized {
                match self {
                    Term::$to(x) => Ok(x),
                    _ => Err(self)
                }
            }
        }
    }
}
impl_term_try_into!(Atom);
impl_term_try_into!(FixInteger);
impl_term_try_into!(BigInteger);
impl_term_try_into!(Float);
impl_term_try_into!(Pid);
impl_term_try_into!(Port);
impl_term_try_into!(Reference);
impl_term_try_into!(ExternalFun);
impl_term_try_into!(InternalFun);
impl_term_try_into!(Binary);
impl_term_try_into!(BitBinary);
impl_term_try_into!(List);
impl_term_try_into!(ImproperList);
impl_term_try_into!(Tuple);
impl_term_try_into!(Map);
