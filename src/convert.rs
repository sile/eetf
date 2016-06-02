use pattern::Pattern;
use pattern::ToRefTerm;

pub type Result<'a, T> = ::std::result::Result<T, Unmatched<'a>>;

pub trait MatchAs {
    fn match_as<'a, P>(&'a self, pattern: P) -> Result<'a, P::Output>
        where P: Pattern<'a, Self>
    {
        pattern.try_match(self)
    }
}
impl MatchAs for ::Atom {}
impl MatchAs for ::Term {}
impl MatchAs for ::Tuple {}

#[derive(Debug, PartialEq)]
pub enum RefTerm<'a> {
    Term(&'a ::Term),
    Atom(&'a ::Atom),
    FixInteger(&'a ::FixInteger),
    BigInteger(&'a ::BigInteger),
    Float(&'a ::Float),
    Pid(&'a ::Pid),
    Port(&'a ::Port),
    Reference(&'a ::Reference),
    ExternalFun(&'a ::ExternalFun),
    InternalFun(&'a ::InternalFun),
    Binary(&'a ::Binary),
    BitBinary(&'a ::BitBinary),
    List(&'a ::List),
    ImproperList(&'a ::ImproperList),
    Tuple(&'a ::Tuple),
    Map(&'a ::Map),
}

#[derive(Debug)]
pub struct Unmatched<'a> {
    pub kind: UnmatchKind,
    pub input: RefTerm<'a>,
    pub cause: Option<Box<Unmatched<'a>>>,
}
impl<'a> Unmatched<'a> {
    pub fn input_type<T>(input: &'a T) -> Self
        where T: ToRefTerm
    {
        Unmatched {
            kind: UnmatchKind::Type,
            input: input.to_ref_term(),
            cause: None,
        }
    }
    pub fn value<T>(input: &'a T) -> Self
        where T: ToRefTerm
    {
        Unmatched {
            kind: UnmatchKind::Value,
            input: input.to_ref_term(),
            cause: None,
        }
    }
    pub fn arity<T>(input: &'a T) -> Self
        where T: ToRefTerm
    {
        Unmatched {
            kind: UnmatchKind::Arity,
            input: input.to_ref_term(),
            cause: None,
        }
    }
    pub fn element<T>(input: &'a T, index: usize, cause: Unmatched<'a>) -> Self
        where T: ToRefTerm
    {
        Unmatched {
            kind: UnmatchKind::Element(index),
            input: input.to_ref_term(),
            cause: Some(Box::new(cause)),
        }
    }
}

#[derive(Debug)]
pub enum UnmatchKind {
    Type,
    Value,
    Arity,
    Element(usize),
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn it_works() {
        let t = Term::from(Atom::from("hoge"));
        t.match_as("hoge").unwrap();

        let t = Term::from(Tuple::from(vec![Term::from(Atom::from("foo")),
                                            Term::from(Atom::from("bar"))]));
        t.match_as(("foo", "bar")).unwrap();

        let t = Tuple::from(vec![Term::from(Atom::from("foo")),
                                 Term::from(Tuple::from(vec![Term::from(Atom::from("bar"))]))]);
        t.match_as(("foo", "bar", "baz")).unwrap();
    }
}
