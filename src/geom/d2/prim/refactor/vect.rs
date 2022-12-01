use std::marker::{
    Copy
};

use std::option::{
    Option
};

use crate::conv::{
    To,
    Ref
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
    HDiv,
    HNeg
};

use crate::rel::{
    HPEq,
    HPOrd
};

pub trait Vector {
    /// value type of vector
    type Val : Copy;

    /// type created by Vector::new so that Vector can be implemented for references 
    /// 
    /// should always be Self for value types and *Self for references
    type Out : Vector<Val = Self::Val, Out = Self::Out>;

    /// create new vector of 2 values
    fn of_vals(x : Self::Val, y : Self::Val) -> Self::Out;

    /// first value of vector
    fn x(&self) -> Self::Val;

    /// second value of vector
    fn y(&self) -> Self::Val;

    /// create new vector of 1 value
    fn of_val(val : Self::Val) -> Self::Out {
        Self::of_vals(val, val)
    }

    /// create new vector where values are 0
    fn zero() -> Self::Out
    where Self::Val : Zero 
    {   
        Self::of_val(Self::Val::zero())
    }    

    /// create new vector where values are 1
    fn one() -> Self::Out
    where Self::Val : One
    {
        Self::of_val(Self::Val::one())
    }
    
    /// both values of vector
    fn vals(&self) -> (Self::Val, Self::Val) {
        (self.x(), self.y())
    }

    /// length of float vector
    fn len(&self) -> Self::Val 
    where Self::Val : Float
    {
        let (x, y) = self.vals();
        (x * x + y * y).sqrt()
    }    

    /// length of vector converting values to some float
    fn len_as<F : Float>(&self) -> F 
    where Self::Val : To<F> + HAdd + HMul
    {
        let (x, y) = self.vals();
        (x * x + y * y).to().sqrt()
    }

    /// add two vectors componentwise
    fn add<V : Vector<Val = Self::Val>, R : Ref<V>>(&self, other : R) -> Self::Out
    where Self::Val : HAdd 
    {
        let x = self.x() + other.r().x();
        let y = self.x() + other.r().y();

        Self::of_vals(x, y)
    }

    /// add two vectors componentwise
    fn sub<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Out
    where Self::Val : HSub 
    {
        let x = self.x() - other.r().x();
        let y = self.y() - other.r().y();

        Self::of_vals(x, y)
    }

    /// add two vectors componentwise
    fn mul<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Out
    where Self::Val : HMul 
    {
        let x = self.x() * other.r().x();
        let y = self.y() * other.r().y();

        Self::of_vals(x, y)
    }

    /// add two vectors componentwise
    fn div<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Out
    where Self::Val : HDiv 
    {
        let x = self.x() / other.r().x();
        let y = self.y() / other.r().y();

        Self::of_vals(x, y)
    }  

    /// negate vector componentwise
    fn neg(&self) -> Self::Out
    where Self::Val : HNeg 
    {
        let x = -self.x();
        let y = -self.y();

        Self::of_vals(x, y)
    }

    /// add value to vector componentwise
    fn vadd(&self, val : Self::Val) -> Self::Out 
    where Self::Val : HAdd 
    {
        let x = self.x() + val;
        let y = self.y() + val;

        Self::of_vals(x, y)
    }

    /// sub value from vector componentwise
    fn vsub(&self, val : Self::Val) -> Self::Out 
    where Self::Val : HSub 
    {
        let x = self.x() - val;
        let y = self.y() - val;

        Self::of_vals(x, y)
    }

    /// mul value to vector componentwise
    fn vmul(&self, val : Self::Val) -> Self::Out 
    where Self::Val : HMul 
    {
        let x = self.x() * val;
        let y = self.y() * val;

        Self::of_vals(x, y)
    }

    /// sub value from vector componentwise
    fn vdiv(&self, val : Self::Val) -> Self::Out 
    where Self::Val : HDiv 
    {
        let x = self.x() / val;
        let y = self.y() / val;

        Self::of_vals(x, y)
    }

