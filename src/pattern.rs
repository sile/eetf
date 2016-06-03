use std;
use std::fmt::Debug;
use num;
use num::traits::ToPrimitive;
use num::bigint::ToBigInt;
use num::bigint::ToBigUint;
use super::*;
use convert::TryAsRef;
use convert::AsOption;
use convert::RefTerm;

pub type Result<'a, T> = std::result::Result<T, Unmatch<'a>>;

pub trait Pattern<'a, T: ?Sized>: Debug + Clone {
    type Output;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output>;

    fn unmatched(&self, input: &'a T) -> Unmatch<'a>
        where RefTerm<'a>: From<&'a T>,
              Self: 'static
    {
        Unmatch {
            input: RefTerm::from(input),
            pattern: Box::new(self.clone()),
            cause: None,
        }
    }
}

#[derive(Debug)]
pub struct Unmatch<'a> {
    pub input: RefTerm<'a>,
    pub pattern: Box<Debug>,
    pub cause: Option<Box<Unmatch<'a>>>,
}
impl<'a> Unmatch<'a> {
    pub fn cause(mut self, cause: Unmatch<'a>) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut curr = &self.cause;
        while let Some(ref next) = *curr {
            depth += 1;
            curr = &next.cause;
        }
        depth
    }
    pub fn max_depth(self, other: Self) -> Self {
        if self.depth() < other.depth() {
            other
        } else {
            self
        }
    }
}

#[derive(Debug, Clone)]
pub enum Union2<A, B> {
    A(A),
    B(B),
}

#[derive(Debug, Clone)]
pub enum Union3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

#[derive(Debug, Clone)]
pub enum Union4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

#[derive(Debug, Clone)]
pub enum Union5<A, B, C, D, E> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}

#[derive(Debug, Clone)]
pub enum Union6<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}

#[derive(Debug, Clone)]
pub struct Any<T>(::std::marker::PhantomData<T>);
impl<T> Any<T>
    where T: Debug
{
    pub fn new() -> Self {
        Any(::std::marker::PhantomData)
    }
}
pub fn any<T>() -> Any<T>
    where T: Debug
{
    Any::new()
}
impl<'a, T, O> Pattern<'a, T> for Any<O>
    where T: TryAsRef<O> + 'static,
          O: Debug + Clone + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = &'a O;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.try_as_ref().ok_or_else(|| self.unmatched(input))
    }
}

impl<'a, T> Pattern<'a, T> for &'static str
    where T: TryAsRef<Atom> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Self;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let a = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        try!((*self == a.name).as_option().ok_or_else(|| self.unmatched(input)));
        Ok(*self)
    }
}

#[derive(Debug, Clone)]
pub struct VarList<P>(pub P);
impl<'a, T, P> Pattern<'a, T> for VarList<P>
    where P: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Vec<P::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let l = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        self.try_match(&l.elements[..])
            .map_err(|e| self.unmatched(input).cause(e))
    }
}
impl<'a, P> Pattern<'a, [Term]> for VarList<P>
    where P: Pattern<'a, Term> + 'static
{
    type Output = Vec<P::Output>;
    fn try_match(&self, input: &'a [Term]) -> Result<'a, Self::Output> {
        let mut output = Vec::with_capacity(input.len());
        for e in input {
            output.push(try!(self.0.try_match(e).map_err(|e| self.unmatched(input).cause(e))));
        }
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct FixList<T>(pub T);
impl<'a, T, P0> Pattern<'a, T> for FixList<(P0,)>
    where P0: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = P0::Output;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 1).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        Ok(o0)
    }
}

impl<'a, T, P0, P1> Pattern<'a, T> for FixList<(P0, P1)>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 2).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!((self.0).1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1))
    }
}

impl<'a, T, P0, P1, P2> Pattern<'a, T> for FixList<(P0, P1, P2)>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 3).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!((self.0).1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!((self.0).2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2))
    }
}

