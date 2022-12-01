use crate::geom::d2::prim::vect::{
    Vect
};

/// 2D limited transported linear span defined by 1 transport vector "base" and N vectors "vects" where the scalar for vectors is in interval \[0, 1\]
pub struct LTSpan<A, const N: usize> {
    base  : Vect<A>,
    vects : [Vect<A>; N]
}

impl<A, const N: usize> LTSpan<A, N> {
    pub fn new(base : Vect<A>, vects : [Vect<A>; N]) -> LTSpan<A, N> {
        LTSpan{base : base, vects : vects}
    }

    pub fn base(&self) -> &Vect<A> {
        &self.base
    }

    pub fn vects(&self) -> &[Vect<A>; N] {
        &self.vects
    }
}