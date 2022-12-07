use crate::num::{
    Zero
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv
};

use crate::rel::{
    HPEq,
    HPOrd
};

use crate::geom::d2::prim::refactor::vect::{
    Vector
};

use crate::geom::d2::prim::refactor::seg::{
    Segment 
};

pub trait Line : Segment {
    /// optional intersection between to lines without epsilon zero checks
    ///
    /// a + r * b = c + s * d
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec<S : Segment<Val = Self::Val>>(&self, other : S) -> Option<<Self::Vect as Vector>::Own> 
    where Self::Val : Zero + HAdd + HSub + HMul + HDiv + HPEq
    {   
        let [s_a, s_ab] = self.vects();
        let [o_a, o_ab] = other.vects();

        let div = o_ab.indep_det(&s_ab)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let p = s_a.add(s_ab.vmul(r));
        Some(p)
    }

    /// optional intersection between to lines with epsilon zero checks
    ///
    /// a + r * b = c + s * d
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec_eps<S : Segment<Val = Self::Val>>(&self, other : S, eps : Self::Val) -> Option<<Self::Vect as Vector>::Own> 
    where Self::Val : Zero + HAdd + HSub + HMul + HDiv + HPOrd
    {   
        let [s_a, s_ab] = self.vects();
        let [o_a, o_ab] = other.vects();

        let div = o_ab.indep_det_eps(&s_ab, eps)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let p = s_a.add(s_ab.vmul(r));
        Some(p)
    }
}
