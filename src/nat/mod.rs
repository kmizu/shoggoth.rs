use bit::{
    _0,
    _1,
    Bit,
};
use reflect::{
    Reifies,
};
use std::marker::{
    MarkerTrait,
};

mod boilerplate;

// Nat wrapper struct (grumble, grumble, coherence...)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct W<N: IsNat>(pub N);

// Classify valid binary nats (positive)
pub trait Pos: MarkerTrait + IsNat {}
impl Pos for _1 {}
impl<P: Pos, B: Bit> Pos for (P, B) {}

pub trait IsNat: MarkerTrait {}
impl IsNat for _0 {}
impl<P: Pos> IsNat for P {}

// Classify valid binary nats (with zero)
pub trait Nat: MarkerTrait + Reifies<Output = usize> {}
impl<N: IsNat + Reifies<Output = usize>> Nat for W<N> {}

pub mod ops {
    pub mod mask {
        use nat::{
            Pos,
        };
        use std::marker::{
            MarkerTrait,
        };

        #[doc(hidden)]
        pub struct IsNul;

        #[doc(hidden)]
        pub struct IsNeg;

        #[doc(hidden)]
        pub struct IsPos<P: Pos>(pub P);

        #[doc(hidden)]
        pub trait Mask: MarkerTrait {}
        impl Mask for IsNul {}
        impl Mask for IsNeg {}
        impl<P: Pos> Mask for IsPos<P> {}
    }

                   pub struct Add;
    #[doc(hidden)] pub struct AddCarry;
                   pub struct Compare;
    #[doc(hidden)] pub struct CompareCont;
    #[doc(hidden)] pub struct DoubleMask;
    #[doc(hidden)] pub struct DoublePredMask;
    #[doc(hidden)] pub struct Eq;
                   pub struct Pred;
    #[doc(hidden)] pub struct PredDouble;
    #[doc(hidden)] pub struct PredMask;
                   pub struct Sub;
    #[doc(hidden)] pub struct SubCont;
    #[doc(hidden)] pub struct SubMask;
    #[doc(hidden)] pub struct SubMaskCarry;
                   pub struct Succ;
    #[doc(hidden)] pub struct SuccDoubleMask;
}

impl<N: IsNat> Reifies for W<N> where N: Reifies<Output = usize> {
    type Output = usize;
    #[inline(always)]
    fn reflect(&self) -> usize {
        self.0.reflect()
    }
}
impl Reifies for _0 {
    type Output = usize;
    #[inline(always)]
    fn reflect(&self) -> usize {
        0
    }
}
impl Reifies for _1 {
    type Output = usize;
    #[inline(always)]
    fn reflect(&self) -> usize {
        1
    }
}
impl<P: Pos, B: Bit> Reifies for (P, B) where
    P: Reifies<Output = usize>,
    B: Reifies<Output = usize>,
{
    type Output = usize;
    #[inline(always)]
    fn reflect(&self) -> usize {
        let &(ref p, ref b) = self;
        2 * p.reflect() + b.reflect()
    }
}

#[cfg(test)]
mod test {
    use bit;
    use nat;
    use reflect::{
        Reifies,
    };

    // FIXME: add algebraic tests

    #[test]
    fn add() {
        let _: Nat!(32768) = nat!(16384) + nat!(16384);
    }

    #[test]
    fn compare() {
        println!("{:?}", nat::ops::Compare.call((nat!(10), nat!(20))));
    }

    #[test]
    fn reflect() {
        println!("{:?}", nat!(16384).reflect());
    }
}