impl<'a, T, P0, P1, P2, P3> Pattern<'a, T> for FixList<(P0, P1, P2, P3)>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 4).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!((self.0).1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!((self.0).2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!((self.0).3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3))
    }
}

impl<'a, T, P0, P1, P2, P3, P4> Pattern<'a, T> for FixList<(P0, P1, P2, P3, P4)>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          P4: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 5).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!((self.0).1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!((self.0).2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!((self.0).3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        let o4 = try!((self.0).4.try_match(&e[4]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3, o4))
    }
}

impl<'a, T, P0, P1, P2, P3, P4, P5> Pattern<'a, T> for FixList<(P0, P1, P2, P3, P4, P5)>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          P4: Pattern<'a, Term> + 'static,
          P5: Pattern<'a, Term> + 'static,
          T: TryAsRef<List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output, P5::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 6).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!((self.0).0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!((self.0).1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!((self.0).2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!((self.0).3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        let o4 = try!((self.0).4.try_match(&e[4]).map_err(|e| self.unmatched(input).cause(e)));
        let o5 = try!((self.0).5.try_match(&e[5]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3, o4, o5))
    }
}

#[derive(Debug, Clone)]
pub struct Nil;
impl<'a, T> Pattern<'a, T> for Nil
    where T: TryAsRef<::List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = &'a [Term];
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let l = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        try!((l.elements.len() == 0).as_option().ok_or_else(|| self.unmatched(input)));
        Ok(&l.elements)
    }
}

#[derive(Debug, Clone)]
pub struct Cons<H, T>(pub H, pub T);
impl<'a, T, P0, P1> Pattern<'a, T> for Cons<P0, P1>
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, [Term]> + 'static,
          T: TryAsRef<::List> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() > 0).as_option().ok_or_else(|| self.unmatched(input)));
        let h = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let t = try!(self.1.try_match(&e[1..]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((h, t))
    }
}

impl<'a, T> Pattern<'a, T> for ()
    where T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = ();
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let t = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        try!((t.elements.len() == 0).as_option().ok_or_else(|| self.unmatched(input)));
        Ok(())
    }
}

impl<'a, T, P0> Pattern<'a, T> for (P0,)
    where P0: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = P0::Output;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let t = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        try!((t.elements.len() == 1).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&t.elements[0]).map_err(|e| self.unmatched(input).cause(e)));
        Ok(o0)
    }
}

impl<'a, T, P0, P1> Pattern<'a, T> for (P0, P1)
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 2).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!(self.1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1))
    }
}

impl<'a, T, P0, P1, P2> Pattern<'a, T> for (P0, P1, P2)
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 3).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!(self.1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!(self.2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2))
    }
}

impl<'a, T, P0, P1, P2, P3> Pattern<'a, T> for (P0, P1, P2, P3)
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 4).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!(self.1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!(self.2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!(self.3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3))
    }
}

impl<'a, T, P0, P1, P2, P3, P4> Pattern<'a, T> for (P0, P1, P2, P3, P4)
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          P4: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 5).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!(self.1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!(self.2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!(self.3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        let o4 = try!(self.4.try_match(&e[4]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3, o4))
    }
}

impl<'a, T, P0, P1, P2, P3, P4, P5> Pattern<'a, T> for (P0, P1, P2, P3, P4, P5)
    where P0: Pattern<'a, Term> + 'static,
          P1: Pattern<'a, Term> + 'static,
          P2: Pattern<'a, Term> + 'static,
          P3: Pattern<'a, Term> + 'static,
          P4: Pattern<'a, Term> + 'static,
          P5: Pattern<'a, Term> + 'static,
          T: TryAsRef<Tuple> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output, P5::Output);
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = &try!(input.try_as_ref().ok_or_else(|| self.unmatched(input))).elements;
        try!((e.len() == 6).as_option().ok_or_else(|| self.unmatched(input)));
        let o0 = try!(self.0.try_match(&e[0]).map_err(|e| self.unmatched(input).cause(e)));
        let o1 = try!(self.1.try_match(&e[1]).map_err(|e| self.unmatched(input).cause(e)));
        let o2 = try!(self.2.try_match(&e[2]).map_err(|e| self.unmatched(input).cause(e)));
        let o3 = try!(self.3.try_match(&e[3]).map_err(|e| self.unmatched(input).cause(e)));
        let o4 = try!(self.4.try_match(&e[4]).map_err(|e| self.unmatched(input).cause(e)));
        let o5 = try!(self.5.try_match(&e[5]).map_err(|e| self.unmatched(input).cause(e)));
        Ok((o0, o1, o2, o3, o4, o5))
    }
}

