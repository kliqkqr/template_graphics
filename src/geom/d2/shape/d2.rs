use std::marker::{
    Copy 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv
};

pub struct Bounds<Vect : Vector> {
    min : Vect,
    max : Vect 
}

impl<Vect : Vector> Bounds<Vect> {
    pub fn new_unchecked(min : Vect, max : Vect) -> Bounds<Vect> {
        Bounds{min : min, max : max}
    }
}

pub trait Shape {
    type Val  : Copy;
    type Vect : Vector<Val = Self::Val>;
    type Own  : Shape<Val = Self::Val, Vect = <Self::Vect as Vector>::Own, Own = Self::Own>;

    /// add vector to shape points (translation)
    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HAdd;

    /// sub vector from shape points (translation)
    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HSub;

    /// mul vector to shape points
    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HMul;

    /// div vector from shape points
    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HDiv;

    /// add vector values with value
    fn vadd(&self, val : Self::Val) -> Self::Own
    where Self::Val : HAdd;

    // sub vector values with value
    fn vsub(&self, val : Self::Val) -> Self::Own
    where Self::Val : HSub;

    /// mul vector values with value (scaling)
    fn vmul(&self, val : Self::Val) -> Self::Own
    where Self::Val : HMul;

    // div vector values with value (scaling)
    fn vdiv(&self, val : Self::Val) -> Self::Own
    where Self::Val : HDiv;

    /// bounding axe aligned rectangle of shape
    fn bounds(&self) -> Bounds<<Self::Vect as Vector>::Own>;

    /// checks if shape contains point
    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool;
}

macro_rules! impl_shape {
    ($Self:ty) => {
        fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
        where Self::Val : HAdd 
        {
            <$Self>::add(self, vect)
        }
    
        fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
        where Self::Val : HSub 
        {
            <$Self>::sub(self, vect)
        }
    
        fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
        where Self::Val : HMul 
        {
            <$Self>::mul(self, vect)
        }
    
        fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
        where Self::Val : HDiv 
        {
            <$Self>::div(self, vect)
        }
    
        fn vadd(&self, val : Self::Val) -> Self::Own
        where Self::Val : HAdd 
        {
            <$Self>::vadd(self, val)
        }
    
        fn vsub(&self, val : Self::Val) -> Self::Own
        where Self::Val : HSub 
        {
            <$Self>::vsub(self, val)
        }
    
        fn vmul(&self, val : Self::Val) -> Self::Own
        where Self::Val : HMul 
        {
            <$Self>::vmul(self, val)
        }
    
        fn vdiv(&self, val : Self::Val) -> Self::Own
        where Self::Val : HDiv 
        {
            <$Self>::vdiv(self, val)
        }

        fn bounds(&self) -> Bounds<<Self::Vect as Vector>::Own> {
            <$Self>::bounds(self)
        }
    
        fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool {
            <$Self>::contains(self, pnt)
        }
    };
}

pub(crate) use impl_shape;

impl<'a, Sh : Shape> Shape for &'a Sh {
    type Val  = Sh::Val;
    type Vect = Sh::Vect;
    type Own  = Sh::Own;

    impl_shape!(Sh);
}