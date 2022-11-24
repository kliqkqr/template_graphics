// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPORTS 
// ----------------------------------------------------------------------------------------------------------------------------------------

use std::clone::{
    Clone
};

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TYPES 
// ----------------------------------------------------------------------------------------------------------------------------------------

// Vectors

/// 2D vector with 2 values "0" "1"
#[derive(Debug)]
pub struct Vect<A>(pub A, pub A, pub A);

// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

impl<A> Vect<A> {
    pub fn new(x : A, y : A, z : A) -> Vect<A> {
        Vect(x, y, z)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   STD TRAIT IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

impl<A : Clone> Clone for Vect<A> {
    fn clone(&self) -> Vect<A> {
        Vect::new(self.0.clone(), self.1.clone(), self.2.clone())
    }
}