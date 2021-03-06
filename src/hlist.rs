/// Heterogeneous lists
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait HList {}

/// Empty heterogeneous list
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;
impl HList for Nil {}

/// Cons heterogeneous list
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T>(pub H, pub T);
impl<H, T: HList> HList for Cons<H, T> {}

/// `HList` predicate implemented when `Self` is heterogeneous cons
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous cons"]
pub trait IsCons: HList {
    type H;
    type T: HList;
    #[inline] fn head(self) -> Self::H;
    #[inline] fn tail(self) -> Self::T;
}

impl<H, T: HList> IsCons for Cons<H, T> {
    type H = H;
    type T = T;
    #[inline] fn head(self) -> H { self.0 }
    #[inline] fn tail(self) -> T { self.1 }
}

/// Prepend for heterogeneous lists
pub trait Prepend<R> {
    type Out;
}

impl<R> Prepend<R> for Nil {
    type Out = R;
}

impl<H, R, T: HList> Prepend<R> for Cons<H, T> where T: Prepend<R> {
    type Out = Cons<H, <T as Prepend<R>>::Out>;
}

/// Snoc (cons to tail) for heterogeneous lists
pub trait Snoc<H> {
    type Out;
}

impl<X> Snoc<X> for Nil {
    type Out = Cons<X, Nil>;
}

impl<H, T: HList, X> Snoc<X> for Cons<H, T> where T: Snoc<X> {
    type Out = Cons<H, <T as Snoc<X>>::Out>;
}

/// Reverse prepend for heterogeneous lists
pub trait ReversePrepend<Acc> {
    type Out;
}

impl<Acc> ReversePrepend<Acc> for Nil {
    type Out = Acc;
}

impl<Acc, H, T: HList> ReversePrepend<Acc> for Cons<H, T> where
    T: ReversePrepend<Cons<H, Acc>>,
{
    type Out = <T as ReversePrepend<Cons<H, Acc>>>::Out;
}

/// Reverse prepend for heterogeneous lists
pub trait Reverse {
    type Out;
}

impl<Xs> Reverse for Xs where Xs: ReversePrepend<Nil> {
    type Out = <Xs as ReversePrepend<Nil>>::Out;
}

/// Alias for heterogeneous nil
pub type HN = Nil;

/// Alias for heterogeneous cons
pub type HC<H, T> = Cons<H, T>;

/// Alias for heterogeneous snoc
pub type HS<T, H> = <T as Snoc<H>>::Out;
