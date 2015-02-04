pub use self::apply::{
    AppEval,
};
pub use self::compose::{
    Cmp,
    Cmp1,
    ProjCods,
    ProjDoms,
};
pub use self::eval::{
    Eval,
    Eval1,
};
pub use self::thunk::{
    Thunk,
};
use hlist::*;
use ty::{
    Infer,
    Tm,
    Ty,
    infer,
};

mod apply;
mod compose;
mod eval;
mod thunk;

/// Type-level operations
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Arrow<D, C>
where
       C: Ty,
       D: Ty + HList,
{}

impl<
       C: Ty,
       D: Ty + HList,
>
    Ty
for
    Arrow<D, C>
{}

/// Predicate providing access to (co)domain of type-level arrows
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level arrow type"]
pub trait
    IsArrow
where
    Self: Ty,
{
    type Dom: Ty + HList;
    type Cod: Ty;
}

impl<
       C: Ty,
       D: Ty + HList,
>
    IsArrow
for
    Arrow<D, C>
{
    type Dom = D;
    type Cod = C;
}

/// Alias for arrow types with an n-ary domain
pub type Ar<D, C> = Arrow<D, C>;

/// Alias for arrow types with a nullary domain
pub type Ar0<C> = Ar<HN, C>;

/// Alias for arrow types with a unary domain
pub type Ar1<D, C> = Ar<HC<D, HN>, C>;

/// Alias for partially applying terms of arrow types to many
/// arguments
pub type Ap<Fx, Xs> =
    <Xs as
        AppEval<
             <Fx as Infer>::Mode,
            <<Fx as Infer>::Ty as IsArrow>::Dom,
            Fx,
        >
    >::Out;

/// Alias for partially applying terms of arrow types to zero
/// arguments
pub type Ap0<Fx> = Ap<Fx, HN>;

/// Alias for partially applying terms of arrow types to a single
/// argument
pub type Ap1<Fx, X> = Ap<Fx, HC<X, HN>>;



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Id<A>
where
       A: Ty,
{}

impl<
       A: Ty,
>
    Infer
for
    Id<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<A, A>;
}

impl<
       A: Ty,
       M: Tm<A>,
>
    Eval<Id<A>>
for
    HC<M, HN>
{
    type Out = M;
}
