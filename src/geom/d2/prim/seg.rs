use std::clone::{
    Clone
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
    POrd,
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D line segment defined by 2 points "a" "b" with direction vector b - a
#[derive(Debug)]
pub struct PSeg<A> {
    a : Vect<A>,
    b : Vect<A>
}

/// 2D line segment defined by 1 position vector "pos" and 1 direction vector "dir"
#[derive(Debug)]
pub struct VSeg<A> {
    pos : Vect<A>,
    dir : Vect<A>
}

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A> PSeg<A> {
    pub fn new(a : Vect<A>, b : Vect<A>) -> PSeg<A> {
        PSeg{a : a, b : b}
    }

    pub fn a(&self) -> &Vect<A> {
        &self.a
    }

    pub fn b(&self) -> &Vect<A> {
        &self.b
    }
}

impl<A> VSeg<A> {
    pub fn new(pos : Vect<A>, dir : Vect<A>) -> VSeg<A> {
        VSeg{pos : pos, dir : dir}
    }

    pub fn pos(&self) -> &Vect<A> {
        &self.pos 
    }    

    pub fn dir(&self) -> &Vect<A> {
        &self.dir
    }
}

impl<A : Clone> Clone for PSeg<A> {
    fn clone(&self) -> PSeg<A> {
        PSeg::new(self.a().clone(), self.b().clone())
    }
}

impl<A : Clone> Clone for VSeg<A> {
    fn clone(&self) -> VSeg<A> {
        VSeg::new(self.pos.clone(), self.dir.clone())
    }
}

//  TRAITS --------------------------------------------------------------------------------------------------------------------------------

pub trait Segment<A> {
    /// create line segment from another line segment
    fn from<B : Segment<A>>(seg : &B) -> Self;

    /// the 2 points that define the line segment
    /// 
    /// with: self.points()\[0\] = self.pos() and self.dir() = self.points()\[1\] - self.points()\[0\]
    fn points(&self) -> [Vect<A>; 2];

    /// the position vector of the line segment
    /// 
    /// with: self.points()\[0\] = self.pos()
    fn pos(&self) -> Vect<A> {
        let [point, _] = self.points();
        point
    }

    /// the direction vector of the line segment
    /// 
    /// with: self.dir() = self.points()\[1\] - self.points()\[0\]
    fn dir(&self) -> Vect<A>
    where A : HSub
    {
        let [point_0, point_1] = self.points();
        point_1 - point_0
    }

    /// add vector to both points of line segment
    fn add(&self, vect : &Vect<A>) -> PSeg<A>
    where A : Clone + HAdd
    {
        let [a, b] = self.points();
        PSeg::new(&a + vect, &b + vect)
    }

    /// sub vector from both points of line segment
    fn sub(&self, vect : &Vect<A>) -> PSeg<A>
    where A : Clone + HSub
    {
        let [a, b] = self.points();
        PSeg::new(&a - vect, &b - vect)
    }

    /// optional intersection between to line segments a + r * b with
    ///
    /// a + r * b = c + s * d
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b) where 0 <= r <= 1
    fn intsec<B : Segment<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> 
    where A : Clone + One + Zero + HAdd + HSub + HMul + HDiv + POrd 
    {
        let a = self.pos();
        let ab = self.dir();
        let c = other.pos();
        let cd = other.dir();

        let div = cd.lin_dep_det(&ab, eps)?;
        let r = cd.det(&c) + a.det(&cd) / div;

        match A::zero() <= r && r <= A::one() {
            true  => Some(a + ab * r),
            false => None
        }
    }
}

//  TRAIT IMPLS ---------------------------------------------------------------------------------------------------------------------------

impl<A : Clone> Segment<A> for PSeg<A> {
    fn from<B : Segment<A>>(seg : &B) -> PSeg<A> {
        let [a, b] = seg.points();
        PSeg::new(a, b)
    }

    fn points(&self) -> [Vect<A>; 2] {
        [self.a().clone(), self.b().clone()]
    }
}

impl<A : Clone + HAdd + HSub> Segment<A> for VSeg<A> {
    fn from<B : Segment<A>>(seg : &B) -> VSeg<A> {
        VSeg::new(seg.pos(), seg.dir())
    }

    fn points(&self) -> [Vect<A>; 2] {
        let a = self.pos.clone();
        let b = a.clone() + self.dir.clone();

        [a, b]
    }

    fn pos(&self) -> Vect<A> {
        self.pos.clone()
    }

    fn dir(&self) -> Vect<A> {
        self.dir.clone()
    }
}