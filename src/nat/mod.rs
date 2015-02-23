use bit::{
    _0,
    _1,
    Bit,
};
use reflect::{
    Reifies,
};
use std::marker::{
    PhantomFn,
};

mod boilerplate;

// Nat wrapper struct (grumble, grumble, coherence...)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct W<N: IsNat>(N);

// Classify valid binary nats (positive)
pub trait Pos: PhantomFn<Self> + IsNat {}
impl Pos for _1 {}
impl<P: Pos, B: Bit> Pos for (P, B) {}

pub trait IsNat: PhantomFn<Self> {}
impl IsNat for _0 {}
impl<P: Pos> IsNat for P {}

// Classify valid binary nats (with zero)
pub trait Nat: PhantomFn<Self> {}
impl<N: IsNat> Nat for W<N> {}

pub struct Add;
pub struct AddCarry;
pub struct Succ;
impl<N: IsNat> Reifies for W<N> where
    N: Reifies<Output = usize>
{
    type Output = usize;
    #[inline]
    fn reflect(&self) -> usize {
        self.0.reflect()
    }
}
impl Reifies for _0 {
    type Output = usize;
    #[inline]
    fn reflect(&self) -> usize {
        0
    }
}
impl Reifies for _1 {
    type Output = usize;
    #[inline]
    fn reflect(&self) -> usize {
        1
    }
}
impl<P: Pos, B: Bit> Reifies for (P, B) where
    P: Reifies<Output = usize>,
    B: Reifies<Output = usize>,
{
    type Output = usize;
    #[inline]
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

    #[test]
    fn add() {
        let _: Nat!(32768) = nat!(16384) + nat!(16384);
    }
}
