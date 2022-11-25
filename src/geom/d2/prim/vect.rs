use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg
};

use crate::conv::{
    To
};

use crate::num::{
    Zero,
    One,
    Float
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HNeg,
};

use crate::rel::{
    POrd,
    ExtHPOrd
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D vector with 2 values "0" "1"
#[derive(Debug)]
pub struct Vect<A>(pub A, pub A);

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A> Vect<A> {
    // static methods

    /// create new vector from 2 values
    pub fn new(x : A, y : A) -> Vect<A> {
        Vect(x, y)
    }

    /// create new vector where values are 0
    pub fn zero() -> Vect<A>
    where A : Zero 
    {   
        Vect::new(A::zero(), A::zero())
    }    

    /// create new vector where values are 1
    pub fn one() -> Vect<A>
    where A : One
    {
        Vect::new(A::one(), A::one())
    }

    // instance methods

    /// map function over vector values
    pub fn map<B : Fn(&A) -> C, C>(&self, func : B) -> Vect<C> {
        let x = func(&self.0);
        let y = func(&self.1); 

        Vect::new(x, y)
    }

    /// length of float vector
    pub fn len(&self) -> A
    where A : Clone + HAdd + HMul + Float
    {   
        (self.0.clone() * self.0.clone() + self.1.clone() * self.1.clone()).sqrt()
    }

    /// length of vector converting values to some float
    pub fn len_as<B>(&self) -> B 
    where A : Clone + HAdd + HMul + To<B>, B : Float
    {
        (self.0.clone() * self.0.clone() + self.1.clone() * self.1.clone()).to().sqrt()
    }

    /// determinant of two vectors a b : (a0 * b1 - a1 * b0) 
    pub fn det(&self, other : &Vect<A>) -> A 
    where A : Clone + HSub + HMul
    {
        self.0.clone() * other.1.clone() - self.1.clone() * other.0.clone()
    }

    /// dot product of two vectors
    pub fn dot(&self, other : &Vect<A>) -> A 
    where A : Clone + HAdd + HMul
    {
        self.0.clone() * other.0.clone() + self.1.clone() * other.1.clone()
    }

    /// componentwise min of two vectors
    pub fn min(&self, other : &Vect<A>) -> Vect<A> 
    where A : Clone + POrd
    {
        let x = self.0.clone().min(other.0.clone());
        let y = self.1.clone().min(other.1.clone());
        
        Vect::new(x, y)
    }   

    /// componentwise max of two vectors
    pub fn max(&self, other : &Vect<A>) -> Vect<A> 
    where A : Clone + POrd
    {
        let x = self.0.clone().max(other.0.clone());
        let y = self.1.clone().max(other.1.clone());
        
        Vect::new(x, y)
    }  

    /// rotate vector 90°
    pub fn orth_l(&self) -> Vect<A> 
    where A : Clone + HNeg
    {
        Vect::new(-self.1.clone(), self.0.clone())
    }

    /// rotate vector 270°
    pub fn orth_r(&self) -> Vect<A> 
    where A : Clone + HNeg
    {
        Vect::new(self.1.clone(), -self.0.clone())
    }

    /// checks linear dependency of two vectors by checking if determinant is in range [0 - eps, 0 + eps]
    pub fn lin_dep(&self, other : &Vect<A>, eps : A) -> bool
    where A : Clone + Zero + HAdd + HSub + HMul + POrd
    {
        self.det(other).inc_in(A::zero() - eps.clone(), A::zero() + eps)
    }

    /// returns determinant if two vectors are linear independent
    pub fn lin_dep_det(&self, other : &Vect<A>, eps : A) -> Option<A> 
    where A : Clone + Zero + HAdd + HSub + HMul + POrd
    {
        let det = self.det(other); 

        match det.clone().inc_in(A::zero() - eps.clone(), A::zero() + eps) {
            false => None,
            true  => Some(det)
        }
    }    
}

//  TRAIT IMPLS ---------------------------------------------------------------------------------------------------------------------------

// Clone

impl<A : Clone> Clone for Vect<A> {
    fn clone(&self) -> Self {
        Vect::new(self.0.clone(), self.1.clone())
    }
}

// Add

impl<A : Add<Output = A>> Add for Vect<A> {
    type Output = Vect<A>;

    fn add(self, other : Self) -> Vect<A> {
        Vect::new(self.0 + other.0, self.1 + other.1)
    }
}

impl<A : Add<Output = A> + Clone> Add<A> for Vect<A> {
    type Output = Vect<A>;

    fn add(self, other : A) -> Vect<A> {
        Vect::new(self.0 + other.clone(), self.1 + other)
    }
}

impl<'a, 'b, A : Clone + Add<Output = A>> Add<&'b Vect<A>> for &'a Vect<A> {
    type Output = Vect<A>;

    fn add(self, other : &'b Vect<A>) -> Vect<A> {
        Vect::new(self.0.clone() + other.0.clone(), self.1.clone() + other.1.clone())
    }
}

impl<'a, A : Clone + Add<Output = A>> Add<A> for &'a Vect<A> {
    type Output = Vect<A>;

    fn add(self, a : A) -> Vect<A> {
        Vect::new(self.0.clone() + a.clone(), self.1.clone() + a)
    }
}

// Sub

impl<A : Sub<Output = A>> Sub for Vect<A> {
    type Output = Vect<A>;

    fn sub(self, other : Self) -> Vect<A> {
        Vect::new(self.0 - other.0, self.1 - other.1)
    }
}

impl<A : Sub<Output = A> + Clone> Sub<A> for Vect<A> {
    type Output = Vect<A>;

    fn sub(self, other : A) -> Vect<A> {
        Vect::new(self.0 - other.clone(), self.1 - other)
    }
}

impl<'a, 'b, A : Clone + Sub<Output = A>> Sub<&'b Vect<A>> for &'a Vect<A> {
    type Output = Vect<A>;

    fn sub(self, other : &'b Vect<A>) -> Vect<A> {
        Vect::new(self.0.clone() - other.0.clone(), self.1.clone() - other.1.clone())
    }
}

impl<'a, A : Clone + Sub<Output = A>> Sub<A> for &'a Vect<A> {
    type Output = Vect<A>;

    fn sub(self, a : A) -> Vect<A> {
        Vect::new(self.0.clone() - a.clone(), self.1.clone() - a)      
    }
}

// Mul

impl<A : Mul<Output = A>> Mul for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : Self) -> Vect<A> {
        Vect::new(self.0 * other.0, self.1 * other.1)
    }
}

