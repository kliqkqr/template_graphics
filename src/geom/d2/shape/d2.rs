use std::marker::{
    Copy 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

pub struct Bounds<Vect : Vector> {
    min : Vect,
    max : Vect 
}

impl<Vect : Vector> Bounds<Vect> {
    pub fn new_unchecked(min : Vect, max : Vect) -> Bounds<Vect> {
        Bounds{min : min, max : max}
    }
}

pub trait Shape {
    type Val  : Copy;
    type Vect : Vector<Val = Self::Val>;

    fn bounds(&self) -> Bounds<<Self::Vect as Vector>::Own>;
    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool;
}

impl<'a, Sh : Shape> Shape for &'a Sh {
    type Val  = Sh::Val;
    type Vect = Sh::Vect;

    fn bounds(&self) -> Bounds<<Self::Vect as Vector>::Own> {
        Sh::bounds(self)
    }

    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool {
        Sh::contains(self, pnt)
    }
}