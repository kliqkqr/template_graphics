use std::clone::{
    Clone
};

use std::marker::{
    Copy
};

use crate::ops::{
    HMul
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D vector with 2 values "0" "1"
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Vect<A>(pub A, pub A, pub A);

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A : Copy> Vect<A> {
    pub fn new(x : A, y : A, z : A) -> Vect<A> {
        Vect(x, y, z)
    }
    
    pub fn x(&self) -> A {
        self.0
    }

    pub fn y(&self) -> A {
        self.1
    }

    pub fn z(&self) -> A {
        self.2
    }

    pub fn vmul(&self, val : A) -> Vect<A> 
    where A : HMul
    {
        Vect::new(self.x() * val, self.y() * val, self.z() * val)
    }
}

//  TRAIT IMPLS ---------------------------------------------------------------------------------------------------------------------------

impl<A : Copy> Clone for Vect<A> {
    fn clone(&self) -> Vect<A> {
        Vect::new(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

