use std::marker::{
    Copy
};

use crate::geom::d2::prim::refactor::vect::{
    Vector
};

use crate::num::{
    Zero,
    One
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

pub trait Segment {
    type Val  : Copy;
    type Vect : Vector<Val = Self::Val, Out = Self::Vect>;
    type Out  : Segment<Vect = Self::Vect>;

    /// create new line segment of 2 points
    fn of_pnts(a : Self::Vect, b : Self::Vect) -> Self::Out;

    /// create new line segment of 1 position vector and 1 direction vector
    fn of_vects(pos : Self::Vect, dir : Self::Vect) -> Self::Out;

    /// first point of line segment
    fn a(&self) -> Self::Vect;

    /// second point of line
    fn b(&self) -> Self::Vect;

    /// direction vector of line segment with ab() = b() - a()
    fn ab(&self) -> Self::Vect;

    /// add vector to line segment points (translation)
    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// sub vector from line segment points (translation)
    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// mul vector to line segment points
    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// div vector from line segment points
    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// mul vector values with value (scaling)
    fn vadd(&self, val : Self::Val) -> Self::Out;

    // div vector values with value (scaling)
    fn vsub(&self, val : Self::Val) -> Self::Out;

    /// mul vector values with value (scaling)
    fn vmul(&self, val : Self::Val) -> Self::Out;

    // div vector values with value (scaling)
    fn vdiv(&self, val : Self::Val) -> Self::Out;

    /// points of line segment with [a(), b()] = pnts()
    fn pnts(&self) -> [Self::Vect; 2] {
        [self.a(), self.b()]
    }
    
    /// optional intersection between to line segments without epsilon zero checks
    ///
    /// a + r * b = c + s * d where r, s in \[0, 1\]
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec<S : Segment<Val = Self::Val>>(&self, other : S) -> Option<Self::Vect> 
    where Self::Val : Zero + One + HAdd + HSub + HMul + HDiv + HPEq + HPOrd
    {   
        let s_a  = self.a();
        let s_ab = self.ab();

        let o_a  = other.a();
        let o_ab = other.ab();

        let div = o_ab.indep_det(&s_ab)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let zero = Self::Val::zero();
        let one  = Self::Val::one();

        match r.inc_in(zero, one) {
            true  => {
                let p = s_a.add(s_ab.vmul(r));
                Some(p)
            },
            false => None   
        }
    }

    /// optional intersection between to line segments with epsilon zero checks
    ///
    /// a + r * b = c + s * d where r, s in \[0, 1\]
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec_eps<S : Segment<Val = Self::Val>>(&self, other : S, eps : Self::Val) -> Option<Self::Vect> 
    where Self::Val : Zero + One + HAdd + HSub + HMul + HDiv + HPOrd
    {   
        let s_a  = self.a();
        let s_ab = self.ab();

        let o_a  = other.a();
        let o_ab = other.ab();

        let div = o_ab.indep_det_eps(&s_ab, eps)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let zero = Self::Val::zero();
        let one  = Self::Val::one();

        match r.inc_in(zero, one) {
            true  => {
                let p = s_a.add(s_ab.vmul(r));
                Some(p)
            },
            false => None   
        }
    }
}