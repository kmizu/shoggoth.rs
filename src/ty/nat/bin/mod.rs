use self::pos::{
    Pos,
};
use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
};
pub use ty::bit::{
    _0,
    _1,
};

/// Binary positive natural numbers
pub mod pos;

/// Binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nat {}
impl Ty for Nat {}

impl Tm<Nat> for _0 {}
impl<P: Tm<Pos>> Tm<Nat> for P {}

/// Type-level successor for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ {}
impl Sig for Succ { type Dom = Nat; type Cod = Nat; }
// 0 => 1
impl FnTm<Succ> for _0
{
    type O = _1;
}
// p => succ(p)
impl<P: Tm<Pos>, Rec: Tm<Nat>> FnTm<Succ> for P where
    P: FnTm<pos::Succ, O = Rec>,
{
    type O = Rec;
}

/// Type-level addition for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl Sig for Add { type Dom = (Nat, Nat); type Cod = Nat; }
impl<P1: Tm<Pos>> FnTm<Add> for ((_0), (P1))
// 0, n => n
{
    type O = P1;
}
impl<P0: Tm<Pos>> FnTm<Add> for (P0, (_0))
// m, 0 => m
{
    type O = P0;
}
// p, q => p + q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> FnTm<Add> for ((P0), (P1)) where
    ((P0), (P1)): FnTm<pos::Add, O = Rec>,
{
    type O = Rec;
}

/// Type-level multiplication for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl Sig for Mul { type Dom = (Nat, Nat); type Cod = Nat; }
// 0, n => 0
impl<P1: Tm<Pos>> FnTm<Mul> for ((_0), (P1))
{
    type O = _0;
}
// m, 0 => 0
impl<P0: Tm<Pos>> FnTm<Mul> for (P0, (_0))
{
    type O = _0;
}
// p, q => p * q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> FnTm<Mul> for ((P0), (P1)) where
    ((P0), (P1)): FnTm<pos::Mul, O = Rec>,
{
    type O = Rec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ty::literal::*;
    use ty::wit::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() { let _: Wit<_16384b> = wit::<Add, (_0b, _16384b)>(); }

    #[test]
    fn add() { let _: Wit<_16384b> = wit::<Add, (_8192b, _8192b)>(); }

    #[test]
    fn mul_0() { let _: Wit<_0b> = wit::<Mul, (_0b, _16384b)>(); }

    #[test]
    fn mul_1() { let _: Wit<_16384b> = wit::<Mul, (_1b, _16384b)>(); }

    #[test]
    fn mul() { let _: Wit<_65536b> = wit::<Mul, (_32b, _2048b)>(); }
}