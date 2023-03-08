use std::marker::{
    Copy 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv,
    HRem,
    HNeg
};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Two {
    fn two() -> Self;
}

pub trait Integer : Copy + HAdd + HSub + HMul + HDiv + HRem + HNeg {

}

pub trait Float : Copy + Zero + One + Two + HAdd + HSub + HMul + HDiv + HRem + HNeg {
    fn sqrt(self) -> Self;

    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn acos(self) -> Self;

    fn pi() -> Self;
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

impl Two for f32 {
    fn two() -> f32 {
        2f32
    }
}

impl Two for f64 {
    fn two() -> f64 {
        2f64
    }
}

impl Two for u32 {
    fn two() -> u32 {
        2u32
    }
}

impl Two for u64 {
    fn two() -> u64 {
        2u64
    }
}

impl Two for usize {
    fn two() -> usize {
        2usize
    }
}

impl Two for i32 {
    fn two() -> i32 {
        2i32
    }
}

impl Two for i64 {
    fn two() -> i64 {
        2i64
    }
}

impl Two for isize {
    fn two() -> isize {
        2isize
    }
}

impl Float for f32 {
    fn sqrt(self) -> f32 {
        f32::sqrt(self)
    }

    fn sin(self) -> Self {
        f32::sin(self)
    }

    fn cos(self) -> Self {
        f32::cos(self)
    }

    fn acos(self) -> Self {
        f32::acos(self)
    }

    fn pi() -> Self {
        std::f32::consts::PI
    }
}

impl Float for f64 {
    fn sqrt(self) -> f64 {
        f64::sqrt(self)
    }

    fn sin(self) -> Self {
        f64::sin(self)
    }

    fn cos(self) -> Self {
        f64::cos(self)
    }

    fn acos(self) -> Self {
        f64::acos(self)
    }

    fn pi() -> Self {
        std::f64::consts::PI
    }
}