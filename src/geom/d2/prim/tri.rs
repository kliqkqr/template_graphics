use std::clone::{
    Clone
};

use std::convert::{
    From 
};

use crate::geom::d2::prim::vect::{
    Vect
};

use crate::geom::d2::prim::rect::{
    Rectangle
};

use crate::ops::{
    HAdd,
    HSub
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D triangle defined by 3 points "0" "1" "2"
#[derive(Debug)]
pub struct PTri<A>(pub Vect<A>, pub Vect<A>, pub Vect<A>);

/// 2D triangle defined by 1 position vector "pos" and 2 direction vectors "dir.0" "dir.1"
#[derive(Debug)]
pub struct VTri<A> {
    pub pos : Vect<A>,
    pub dir : (Vect<A>, Vect<A>)
}

/// 2D triangle defined by 1 position vector and 2 direction vectors where direction vectors are orthogonal
#[derive(Debug)]
pub struct OVTri<A> {
    tri : VTri<A>
}

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A> PTri<A> {
    fn new(a : Vect<A>, b : Vect<A>, c : Vect<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}

impl<A> VTri<A> {
    fn new(pos : Vect<A>, dir0 : Vect<A>, dir1 : Vect<A>) -> VTri<A> {
        VTri{pos : pos, dir : (dir0, dir1)}
    }
}

impl<A> OVTri<A> {
    pub fn new_unchecked(tri : VTri<A>) -> OVTri<A> {
        OVTri{tri : tri}
    }

    pub fn pos(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.pos.clone()
    }

    pub fn dir0(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.dir.0.clone()
    }

    pub fn dir1(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.dir.1.clone()
    }

    pub fn dirs(&self) -> (Vect<A>, Vect<A>) 
    where A : Clone
    {
        self.tri.dir.clone()
    }
}

//  TRAITS --------------------------------------------------------------------------------------------------------------------------------

pub trait Triangle<A> {
    fn points(&self) -> [Vect<A>; 3];
}

//  TRAIT IMPLS ---------------------------------------------------------------------------------------------------------------------------

impl<A, B : Rectangle<A>> From<&B> for OVTri<A> {
    fn from(rect : &B) -> Self {
        let (pos, dir) = rect.span();
        let tri = VTri::new(pos, dir.0, dir.1);

        OVTri::new_unchecked(tri)
    }
}

impl<A : Clone + HSub> Triangle<A> for PTri<A> {
    fn points(&self) -> [Vect<A>; 3] {
        [self.0.clone(), self.1.clone(), self.2.clone()]
    }
}

impl<A : Clone + HAdd> Triangle<A> for VTri<A> {
    fn points(&self) -> [Vect<A>; 3] {
        let a = self.pos.clone();
        let b = self.pos.clone() + self.dir.0.clone();
        let c = self.pos.clone() + self.dir.1.clone();

        [a, b, c]
    }
}