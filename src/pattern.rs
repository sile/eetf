use ::Term;
use ::Atom;
use ::Tuple;
use convert::TryAsRef;
use convert::AsOption;

pub trait Pattern<'a, T: ?Sized> {
    type Output;
    type Error;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug)]
pub struct Unmatch<T, C> {
    pub input: T,
    pub kind: UnmatchKind,
    pub cause: Option<Box<C>>,
}
impl<T, C> Unmatch<T, C> {
    pub fn new(input: T, kind: UnmatchKind) -> Self {
        Unmatch {
            input: input,
            kind: kind,
            cause: None,
        }
    }
    pub fn with_cause(input: T, kind: UnmatchKind, cause: C) -> Self {
        Unmatch {
            input: input,
            kind: kind,
            cause: Some(Box::new(cause)),
        }
    }
    pub fn input_type(input: T) -> Self {
        Unmatch::new(input, UnmatchKind::Type)
    }
    pub fn value(input: T) -> Self {
        Unmatch::new(input, UnmatchKind::Value)
    }
    pub fn arity(input: T) -> Self {
        Unmatch::new(input, UnmatchKind::Arity)
    }
    pub fn element(input: T, index: usize, cause: C) -> Self {
        Unmatch::with_cause(input, UnmatchKind::Element(index), cause)
    }
    pub fn head(input: T, cause: C) -> Self {
        Unmatch::with_cause(input, UnmatchKind::Head, cause)
    }
    pub fn tail(input: T, cause: C) -> Self {
        Unmatch::with_cause(input, UnmatchKind::Tail, cause)
    }
    pub fn every(input: T, cause: C) -> Self {
        Unmatch::with_cause(input, UnmatchKind::Every, cause)
    }
}

#[derive(Debug)]
pub enum UnmatchKind {
    Type,
    Value,
    Arity,
    Element(usize),
    Head,
    Tail,
    Every,
    Other,
}

#[derive(Debug)]
pub enum Union2<A, B> {
    A(A),
    B(B),
}

#[derive(Debug)]
pub enum Union3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

#[derive(Debug)]
pub enum Union4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

#[derive(Debug)]
pub enum Union5<A, B, C, D, E> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}

#[derive(Debug)]
pub enum Union6<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}

#[derive(Debug)]
pub struct Any<T>(::std::marker::PhantomData<T>);
impl<T> Any<T> {
    pub fn new() -> Self {
        Any(::std::marker::PhantomData)
    }
}
pub fn any<T>() -> Any<T> {
    Any::new()
}
impl<'a, T, O> Pattern<'a, T> for Any<O>
    where T: TryAsRef<O> + 'static,
          O: 'static
{
    type Output = &'a O;
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        input.try_as_ref().ok_or_else(|| Unmatch::input_type(input))
    }
}

impl<'a, T> Pattern<'a, T> for &'static str
    where T: TryAsRef<Atom> + 'static
{
    type Output = Self;
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let a = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((*self == a.name).as_option().ok_or_else(|| Unmatch::value(input)));
        Ok(*self)
    }
}

#[derive(Debug)]
pub struct List<P>(pub P);
impl<'a, T, P> Pattern<'a, T> for List<P>
    where P: Pattern<'a, Term>,
          T: TryAsRef<::List> + 'static
{
    type Output = Vec<P::Output>;
    type Error = Unmatch<&'a T, P::Error>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let l = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        self.try_match(&l.elements[..])
            .map_err(|e| Unmatch::with_cause(input, e.kind, *e.cause.unwrap()))
    }
}
impl<'a, P> Pattern<'a, [Term]> for List<P>
    where P: Pattern<'a, Term>
{
    type Output = Vec<P::Output>;
    type Error = Unmatch<&'a [Term], P::Error>;
    fn try_match(&self, input: &'a [Term]) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::with_capacity(input.len());
        for (i, e) in input.iter().enumerate() {
            output.push(try!(self.0.try_match(e).map_err(|e| Unmatch::element(input, i, e))));
        }
        Ok(output)
    }
}

#[derive(Debug)]
pub struct Nil;
impl<'a, T> Pattern<'a, T> for Nil
    where T: TryAsRef<::List> + 'static
{
    type Output = &'a [Term];
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let l = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((l.elements.len() == 0).as_option().ok_or_else(|| Unmatch::arity(input)));
        Ok(&l.elements)
    }
}

