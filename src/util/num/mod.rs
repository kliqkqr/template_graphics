use std::cmp::{
    PartialEq,
    PartialOrd
};

use std::marker::{
    Copy
};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg
};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Num : Sized 
              + Copy
              + Zero 
              + One
              + Add<Output = Self> 
              + Sub<Output = Self> 
              + Mul<Output = Self>
              + Div<Output = Self>
              + Rem<Output = Self>
              + Neg<Output = Self>
              + PartialEq
              + PartialOrd
{   
    /// exclusive test if in range
    fn exc_in(self, min : Self, max : Self) -> bool {
        min < self && self < max
    }

    /// inclusive test if in range
    fn inc_in(self, min : Self, max : Self) -> bool {
        min <= self && self <= max
    }
}

impl<A : Sized 
       + Copy
       + Zero 
       + One
       + Add<Output = Self> 
       + Sub<Output = Self> 
       + Mul<Output = Self>
       + Div<Output = Self>
       + Rem<Output = Self>
       + Neg<Output = Self>
       + PartialEq
       + PartialOrd>  
Num for A {}

impl Zero for f32 {
    fn zero() -> f32 {
        0f32
    }
}

impl Zero for f64 {
    fn zero() -> f64 {
        0f64
    }
}

impl Zero for i32 {
    fn zero() -> i32 {
        0i32
    }
}

impl Zero for i64 {
    fn zero() -> i64 {
        0i64
    }
}

impl One for f32 {
    fn one() -> f32 {
        1f32
    }
}

impl One for f64 {
    fn one() -> f64 {
        1f64
    }
}

impl One for i32 {
    fn one() -> i32 {
        1i32
    }
}

impl One for i64 {
    fn one() -> i64 {
        1i64
    }
}
