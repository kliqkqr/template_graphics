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

impl Zero for u32 {
    fn zero() -> u32 {
        0u32
    }
}

impl Zero for u64 {
    fn zero() -> u64 {
        0u64
    }
}

impl Zero for usize {
    fn zero() -> usize {
        0usize
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

impl Zero for isize {
    fn zero() -> isize {
        0isize
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

impl One for u32 {
    fn one() -> u32 {
        1u32
    }
}

impl One for u64 {
    fn one() -> u64 {
        1u64
    }
}

impl One for usize {
    fn one() -> usize {
        1usize
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

impl One for isize {
    fn one() -> isize {
        1isize
    }
}