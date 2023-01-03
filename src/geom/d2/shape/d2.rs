
use std::marker::{
    Copy 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::num::{
    Zero
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv
};

use crate::rel::{
    HPOrd
};

#[derive(Debug)]
pub struct Bounds<Vect : Vector> {
    start : Vect,
    end   : Vect 
}

impl<Vect : Vector> Bounds<Vect> {
    pub fn new_unchecked(start : Vect, end : Vect) -> Bounds<Vect> {
        Bounds{start : start, end : end}
    }

    pub fn new(a : Vect, b : Vect) -> Bounds<Vect::Own> 
    where Vect::Val : HPOrd
    {
        let start = a.min(&b);
        let end   = a.max(&b);

        Bounds::new_unchecked(start, end)
    }

    pub fn with_zero_unchecked(end : Vect) -> Bounds<Vect::Own> 
    where Vect::Val : Zero
    {   
        let start = Vect::zero();
        let end   = Vect::of(end);

        Bounds::new_unchecked(start, end)
    }

    pub fn start(&self) -> Vect::Own {
        self.start.to()
    }

    pub fn end(&self) -> Vect::Own {
        self.end.to()
    }

    pub fn size(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.end.sub(&self.start)
    }

    pub fn map_unchecked<V : Vector, F : Fn(&Vect) -> V>(&self, func : F) -> Bounds<V> {
        let start = func(&self.start);
        let end   = func(&self.end);

        Bounds::new_unchecked(start, end)
    }

    pub fn map<V : Vector, F : Fn(&Vect) -> V>(&self, func : F) -> Bounds<V::Own> 
    where V::Val : HPOrd
    {
        let start = func(&self.start);
        let end   = func(&self.end);

        Bounds::new(start, end)
    }

    pub fn vmap<V : Vector<Own = V>, F : Fn(Vect::Val) -> V::Val>(&self, func : F) -> Bounds<V> 
    where V::Val : HPOrd
    {
        let start = self.start.map::<V, _>(&func);
        let end   = self.end.map::<V, _>(&func);

        Bounds::new(start, end)
    }

    pub fn clamp<V : Vector<Val = Vect::Val>>(&self, bounds : Bounds<V>) -> Bounds<Vect::Own>
    where Vect::Val : HPOrd
    {
        let start = self.start.max(bounds.start());
        let end   = self.end.min(bounds.end());

        Bounds::new_unchecked(start, end)
    }
}

pub trait Shape {
    type Val  : Copy;
    type Vect : Vector<Val = Self::Val, Own = Self::Vect>;
    type Own  : Shape<Val = Self::Val, Vect = Self::Vect, Own = Self::Own>;

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

    /// bounding axis aligned rectangle of shape
    fn bounds(&self) -> Bounds<Self::Vect>;

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

        fn bounds(&self) -> Bounds<Self::Vect> {
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