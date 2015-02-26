#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

#[cfg(feature = "reflection")]
mod reflection {
    use reflect::Reifies;
    use super::*;

    impl Reifies for FF {
        type Output = bool;
        #[inline]
        fn reflect(&self) -> bool {
            false
        }
    }
    impl Reifies for TT {
        type Output = bool;
        #[inline]
        fn reflect(&self) -> bool {
            true
        }
    }
}
