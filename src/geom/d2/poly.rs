use std::vec::{
    Vec
};

use crate::geom::d2::prim::vect::{
    Vector
};

use crate::geom::d2::shape::d2::{
    Bounds 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv
};

use crate::rel::{
    HPOrd
};

pub struct Poly<Vect : Vector> {
    vertices : Vec<Vect>
}

impl<Vect : Vector> Poly<Vect> {
    // General methods

    /// create new Polygon of vertices
    pub fn new(vertices : Vec<Vect>) -> Poly<Vect> {
        Poly{vertices : vertices}
    }

    pub fn vertices(&self) -> &Vec<Vect> {
        &self.vertices
    }

    pub fn fit_bounds<V : Vector<Val = Vect::Val>>(&self, bounds : Bounds<V>) -> Poly<Vect::Own> 
    where Vect::Val : HAdd + HSub + HMul + HDiv + HPOrd
    {
        let self_bounds = self.bounds();
        
        let ratios = bounds.size().div(self_bounds.size());
        let offset = bounds.start().sub(self_bounds.start());

        self.mul(ratios).add(offset)
    }

    // Shape methods

    pub fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> Poly<Vect::Own> 
    where Vect::Val : HAdd
    {
        let vertices = self.vertices().iter().map(|vertex| vertex.add(&vect)).collect();
        Poly::new(vertices)
    }

    pub fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> Poly<Vect::Own> 
    where Vect::Val : HMul
    {
        let vertices = self.vertices().iter().map(|vertex| vertex.mul(&vect)).collect();
        Poly::new(vertices)
    }

    pub fn bounds(&self) -> Bounds<Vect::Own> 
    where Vect::Val : HPOrd
    {
        let mut start = Vect::of(&self.vertices[0]);
        let mut end = Vect::of(&self.vertices[0]);

        for vertex in &self.vertices {
            start = start.min(vertex);
            end   = end.max(vertex);
        }

        Bounds::new_unchecked(start, end)
    }
} 