#[derive(Debug)]
pub struct Cons<H, T>(pub H, pub T);
impl<'a, T, P0, P1> Pattern<'a, T> for Cons<P0, P1>
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, [Term]>,
          T: TryAsRef<::List> + 'static
{
    type Output = (P0::Output, P1::Output);
    type Error = Unmatch<&'a T, Union2<P0::Error, P1::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let l = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((l.elements.len() > 0).as_option().ok_or_else(|| Unmatch::arity(input)));
        let h = try!(self.0
            .try_match(&l.elements[0])
            .map_err(|e| Unmatch::head(input, Union2::A(e))));
        let t = try!(self.1
            .try_match(&l.elements[1..])
            .map_err(|e| Unmatch::tail(input, Union2::B(e))));
        Ok((h, t))
    }
}

impl<'a, T> Pattern<'a, T> for ()
    where T: TryAsRef<Tuple> + 'static
{
    type Output = ();
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 0).as_option().ok_or_else(|| Unmatch::arity(input)));
        Ok(())
    }
}

impl<'a, T, P0> Pattern<'a, T> for (P0,)
    where P0: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = P0::Output;
    type Error = Unmatch<&'a T, P0::Error>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 1).as_option().ok_or_else(|| Unmatch::arity(input)));
        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, e)));
        Ok(o0)
    }
}

impl<'a, T, P0, P1> Pattern<'a, T> for (P0, P1)
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = (P0::Output, P1::Output);
    type Error = Unmatch<&'a T, Union2<P0::Error, P1::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 2).as_option().ok_or_else(|| Unmatch::arity(input)));
        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, Union2::A(e))));
        let o1 = try!(self.1
            .try_match(&t.elements[1])
            .map_err(|e| Unmatch::element(input, 1, Union2::B(e))));
        Ok((o0, o1))
    }
}

impl<'a, T, P0, P1, P2> Pattern<'a, T> for (P0, P1, P2)
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, Term>,
          P2: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = (P0::Output, P1::Output, P2::Output);
    type Error = Unmatch<&'a T, Union3<P0::Error, P1::Error, P2::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 3).as_option().ok_or_else(|| Unmatch::arity(input)));
        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, Union3::A(e))));
        let o1 = try!(self.1
            .try_match(&t.elements[1])
            .map_err(|e| Unmatch::element(input, 1, Union3::B(e))));
        let o2 = try!(self.2
            .try_match(&t.elements[2])
            .map_err(|e| Unmatch::element(input, 2, Union3::C(e))));
        Ok((o0, o1, o2))
    }
}

impl<'a, T, P0, P1, P2, P3> Pattern<'a, T> for (P0, P1, P2, P3)
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, Term>,
          P2: Pattern<'a, Term>,
          P3: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output);
    type Error = Unmatch<&'a T, Union4<P0::Error, P1::Error, P2::Error, P3::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 4).as_option().ok_or_else(|| Unmatch::arity(input)));

        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, Union4::A(e))));
        let o1 = try!(self.1
            .try_match(&t.elements[1])
            .map_err(|e| Unmatch::element(input, 1, Union4::B(e))));
        let o2 = try!(self.2
            .try_match(&t.elements[2])
            .map_err(|e| Unmatch::element(input, 2, Union4::C(e))));
        let o3 = try!(self.3
            .try_match(&t.elements[3])
            .map_err(|e| Unmatch::element(input, 3, Union4::D(e))));
        Ok((o0, o1, o2, o3))
    }
}

impl<'a, T, P0, P1, P2, P3, P4> Pattern<'a, T> for (P0, P1, P2, P3, P4)
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, Term>,
          P2: Pattern<'a, Term>,
          P3: Pattern<'a, Term>,
          P4: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output);
    type Error = Unmatch<&'a T, Union5<P0::Error, P1::Error, P2::Error, P3::Error, P4::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 5).as_option().ok_or_else(|| Unmatch::arity(input)));

        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, Union5::A(e))));
        let o1 = try!(self.1
            .try_match(&t.elements[1])
            .map_err(|e| Unmatch::element(input, 1, Union5::B(e))));
        let o2 = try!(self.2
            .try_match(&t.elements[2])
            .map_err(|e| Unmatch::element(input, 2, Union5::C(e))));
        let o3 = try!(self.3
            .try_match(&t.elements[3])
            .map_err(|e| Unmatch::element(input, 3, Union5::D(e))));
        let o4 = try!(self.4
            .try_match(&t.elements[4])
            .map_err(|e| Unmatch::element(input, 4, Union5::E(e))));
        Ok((o0, o1, o2, o3, o4))
    }
}