    /// map function over vector values and convert Self to some other Vector
    fn map<A, F : Fn(Self::Val) -> A, V : Vector<Val = A, Out = V>>(&self, func : F) -> V {
        let x = func(self.x());
        let y = func(self.y());

        V::of_vals(x, y)
    }

    /// determinant of two vectors a b : (a.x * b.y - a.y * b.x) 
    fn det<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Val
    where Self::Val : HSub + HMul 
    {
        self.x() * other.y() - self.y() * other.x()
    }

    /// determinant of two vectors a b : (a.x * b.y - a.y * b.x) 
    fn dot<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Val
    where Self::Val : HAdd + HMul 
    {
        self.x() * other.x() + self.y() * other.y()
    }

    /// componentwise min of two vectors
    fn min<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Out
    where Self::Val : HPOrd 
    {
        let x = self.x().min(other.x());
        let y = self.y().min(other.y());

        Self::of_vals(x, y)
    }

    /// componentwise max of two vectors
    fn max<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Out
    where Self::Val : HPOrd 
    {
        let x = self.x().max(other.x());
        let y = self.y().max(other.y());

        Self::of_vals(x, y)
    }

    /// rotate vector 90°
    fn orth_l(&self) -> Self::Out 
    where Self::Val : HNeg
    {
        Self::of_vals(-self.y(), self.x())
    }

    /// rotate vector 270°
    fn orth_r(&self) -> Self::Out 
    where Self::Val : HNeg
    {
        Self::of_vals(self.y(), -self.x())
    }

    /// checks linear dependency of two vectors by checking if determinant is in range [0 - eps, 0 + eps]
    fn lin_dep<V : Vector<Val = Self::Val>>(&self, other : V) -> bool
    where Self::Val : Zero + HAdd + HSub + HMul + HPEq
    {
        let zero = Self::Val::zero();
        let det  = self.det(other);

        det == zero
    }

    /// checks linear dependency of two vectors by checking if determinant is in range [0 - eps, 0 + eps]
    fn lin_dep_eps<V : Vector<Val = Self::Val>>(&self, other : V, eps : Self::Val) -> bool
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd
    {
        let zero = Self::Val::zero();
        let det  = self.det(other);

        det.inc_in(zero - eps, zero + eps)
    }

    /// returns determinant if two vectors are linear independent
    fn indep_det<V : Vector<Val = Self::Val>>(&self, other : V) -> Option<Self::Val> 
    where Self::Val : Zero + HAdd + HSub + HMul + HPEq
    {   
        let zero = Self::Val::zero();
        let det  = self.det(other); 

        match det == zero {
            false => None,
            true  => Some(det)
        }
    } 

    /// returns determinant if two vectors are linear independent
    fn indep_det_eps<V : Vector<Val = Self::Val>>(&self, other : V, eps : Self::Val) -> Option<Self::Val> 
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd
    {   
        let zero = Self::Val::zero();
        let det  = self.det(other); 

        match det.inc_in(zero - eps, zero + eps) {
            false => None,
            true  => Some(det)
        }
    }    
}

impl<'a, A : Vector> Vector for &'a A {
    type Val = A::Val;

    type Out = A::Out;

    fn of_vals(x : Self::Val, y : Self::Val) -> Self::Out {
        A::of_vals(x, y)
    }

    fn x(&self) -> Self::Val {
        A::x(self)
    }

    fn y(&self) -> Self::Val {
        A::y(self)
    }
}

impl<A : Copy> Vector for (A, A) {
    type Val = A;
    type Out = (A, A);

    fn of_vals(x : Self::Val, y : Self::Val) -> Self::Out {
        (x, y)
    }

    fn x(&self) -> Self::Val {
        self.0
    }

    fn y(&self) -> Self::Val {
        self.1
    }
} 

impl<A : Copy> Vector for [A; 2] {
    type Val = A;
    type Out = [A; 2];

    fn of_vals(x : Self::Val, y : Self::Val) -> Self::Out {
        [x, y]
    }

    fn x(&self) -> Self::Val {
        self[0]
    }

    fn y(&self) -> Self::Val {
        self[1]
    }
}