macro_rules! try_err {
    ($e:expr) => {
        match $e {
            Ok(value) => return Ok(value),
            Err(err) => err
        }
    }
}

#[derive(Debug, Clone)]
pub struct Or<T>(pub T);
impl<'a, T, P0, P1> Pattern<'a, T> for Or<(P0, P1)>
    where P0: Pattern<'a, T> + 'static,
          P1: Pattern<'a, T> + 'static,
          T: 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Union2<P0::Output, P1::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = try_err!((self.0).0.try_match(input).map(|o| Union2::A(o)));
        let e = try_err!((self.0).1.try_match(input).map(|o| Union2::B(o))).max_depth(e);
        Err(self.unmatched(input).cause(e))
    }
}
impl<'a, T, P0, P1, P2> Pattern<'a, T> for Or<(P0, P1, P2)>
    where P0: Pattern<'a, T> + 'static,
          P1: Pattern<'a, T> + 'static,
          P2: Pattern<'a, T> + 'static,
          T: 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Union3<P0::Output, P1::Output, P2::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = try_err!((self.0).0.try_match(input).map(|o| Union3::A(o)));
        let e = try_err!((self.0).1.try_match(input).map(|o| Union3::B(o))).max_depth(e);
        let e = try_err!((self.0).2.try_match(input).map(|o| Union3::C(o))).max_depth(e);
        Err(self.unmatched(input).cause(e))
    }
}
impl<'a, T, P0, P1, P2, P3> Pattern<'a, T> for Or<(P0, P1, P2, P3)>
    where P0: Pattern<'a, T> + 'static,
          P1: Pattern<'a, T> + 'static,
          P2: Pattern<'a, T> + 'static,
          P3: Pattern<'a, T> + 'static,
          T: 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Union4<P0::Output, P1::Output, P2::Output, P3::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = try_err!((self.0).0.try_match(input).map(|o| Union4::A(o)));
        let e = try_err!((self.0).1.try_match(input).map(|o| Union4::B(o))).max_depth(e);
        let e = try_err!((self.0).2.try_match(input).map(|o| Union4::C(o))).max_depth(e);
        let e = try_err!((self.0).3.try_match(input).map(|o| Union4::D(o))).max_depth(e);
        Err(self.unmatched(input).cause(e))
    }
}
impl<'a, T, P0, P1, P2, P3, P4> Pattern<'a, T> for Or<(P0, P1, P2, P3, P4)>
    where P0: Pattern<'a, T> + 'static,
          P1: Pattern<'a, T> + 'static,
          P2: Pattern<'a, T> + 'static,
          P3: Pattern<'a, T> + 'static,
          P4: Pattern<'a, T> + 'static,
          T: 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Union5<P0::Output, P1::Output, P2::Output, P3::Output, P4::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = try_err!((self.0).0.try_match(input).map(|o| Union5::A(o)));
        let e = try_err!((self.0).1.try_match(input).map(|o| Union5::B(o))).max_depth(e);
        let e = try_err!((self.0).2.try_match(input).map(|o| Union5::C(o))).max_depth(e);
        let e = try_err!((self.0).3.try_match(input).map(|o| Union5::D(o))).max_depth(e);
        let e = try_err!((self.0).4.try_match(input).map(|o| Union5::E(o))).max_depth(e);
        Err(self.unmatched(input).cause(e))
    }
}
impl<'a, T, P0, P1, P2, P3, P4, P5> Pattern<'a, T> for Or<(P0, P1, P2, P3, P4, P5)>
    where P0: Pattern<'a, T> + 'static,
          P1: Pattern<'a, T> + 'static,
          P2: Pattern<'a, T> + 'static,
          P3: Pattern<'a, T> + 'static,
          P4: Pattern<'a, T> + 'static,
          P5: Pattern<'a, T> + 'static,
          T: 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = Union6<P0::Output, P1::Output, P2::Output, P3::Output, P4::Output, P5::Output>;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let e = try_err!((self.0).0.try_match(input).map(|o| Union6::A(o)));
        let e = try_err!((self.0).1.try_match(input).map(|o| Union6::B(o))).max_depth(e);
        let e = try_err!((self.0).2.try_match(input).map(|o| Union6::C(o))).max_depth(e);
        let e = try_err!((self.0).3.try_match(input).map(|o| Union6::D(o))).max_depth(e);
        let e = try_err!((self.0).4.try_match(input).map(|o| Union6::E(o))).max_depth(e);
        let e = try_err!((self.0).5.try_match(input).map(|o| Union6::F(o))).max_depth(e);
        Err(self.unmatched(input).cause(e))
    }
}

