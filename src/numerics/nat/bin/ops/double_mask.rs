use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;

// Fn: DoubleMask //////////////////////////////////////////////////////////////

// x -> 2 * x

/// `double_mask(is_nul) ==> is_nul`
ty! {
    fam DoubleMask(mask::IsNul,) => mask::IsNul {
        DoubleMask(mask::IsNul,) => mask::IsNul
    }
}
/// `double_mask(is_neg) ==> is_neg`
ty! {
    fam DoubleMask(mask::IsNeg,) => mask::IsNeg {
        DoubleMask(mask::IsNeg,) => mask::IsNeg
    }
}
/// `double_mask(is_pos(p)) ==> is_pos(p:0)`
ty! {
    fam DoubleMask(mask::IsPos<P>,) => mask::IsPos<(P, _0)> {
        DoubleMask(mask::IsPos(p),) => mask::IsPos((p, _0))
    } for :[ P: Pos ]
}