impl<A : Mul<Output = A> + Clone> Mul<A> for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : A) -> Vect<A> {
        Vect::new(self.0 * other.clone(), self.1 * other)
    }
}

impl<'a, 'b, A : Clone + Mul<Output = A>> Mul<&'b Vect<A>> for &'a Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : &'b Vect<A>) -> Vect<A> {
        Vect::new(self.0.clone() * other.0.clone(), self.1.clone() * other.1.clone())
    }
}

impl<'a, A : Clone + Mul<Output = A>> Mul<A> for &'a Vect<A> {
    type Output = Vect<A>;

    fn mul(self, a : A) -> Vect<A> {
        Vect::new(self.0.clone() * a.clone(), self.1.clone() * a)      
    }
}

// Div

impl<A : Div<Output = A>> Div for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : Self) -> Vect<A> {
        Vect::new(self.0 / other.0, self.1 / other.1)
    }
}

impl<A : Div<Output = A> + Clone> Div<A> for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : A) -> Vect<A> {
        Vect::new(self.0 / other.clone(), self.1 / other)
    }
}

impl<'a, 'b, A : Clone + Div<Output = A>> Div<&'b Vect<A>> for &'a Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : &'b Vect<A>) -> Vect<A> {
        Vect::new(self.0.clone() / other.0.clone(), self.1.clone() / other.1.clone())
    }
}

impl<'a, A : Clone + Div<Output = A>> Div<A> for &'a Vect<A> {
    type Output = Vect<A>;

    fn div(self, a : A) -> Vect<A> {
        Vect::new(self.0.clone() / a.clone(), self.1.clone() / a)      
    }
}

// Neg

impl<A : Neg<Output = A>> Neg for Vect<A> {
    type Output = Vect<A>;

    fn neg(self) -> Vect<A> {
        Vect::new(-self.0, -self.1)
    }
}

impl<'a, A : Clone + Neg<Output = A>> Neg for &'a Vect<A> {
    type Output = Vect<A>;

    fn neg(self) -> Vect<A> {
        Vect::new(-self.0.clone(), -self.1.clone())
    }
}