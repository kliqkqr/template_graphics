use std::clone::{
    Clone
};

use std::marker::{
    PhantomData 
};

use crate::geom::d2::prim::seg::{
    Segment
};

use crate::geom::d2::prim::vect::{
    Vect
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
    POrd
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D line defined by 1 line segment "seg"
#[derive(Debug)]
pub struct SLine<A, B : Segment<A>> {
    seg     : B,
    _phant  : PhantomData<A>
}

//  TRAITS --------------------------------------------------------------------------------------------------------------------------------

pub trait Line<A> : Segment<A> {
    fn intsec<B : Line<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> 
    where A : Clone + One + Zero + HAdd + HSub + HMul + HDiv + POrd 
    {
        let a = self.pos();
        let ab = self.dir();
        let c = other.pos();
        let cd = other.dir();

        let div = cd.lin_dep_det(&ab, eps)?;
        let r = cd.det(&c) + a.det(&cd) / div;

        Some(a + ab * r)
    }
}