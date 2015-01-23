pub use self::bit::{
    Bit,
    _0,
    _1,
};
pub use self::bool::{
    And,
    Bool,
    FF,
    If,
    Not,
    Or,
    TT,
};
pub use self::fun::{
    Act,
    Arr,
    Dep,
    Eval,
    Lower,
    Rule,
    Sig,
};
pub use self::list::{
    Append,
    List,
};
pub use self::wit::{
    Wit,
};
pub use self::zipper::{
    Get,
    Left,
    Put,
    Right,
    Unzip,
    ZCons,
    Zip,
    Zipper,
};
use hlist::{
    HCons,
    HList,
    HNil,
};

mod bit;
mod bool;
mod fun;
mod list;
mod wit;
mod zipper;

/// Type-level binary integers
pub mod int;

/// Type-level natural numbers
pub mod nat;

/// Predicate classifying type-level "types"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type"]
pub trait Ty {}

/// Predicate classifying "typed" type-level "terms"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level term"]
pub trait Tm<A: Ty> {}

/// Type-level type of normal Rust types
pub enum Star {}

/// Normal Rust types lifted to terms as the type-level
pub enum Lift<A> {}
trait Rust { type Out; }
impl<A> Rust for Lift<A> { type Out = A; }

/// ```ignore
/// ----------
/// Star :: Ty
/// ```
impl Ty for Star {}

/// ```ignore
/// ----------
/// HNil :: Ty
/// ```
impl Ty for HNil {}

/// ```ignore
/// HTy :: Ty
/// TTy :: HList
/// TTy :: Ty
/// ---------------------
/// HCons<HTy, TTy> :: Ty
/// ```
impl<HTy, TTy> Ty for HCons<HTy, TTy> where
    HTy: Ty,
    TTy: HList + Ty,
{}

/// ```ignore
/// A : Rust
/// --------------
/// Lift<A> : Star
/// ```
impl<A> Tm<Star> for Lift<A> {}

/// ```ignore
/// -----------
/// HNil : Star
/// ```
impl Tm<Star> for HNil {}

/// ```ignore
/// HTm : Star
/// TTm :: HList, TTm : Star
/// ----------------------
/// HCons<HTm, TTm> : Star
/// ```
impl<HTm, TTm> Tm<Star> for HCons<HTm, TTm> where
    HTm: Tm<Star>,
    TTm: Tm<Star> + HList,
{}

/// ```ignore
/// -----------
/// HNil : HNil
/// ```
impl Tm<HNil> for HNil {}

/// ```ignore
/// HTy :: Ty
/// TTy :: HList, TTy :: Ty
/// HTm : HTy
/// TTm :: HList, TTm : TTy
/// ---------------------------------
/// HCons<HTm, TTm> : HCons<HTy, TTy>
/// ```
impl<HTy, TTy, HTm, TTm> Tm<HCons<HTy, TTy>> for HCons<HTm, TTm> where
    HTy: Ty,
    TTy: Ty + HList,
    HTm: Tm<HTy>,
    TTm: Tm<TTy> + HList,
{}

/// 0
pub type     _0b = bit::_0;
/// 2^0
pub type     _1b = bit::_1;
/// 2^1
pub type     _2b = (    _1b, _0b);
/// 2^2
pub type     _4b = (    _2b, _0b);
/// 2^3
pub type     _8b = (    _4b, _0b);
/// 2^4
pub type    _16b = (    _8b, _0b);
/// 2^5
pub type    _32b = (   _16b, _0b);
/// 2^6
pub type    _64b = (   _32b, _0b);
/// 2^7
pub type   _128b = (   _64b, _0b);
/// 2^8
pub type   _256b = (  _128b, _0b);
/// 2^9
pub type   _512b = (  _256b, _0b);
/// 2^10
pub type  _1024b = (  _512b, _0b);
/// 2^11
pub type  _2048b = ( _1024b, _0b);
/// 2^12
pub type  _4096b = ( _2048b, _0b);
/// 2^13
pub type  _8192b = ( _4096b, _0b);
/// 2^14
pub type _16384b = ( _8192b, _0b);
/// 2^15
pub type _32768b = (_16384b, _0b);
/// 2^16
pub type _65536b = (_32768b, _0b);
