use std::marker::Sized;
use matcher::Result;
use matcher::Unmatched;
use matcher::RefTerm;
use convert::TryAsRef;

pub trait Pattern<'a, I: ?Sized> {
    type Output;
    fn try_match(self, input: &'a I) -> Result<'a, Self::Output>;
}

pub trait ToRefTerm {
    fn to_ref_term(&self) -> RefTerm;
}

impl ToRefTerm for ::Term {
    fn to_ref_term(&self) -> RefTerm {
        RefTerm::Term(self)
    }
}
impl ToRefTerm for ::Tuple {
    fn to_ref_term(&self) -> RefTerm {
        RefTerm::Tuple(self)
    }
}
impl ToRefTerm for ::Atom {
    fn to_ref_term(&self) -> RefTerm {
        RefTerm::Atom(self)
    }
}


impl<'a, T> Pattern<'a, T> for &'static str
    where T: TryAsRef<::Atom> + ToRefTerm
{
    type Output = Self;
    fn try_match(self, input: &'a T) -> Result<Self::Output> {
        let input = try!(input.try_as_ref()
            .ok_or_else(|| Unmatched::input_type(input)));
        if self == input.name {
            Ok(self)
        } else {
            Err(Unmatched::value(input))
        }
    }
}

impl<'a, T, P0, P1> Pattern<'a, T> for (P0, P1)
    where P0: Pattern<'a, ::Term>,
          P1: Pattern<'a, ::Term>,
          T: TryAsRef<::Tuple> + ToRefTerm
{
    type Output = (P0::Output, P1::Output);
    fn try_match(self, input: &'a T) -> Result<Self::Output> {
        let input = try!(input.try_as_ref().ok_or(Unmatched::input_type(input)));
        if input.elements.len() != 2 {
            return Err(Unmatched::arity(input));
        }
        Ok((try!(self.0.try_match(&input.elements[0]).map_err(|e| Unmatched::element(input, 0, e))),
            try!(self.1.try_match(&input.elements[1]).map_err(|e| Unmatched::element(input, 1, e)))))
    }
}

impl<'a, T, P0, P1, P2> Pattern<'a, T> for (P0, P1, P2)
    where P0: Pattern<'a, ::Term>,
          P1: Pattern<'a, ::Term>,
          P2: Pattern<'a, ::Term>,
          T: TryAsRef<::Tuple> + ToRefTerm
{
    type Output = (P0::Output, P1::Output, P2::Output);
    fn try_match(self, input: &'a T) -> Result<Self::Output> {
        let input = try!(input.try_as_ref().ok_or(Unmatched::input_type(input)));
        if input.elements.len() != 3 {
            return Err(Unmatched::arity(input));
        }
        Ok((try!(self.0.try_match(&input.elements[0]).map_err(|e| Unmatched::element(input, 0, e))),
            try!(self.1.try_match(&input.elements[1]).map_err(|e| Unmatched::element(input, 1, e))),
            try!(self.2.try_match(&input.elements[2]).map_err(|e| Unmatched::element(input, 2, e)))))
    }
}
