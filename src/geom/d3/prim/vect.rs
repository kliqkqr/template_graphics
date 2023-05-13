use std::marker::{
    Copy 
};

use crate::num::{
    Float 
};

use crate::rel::{
    HPEq 
};

pub type Vect<Val> = (Val, Val, Val);

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

    /// third value of vector
    fn z(&self) -> Self::Val;

    /// checks equality componentwise
    fn equal<V : Vector<Val = Self::Val>>(&self, other : V) -> bool  
    where Self::Val : HPEq
    {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }

    fn rotate_y(&self, angle : Self::Val) -> Self::Own 
    where Self::Val : Float
    {
        let sin = angle.sin();
        let cos = angle.cos();

        let x =  self.x() * cos + self.z() * sin;
        let y =  self.y();
        let z = -self.x() * sin + self.z() * cos;

        Self::of((x, y, z))
    }

    fn rotate_z(&self, angle : Self::Val) -> Self::Own 
    where Self::Val : Float
    {
        let sin = angle.sin();
        let cos = angle.cos();

        let x = self.x() * cos - self.y() * sin;
        let y = self.x() * sin + self.y() * cos;
        let z = self.z();

        Self::of((x, y, z))
    }
}

impl<'a, Vect : Vector> Vector for &'a Vect {
    type Val = Vect::Val;
    type Own = Vect::Own;

    fn of<V : Vector<Val = Self::Val>>(vect : V) -> Self::Own {
        Vect::of(vect)
    }

    fn x(&self) -> Self::Val {
        Vect::x(self)
    }

    fn y(&self) -> Self::Val {
        Vect::y(self)
    }

    fn z(&self) -> Self::Val {
        Vect::z(self)
    }
}

impl<Val : Copy> Vector for Vect<Val> {
    type Val = Val;
    type Own = Vect<Val>;

    fn of<V : Vector<Val = Self::Val>>(vect : V) -> Self::Own {
        (vect.x(), vect.y(), vect.z())
    }

    fn x(&self) -> Self::Val {
        self.0
    }

    fn y(&self) -> Self::Val {
        self.1
    }

    fn z(&self) -> Self::Val {
        self.2
    }
}