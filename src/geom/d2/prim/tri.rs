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

/// 2D triangle defined by 3 points "a" "b" "c"
#[derive(Debug)]
pub struct PTri<A> {
    a : Vect<A>,
    b : Vect<A>,
    c : Vect<A>
}

/// 2D triangle defined by 1 position vector "pos" and 2 direction vectors "dirs.0" "dirs.1"
#[derive(Debug)]
pub struct VTri<A> {
    pos  : Vect<A>,
    dirs : (Vect<A>, Vect<A>),
}

/// 2D triangle defined by 1 position vector and 2 direction vectors where direction vectors are orthogonal
#[derive(Debug)]
pub struct OVTri<A> {
    tri : VTri<A>
}

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A> PTri<A> {
    pub fn new(a : Vect<A>, b : Vect<A>, c : Vect<A>) -> PTri<A> {
        PTri{a : a, b : b, c : c}
    }

    pub fn a(&self) -> &Vect<A> {
        &self.a
    }

    pub fn b(&self) -> &Vect<A> {
        &self.b
    }

    pub fn c(&self) -> &Vect<A> {
        &self.c
    }
}

impl<A> VTri<A> {
    pub fn new(pos : Vect<A>, dir0 : Vect<A>, dir1 : Vect<A>) -> VTri<A> {
        VTri{pos : pos, dirs : (dir0, dir1)}
    }

    pub fn pos(&self) -> &Vect<A> {
        &self.pos
    }

    pub fn dir0(&self) -> &Vect<A> {
        &self.dirs.0
    }

    pub fn dir1(&self) -> &Vect<A> {
        &self.dirs.1
    }

    pub fn dirs(&self) -> &(Vect<A>, Vect<A>) {
        &self.dirs
    }
}

impl<A> OVTri<A> {
    pub fn new_unchecked(tri : VTri<A>) -> OVTri<A> {
        OVTri{tri : tri}
    }

    pub fn pos(&self) -> &Vect<A> {
        &self.tri.pos
    }

    pub fn dir0(&self) -> &Vect<A> {
        &self.tri.dirs.0
    }

    pub fn dir1(&self) -> &Vect<A> {
        &self.tri.dirs.1
    }

    pub fn dirs(&self) -> &(Vect<A>, Vect<A>) {
        &self.tri.dirs
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
        [self.a().clone(), self.b().clone(), self.c().clone()]
    }
}

impl<A : Clone + HAdd> Triangle<A> for VTri<A> {
    fn points(&self) -> [Vect<A>; 3] {
        let (ab, ac) = self.dirs();

        let a = self.pos().clone();
        let b = &a + ab;
        let c = &a + ac;

        [a, b, c]
    }
}