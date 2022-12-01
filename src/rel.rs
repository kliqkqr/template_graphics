use std::cmp::{
    PartialEq,
    PartialOrd
};

/// trait for partial equality
pub trait PEq<Rhs = Self> : PartialEq<Rhs> {}

/// trait for partial order
pub trait POrd<Rhs = Self> : PartialOrd<Rhs> {}


/// extension for homogenous partial orders
#[deprecated]
pub trait ExtHPOrd: Sized + POrd<Self> {
    /// minimum of two numbers that returns self if equal
    fn min(self, other : Self) -> Self {
        match self <= other {
            true  => self,
            false => other 
        }
    }

    /// maximum of two numbers that returns self if equal
    fn max(self, other : Self) -> Self {
        match self >= other {
            true  => self,
            false => other
        }
    }

    /// exclusive test if in range
    fn exc_in(self, min : Self, max : Self) -> bool {
        min < self && self < max
    }

    /// inclusive test if in range
    fn inc_in(self, min : Self, max : Self) -> bool {
        min <= self && self <= max
    }
}

/// trait for homogenous partial equality
pub trait HPEq : Sized + PEq<Self> {}

/// trait for homogenous partial order
pub trait HPOrd : Sized + POrd<Self> {
    /// minimum of two numbers that returns self if equal
    fn min(self, other : Self) -> Self {
        match self <= other {
            true  => self,
            false => other 
        }
    }

    /// maximum of two numbers that returns self if equal
    fn max(self, other : Self) -> Self {
        match self >= other {
            true  => self,
            false => other
        }
    }

    /// exclusive test if in range
    fn exc_in(self, min : Self, max : Self) -> bool {
        min < self && self < max
    }

    /// inclusive test if in range
    fn inc_in(self, min : Self, max : Self) -> bool {
        min <= self && self <= max
    }
}

impl<A : PartialEq<Rhs>, Rhs> PEq<Rhs> for A {}

impl<A : PEq> HPEq for A {}

impl<A : PartialOrd<Rhs>, Rhs> POrd<Rhs> for A {} 

impl<A : POrd> HPOrd for A {}

impl<A : POrd<A>> ExtHPOrd for A {}