impl<'a, T, P0, P1, P2, P3, P4, P5> Pattern<'a, T> for (P0, P1, P2, P3, P4, P5)
    where P0: Pattern<'a, Term>,
          P1: Pattern<'a, Term>,
          P2: Pattern<'a, Term>,
          P3: Pattern<'a, Term>,
          P4: Pattern<'a, Term>,
          P5: Pattern<'a, Term>,
          T: TryAsRef<Tuple> + 'static
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output, P5::Output);
    type Error = Unmatch<&'a T,
            Union6<P0::Error, P1::Error, P2::Error, P3::Error, P4::Error, P5::Error>>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let t = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        try!((t.elements.len() == 6).as_option().ok_or_else(|| Unmatch::arity(input)));

        let o0 = try!(self.0
            .try_match(&t.elements[0])
            .map_err(|e| Unmatch::element(input, 0, Union6::A(e))));
        let o1 = try!(self.1
            .try_match(&t.elements[1])
            .map_err(|e| Unmatch::element(input, 1, Union6::B(e))));
        let o2 = try!(self.2
            .try_match(&t.elements[2])
            .map_err(|e| Unmatch::element(input, 2, Union6::C(e))));
        let o3 = try!(self.3
            .try_match(&t.elements[3])
            .map_err(|e| Unmatch::element(input, 3, Union6::D(e))));
        let o4 = try!(self.4
            .try_match(&t.elements[4])
            .map_err(|e| Unmatch::element(input, 4, Union6::E(e))));
        let o5 = try!(self.5
            .try_match(&t.elements[5])
            .map_err(|e| Unmatch::element(input, 5, Union6::F(e))));
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

