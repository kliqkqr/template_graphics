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
    pub seg : B,
    _phant  : PhantomData<A>
}

//  TRAITS --------------------------------------------------------------------------------------------------------------------------------

pub trait Line<A> : Segment<A> {
    fn intsec<B : Line<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> 
    where A : Clone + One + Zero + HAdd + HSub + HMul + HDiv + POrd 
    {
        let a = self.pos();
        let b = self.dir();
        let c = other.pos();
        let d = other.dir();

        let div = d.lin_dep_det(&b, eps)?;
        let r = d.det(&c) + a.det(&d) / div;

        Some(a + b * r)
    }
}