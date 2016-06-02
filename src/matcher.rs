use pattern::Pattern;
use super::*;

pub trait AsMatch {
    fn as_match<'a, P>(&'a self, pattern: P) -> Result<P::Output, P::Error>
        where P: Pattern<'a, Self>
    {
        pattern.try_match(self)
    }
}

impl AsMatch for Term {}
impl AsMatch for Atom {}
impl AsMatch for FixInteger {}
impl AsMatch for BigInteger {}
impl AsMatch for Float {}
impl AsMatch for Pid {}
impl AsMatch for Port {}
impl AsMatch for Reference {}
impl AsMatch for ExternalFun {}
impl AsMatch for InternalFun {}
impl AsMatch for Binary {}
impl AsMatch for BitBinary {}
impl AsMatch for List {}
impl AsMatch for ImproperList {}
impl AsMatch for Tuple {}
impl AsMatch for Map {}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use pattern::any;
    use pattern::U8;

    #[test]
    fn it_works() {
        let t = Term::from(Atom::from("hoge"));
        t.as_match("hoge").unwrap();

        let t = Term::from(Tuple::from(vec![Term::from(Atom::from("foo")),
                                            Term::from(Atom::from("bar"))]));
        let (_, v) = t.as_match(("foo", any::<Atom>())).unwrap();
        assert_eq!("bar", v.name);

        let t = Tuple::from(vec![Term::from(Atom::from("foo")),
                                 Term::from(Atom::from("bar")),
                                 Term::from(Tuple::from(vec![Term::from(Atom::from("bar"))]))]);
        assert!(t.as_match(("foo", "bar", "baz")).is_err());

        let t = Term::from(FixInteger::from(8));
        t.as_match(U8).unwrap();
    }
}