#[derive(Debug)]
pub struct Or<T>(pub T);
impl<'a, T, P0, P1> Pattern<'a, T> for Or<(P0, P1)>
    where P0: Pattern<'a, T>,
          P1: Pattern<'a, T>,
          T: 'static
{
    type Output = Union2<P0::Output, P1::Output>;
    type Error = Unmatch<&'a T, (P0::Error, P1::Error)>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let e0 = try_err!((self.0).0.try_match(input).map(|o| Union2::A(o)));
        let e1 = try_err!((self.0).1.try_match(input).map(|o| Union2::B(o)));
        Err(Unmatch::every(input, (e0, e1)))
    }
}
impl<'a, T, P0, P1, P2> Pattern<'a, T> for Or<(P0, P1, P2)>
    where P0: Pattern<'a, T>,
          P1: Pattern<'a, T>,
          P2: Pattern<'a, T>,
          T: 'static
{
    type Output = Union3<P0::Output, P1::Output, P2::Output>;
    type Error = Unmatch<&'a T, (P0::Error, P1::Error, P2::Error)>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let e0 = try_err!((self.0).0.try_match(input).map(|o| Union3::A(o)));
        let e1 = try_err!((self.0).1.try_match(input).map(|o| Union3::B(o)));
        let e2 = try_err!((self.0).2.try_match(input).map(|o| Union3::C(o)));
        Err(Unmatch::every(input, (e0, e1, e2)))
    }
}
impl<'a, T, P0, P1, P2, P3> Pattern<'a, T> for Or<(P0, P1, P2, P3)>
    where P0: Pattern<'a, T>,
          P1: Pattern<'a, T>,
          P2: Pattern<'a, T>,
          P3: Pattern<'a, T>,
          T: 'static
{
    type Output = Union4<P0::Output, P1::Output, P2::Output, P3::Output>;
    type Error = Unmatch<&'a T, (P0::Error, P1::Error, P2::Error, P3::Error)>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let e0 = try_err!((self.0).0.try_match(input).map(|o| Union4::A(o)));
        let e1 = try_err!((self.0).1.try_match(input).map(|o| Union4::B(o)));
        let e2 = try_err!((self.0).2.try_match(input).map(|o| Union4::C(o)));
        let e3 = try_err!((self.0).3.try_match(input).map(|o| Union4::D(o)));
        Err(Unmatch::every(input, (e0, e1, e2, e3)))
    }
}
impl<'a, T, P0, P1, P2, P3, P4> Pattern<'a, T> for Or<(P0, P1, P2, P3, P4)>
    where P0: Pattern<'a, T>,
          P1: Pattern<'a, T>,
          P2: Pattern<'a, T>,
          P3: Pattern<'a, T>,
          P4: Pattern<'a, T>,
          T: 'static
{
    type Output = Union5<P0::Output, P1::Output, P2::Output, P3::Output, P4::Output>;
    type Error = Unmatch<&'a T, (P0::Error, P1::Error, P2::Error, P3::Error, P4::Error)>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let e0 = try_err!((self.0).0.try_match(input).map(|o| Union5::A(o)));
        let e1 = try_err!((self.0).1.try_match(input).map(|o| Union5::B(o)));
        let e2 = try_err!((self.0).2.try_match(input).map(|o| Union5::C(o)));
        let e3 = try_err!((self.0).3.try_match(input).map(|o| Union5::D(o)));
        let e4 = try_err!((self.0).4.try_match(input).map(|o| Union5::E(o)));
        Err(Unmatch::every(input, (e0, e1, e2, e3, e4)))
    }
}
impl<'a, T, P0, P1, P2, P3, P4, P5> Pattern<'a, T> for Or<(P0, P1, P2, P3, P4, P5)>
    where P0: Pattern<'a, T>,
          P1: Pattern<'a, T>,
          P2: Pattern<'a, T>,
          P3: Pattern<'a, T>,
          P4: Pattern<'a, T>,
          P5: Pattern<'a, T>,
          T: 'static
{
    type Output = Union6<P0::Output, P1::Output, P2::Output, P3::Output, P4::Output, P5::Output>;
    type Error = Unmatch<&'a T, (P0::Error, P1::Error, P2::Error, P3::Error, P4::Error, P5::Error)>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let e0 = try_err!((self.0).0.try_match(input).map(|o| Union6::A(o)));
        let e1 = try_err!((self.0).1.try_match(input).map(|o| Union6::B(o)));
        let e2 = try_err!((self.0).2.try_match(input).map(|o| Union6::C(o)));
        let e3 = try_err!((self.0).3.try_match(input).map(|o| Union6::D(o)));
        let e4 = try_err!((self.0).4.try_match(input).map(|o| Union6::E(o)));
        let e5 = try_err!((self.0).5.try_match(input).map(|o| Union6::F(o)));
        Err(Unmatch::every(input, (e0, e1, e2, e3, e4, e5)))
    }
}

pub struct Ascii;
impl<'a, T> Pattern<'a, T> for Ascii
    where T: TryAsRef<::FixInteger> + 'static
{
    type Output = char;
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let n = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input))).value;
        if 0 <= n && n < 0x100 {
            Ok(n as u8 as char)
        } else {
            Err(Unmatch::value(input))
        }
    }
}

pub struct Unicode;
impl<'a, T> Pattern<'a, T> for Unicode
    where T: TryAsRef<::FixInteger> + 'static
{
    type Output = char;
    type Error = Unmatch<&'a T, ()>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let n = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input))).value;
        ::std::char::from_u32(n as u32).ok_or_else(|| Unmatch::value(input))
    }
}

pub struct Str<C>(pub C);
impl<'a, T, C> Pattern<'a, T> for Str<C>
    where T: TryAsRef<::List> + 'static,
          C: Pattern<'a, ::Term, Output = char>
{
    type Output = String;
    type Error = Unmatch<&'a T, C::Error>;
    fn try_match(&self, input: &'a T) -> Result<Self::Output, Self::Error> {
        let l = try!(input.try_as_ref().ok_or_else(|| Unmatch::input_type(input)));
        let mut s = String::with_capacity(l.elements.len());
        for (i, e) in l.elements.iter().enumerate() {
            let c = try!(self.0.try_match(e).map_err(|e| Unmatch::element(input, i, e)));
            s.push(c);
        }
        Ok(s)
    }
}

//

// TODO
// UnicodeStr, UTF8-str, ASCII-str
// u8, i8, u16, i16, i32, u32, i64, u64, f32, f64, bigint, biguint