#[derive(Debug, Clone)]
pub struct Ascii;
impl<'a, T> Pattern<'a, T> for Ascii
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = char;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let n = try!(input.to_u8().ok_or_else(|| self.unmatched(input)));
        if n < 0x80 {
            Ok(n as char)
        } else {
            Err(self.unmatched(input))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unicode;
impl<'a, T> Pattern<'a, T> for Unicode
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = char;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let n = try!(input.to_u32().ok_or_else(|| self.unmatched(input)));
        ::std::char::from_u32(n).ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct Str<C>(pub C);
impl<'a, T, C> Pattern<'a, T> for Str<C>
    where T: TryAsRef<::List> + 'static,
          C: Pattern<'a, Term, Output = char> + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = String;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        let l = try!(input.try_as_ref().ok_or_else(|| self.unmatched(input)));
        let mut s = String::with_capacity(l.elements.len());
        for e in &l.elements {
            let c = try!(self.0.try_match(e).map_err(|e| self.unmatched(input).cause(e)));
            s.push(c);
        }
        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub struct U8;
impl<'a, T> Pattern<'a, T> for U8
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = u8;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_u8().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct I8;
impl<'a, T> Pattern<'a, T> for I8
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = i8;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_i8().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct U16;
impl<'a, T> Pattern<'a, T> for U16
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = u16;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_u16().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct I16;
impl<'a, T> Pattern<'a, T> for I16
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = i16;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_i16().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct U32;
impl<'a, T> Pattern<'a, T> for U32
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = u32;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_u32().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct I32;
impl<'a, T> Pattern<'a, T> for I32
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = i32;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_i32().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct U64;
impl<'a, T> Pattern<'a, T> for U64
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = u64;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_u64().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct I64;
impl<'a, T> Pattern<'a, T> for I64
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = i64;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_i64().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct Int;
impl<'a, T> Pattern<'a, T> for Int
    where T: ToBigInt + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = num::BigInt;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_bigint().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct Uint;
impl<'a, T> Pattern<'a, T> for Uint
    where T: ToBigUint + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = num::BigUint;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_biguint().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct F32;
impl<'a, T> Pattern<'a, T> for F32
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = f32;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_f32().ok_or_else(|| self.unmatched(input))
    }
}

#[derive(Debug, Clone)]
pub struct F64;
impl<'a, T> Pattern<'a, T> for F64
    where T: ToPrimitive + 'static,
          RefTerm<'a>: From<&'a T>
{
    type Output = f64;
    fn try_match(&self, input: &'a T) -> Result<'a, Self::Output> {
        input.to_f64().ok_or_else(|| self.unmatched(input))
    }
}
