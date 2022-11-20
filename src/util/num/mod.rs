use std::marker::{
    Copy
};

use std::ops::{
    Neg
};

pub trait Num : num_traits::Num + Copy + Neg<Output = Self> {

}

impl<A : num_traits::Num + Copy + Neg<Output = Self>> Num for A {

}