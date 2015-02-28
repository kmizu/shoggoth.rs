use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;

// Fn: SuccDoubleMask //////////////////////////////////////////////////////////

// x -> 2 * x + 1

/// `succ_double_mask(is_nul) ==> is_pos(1)`
ty! {
    fam SuccDoubleMask(mask::IsNul,) => mask::IsPos<_1> {
        SuccDoubleMask(mask::IsNul,) => mask::IsPos(_1)
    }
}
/// `succ_double_mask(is_neg) ==> is_neg`
ty! {
    fam SuccDoubleMask(mask::IsNeg,) => mask::IsNeg {
        SuccDoubleMask(mask::IsNeg,) => mask::IsNeg
    }
}
/// `succ_double_mask(is_pos(p)) ==> is_pos(p:1)`
ty! {
    fam SuccDoubleMask(mask::IsPos<P>,) => mask::IsPos<(P, _1)> {
        SuccDoubleMask(mask::IsPos(p),) => mask::IsPos((p, _1))
    } for :[ P: Pos ]
}
