// Data Definitions ////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T>(pub H, pub T);

// Aliases /////////////////////////////////////////////////////////////////////

pub type Append<Xs, Ys> = <Xs as ::std::ops::Add<Ys>>::Output;
pub type Single<X> = Cons<X, Nil>;

// Classifiers /////////////////////////////////////////////////////////////////

pub trait List {
    fn cons<X>(self, x: X) -> Cons<X, Self> where Self: Sized {
        Cons(x, self)
    }
}
impl List for Nil {
}
impl<H, T> List for Cons<H, T> {
}

// Infix Operators /////////////////////////////////////////////////////////////

impl<Ys> ::std::ops::Add<Ys> for Nil {
    type Output = Ys;
    fn add(self, rhs: Ys) -> Ys {
        rhs
    }
}
impl<Rec: Sized, X, Xs, Ys> ::std::ops::Add<Ys> for Cons<X, Xs> where
    Xs: ::std::ops::Add<Ys, Output = Rec>,
{
    type Output = Cons<X, Rec>;
    fn add(self, rhs: Ys) -> Cons<X, Rec> {
        Cons(self.0, self.1 + rhs)
    }
}

// Other Operators /////////////////////////////////////////////////////////////

pub trait ToSingleton {
    type Out = Cons<Self, Nil>;
    fn single(self) -> Cons<Self, Nil> where Self: Sized,
    {
        Cons(self, Nil)
    }
}
impl<A> ToSingleton for A {
    type Out = Single<A>;
}