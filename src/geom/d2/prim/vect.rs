use std::marker::{
    Copy
};

use std::option::{
    Option
};

use crate::conv::{
    To
};

use crate::num::{
    Zero,
    One,
    Two,
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

pub type Vect<Val> = (Val, Val);

pub trait Vector {
    /// value type of vector
    type Val : Copy;
    /// type that owns it values returned by methods
    type Own : Vector<Val = Self::Val, Own = Self::Own>;

    /// create new vector of other vector
    fn of<V : Vector<Val = Self::Val>>(vect : V) -> Self::Own;

    /// first value of vector
    fn x(&self) -> Self::Val;

    /// second value of vector
    fn y(&self) -> Self::Val;

    /// create new vector where values are 0
    fn zero() -> Self::Own
    where Self::Val : Zero 
    {       
        let zero = Self::Val::zero();

        Self::of((zero, zero))
    }    

    /// create new vector where values are 1
    fn one() -> Self::Own
    where Self::Val : One
    {
        let one = Self::Val::one();

        Self::of((one, one))
    }
    
    /// both values of vector
    fn vals(&self) -> [Self::Val; 2] {
        [self.x(), self.y()]
    }

    /// check if vectors are equal
    fn equal<V : Vector<Val = Self::Val>>(&self, other : V) -> bool 
    where Self::Val : HPEq
    {
        self.x() == other.x() && self.y() == other.y()
    }

    fn to<V>(&self) -> V 
    where Self : Sized,
          V    : Vector<Val = Self::Val, Own = V>
    {
        V::of(self)
    }

    /// length of float vector
    fn len(&self) -> Self::Val 
    where Self::Val : Float
    {
        let [x, y] = self.vals();
        (x * x + y * y).sqrt()
    }    

    /// length of vector converting values to some float
    fn len_as<F : Float>(&self) -> F 
    where Self::Val : To<F> + HAdd + HMul
    {
        let [x, y] = self.vals();
        (x * x + y * y).to().sqrt()
    }

    /// add two vectors componentwise
    fn add<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HAdd 
    {
        let x = self.x() + other.x();
        let y = self.y() + other.y();

        Self::of((x, y))
    } 

    /// add two vectors componentwise
    fn sub<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HSub 
    {
        let x = self.x() - other.x();
        let y = self.y() - other.y();

        Self::of((x, y))
    }

    /// add two vectors componentwise
    fn mul<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HMul 
    {
        let x = self.x() * other.x();
        let y = self.y() * other.y();

        Self::of((x, y))
    }

    /// add two vectors componentwise
    fn div<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HDiv 
    {
        let x = self.x() / other.x();
        let y = self.y() / other.y();

        Self::of((x, y))
    }  

    /// negate vector componentwise
    fn neg(&self) -> Self::Own
    where Self::Val : HNeg 
    {
        let x = -self.x();
        let y = -self.y();

        Self::of((x, y))
    }

    /// add value to vector componentwise
    fn vadd(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HAdd 
    {
        let x = self.x() + val;
        let y = self.y() + val;

        Self::of((x, y))
    }

    /// sub value from vector componentwise
    fn vsub(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HSub 
    {
        let x = self.x() - val;
        let y = self.y() - val;

        Self::of((x, y))
    }

    /// mul value to vector componentwise
    fn vmul(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HMul 
    {
        let x = self.x() * val;
        let y = self.y() * val;

        Self::of((x, y))
    }

    /// sub value from vector componentwise
    fn vdiv(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HDiv 
    {
        let x = self.x() / val;
        let y = self.y() / val;

        Self::of((x, y))
    }

    /// map function over vector values and convert Self to some other Vector
    fn map<V : Vector<Own = V>, F : Fn(Self::Val) -> V::Val>(&self, func : F) -> V {
        let x = func(self.x());
        let y = func(self.y());

        V::of((x, y))
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
    fn min<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HPOrd 
    {
        let x = self.x().min(other.x());
        let y = self.y().min(other.y());

        Self::of((x, y))
    }

    /// componentwise max of two vectors
    fn max<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Own
    where Self::Val : HPOrd 
    {
        let x = self.x().max(other.x());
        let y = self.y().max(other.y());

        Self::of((x, y))
    }

    /// rotate vector 90°
    fn orth_l(&self) -> Self::Own 
    where Self::Val : HNeg
    {
        Self::of((-self.y(), self.x()))
    }

    /// rotate vector 270°
    fn orth_r(&self) -> Self::Own 
    where Self::Val : HNeg
    {
        Self::of((self.y(), -self.x()))
    }

    /// checks linear dependency of two vectors by checking if determinant is 0
    fn lin_dep<V : Vector<Val = Self::Val>>(&self, other : V) -> bool
    where Self::Val : Zero + HAdd + HSub + HMul + HPEq
    {
        let zero = Self::Val::zero();
        let det  = self.det(other);

        det == zero
    }

    /// checks linear dependency of two vectors by checking if determinant is in [0 - eps, 0 + eps]
    fn lin_dep_eps<V : Vector<Val = Self::Val>>(&self, other : V, eps : Self::Val) -> bool
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd
    {
        let zero = Self::Val::zero();
        let det  = self.det(other);

        det.inc_in(zero - eps, zero + eps)
    }

    /// returns determinant if two vectors are linear independent (det == 0)
    fn indep_det<V : Vector<Val = Self::Val>>(&self, other : V) -> Option<Self::Val> 
    where Self::Val : Zero + HAdd + HSub + HMul + HPEq
    {   
        let zero = Self::Val::zero();
        let det  = self.det(other); 

        match det == zero {
            false => Some(det),
            true  => None
        }
    } 

    /// returns determinant if two vectors are linear independent (det in [0 - eps, 0 + eps])
    fn indep_det_eps<V : Vector<Val = Self::Val>>(&self, other : V, eps : Self::Val) -> Option<Self::Val> 
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd
    {   
        let zero = Self::Val::zero();
        let det  = self.det(other); 

        match det.inc_in(zero - eps, zero + eps) {
            false => Some(det),
            true  => None
        }
    }    

    /// checks if other vector is left or right of self
    fn left<V : Vector<Val = Self::Val>>(&self, other : V) -> bool 
    where Self::Val : Zero + HSub + HMul + HPOrd
    {    
        self.det(other) > Self::Val::zero()
    } 

    /// angle between two vectors of floats
    fn angle<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Val 
    where Self::Val : Float 
    {
        let dot = self.dot(&other);
        let div = self.len() * other.len();

        (dot / div).acos()
    }

    /// left angle of self vector to other vector
    fn angle_l<V : Vector<Val = Self::Val>>(&self, other : V) -> Self::Val 
    where Self::Val : Zero + Two + Float + HSub + HMul + HPOrd
    {
        let angle = self.angle(&other);

        match self.left(other) {
            false => Self::Val::two() * Self::Val::pi() - angle,
            true  => angle 
        }
    } 
}

impl<'a, Vect : Vector> Vector for &'a Vect {
    type Val = Vect::Val;
    type Own = Vect::Own;

    fn of<V : Vector<Val = Vect::Val>>(vect : V) -> Vect::Own {
        Vect::of(vect)
    }

    fn x(&self) -> Self::Val {
        Vect::x(self)
    }

    fn y(&self) -> Self::Val {
        Vect::y(self)
    }
}

impl<Val : Copy> Vector for (Val, Val) {
    type Val = Val;
    type Own = (Val, Val);

    fn of<V : Vector<Val = Val>>(vect : V) -> (Val, Val) {
        (vect.x(), vect.y())
    }

    fn x(&self) -> Val {
        self.0
    }

    fn y(&self) -> Val {
        self.1
    }
} 
