use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg
};

/// homogenous add where Output = Self
pub trait HAdd<Rhs = Self> : Sized + Add<Rhs, Output = Self> {}

/// homogenous sub where Output = Self
pub trait HSub<Rhs = Self> : Sized + Sub<Rhs, Output = Self> {}

/// homogenous mul where Output = Self
pub trait HMul<Rhs = Self> : Sized + Mul<Rhs, Output = Self> {}

/// homogenous div where Output = Self
pub trait HDiv<Rhs = Self> : Sized + Div<Rhs, Output = Self> {}

/// homogenous rem where Output = Self
pub trait HRem<Rhs = Self> : Sized + Rem<Rhs, Output = Self> {}

/// homogenous neg where Output = Self
pub trait HNeg : Sized + Neg<Output = Self> {}

impl<A : Add<B, Output = A>, B> HAdd<B> for A {}
impl<A : Sub<B, Output = A>, B> HSub<B> for A {}
impl<A : Mul<B, Output = A>, B> HMul<B> for A {}
impl<A : Div<B, Output = A>, B> HDiv<B> for A {}
impl<A : Rem<B, Output = A>, B> HRem<B> for A {}
impl<A : Neg<Output = A>> HNeg for A {}