use crate::geom::d2::shape::d2::{
    Bounds,
    Shape,
    impl_shape
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv,
};

use crate::num::{
    Zero, 
}; 

use crate::rel::{
    HPOrd 
};

/// 2D rectangle defined by 4 points "a" "b" "c" "d"
pub struct PRect<V : Vector> {
    a : V,
    b : V,
    c : V,
    d : V
}

/// 2D rectangle defined by 1 position vector "a" and 3 direction vectors "ab" "ac" "ad"
pub struct VRect<V : Vector> {
    a  : V,
    ab : V,
    ac : V,
    ad : V
}

impl<Vect : Vector> PRect<Vect> {
    // General methods

    /// create new PRect of 4 points "a" "b" "c" "d" without any checks
    pub fn new_unchecked(a : Vect, b : Vect, c : Vect, d : Vect) -> PRect<Vect> {
        PRect{a : a, b : b, c : c, d : d}
    }

    // Shape methods

    /// add vector to rectangle points (translation)
    pub fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);
        let b = self.b.add(&vect);
        let c = self.c.add(&vect);
        let d = self.d.add(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    /// sub vector from rectangle points (translation)
    pub fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);
        let b = self.b.sub(&vect);
        let c = self.c.sub(&vect);
        let d = self.d.sub(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    /// mul vector to rectangle points
    pub fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.mul(&vect);
        let b = self.b.mul(&vect);
        let c = self.c.mul(&vect);
        let d = self.d.mul(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    /// div vector from rectangle points
    pub fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.div(&vect);
        let b = self.b.div(&vect);
        let c = self.c.div(&vect);
        let d = self.d.div(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    /// add vector values with value
    pub fn vadd(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);
        let b = self.b.vadd(val);
        let c = self.c.vadd(val);
        let d = self.d.vadd(val);

        PRect::new_unchecked(a, b, c, d)
    }

    // sub vector values with value
    pub fn vsub(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);
        let b = self.b.vsub(val);
        let c = self.c.vsub(val);
        let d = self.d.vsub(val);

        PRect::new_unchecked(a, b, c, d)
    }

    /// mul vector values with value (scaling)
    pub fn vmul(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let b = self.b.vmul(val);
        let c = self.c.vmul(val);
        let d = self.d.vmul(val);

        PRect::new_unchecked(a, b, c, d)
    }

    // div vector values with value (scaling)
    pub fn vdiv(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let b = self.b.vdiv(val);
        let c = self.c.vdiv(val);
        let d = self.d.vdiv(val);

        PRect::new_unchecked(a, b, c, d)
    }

    /// bounding axe aligned rectangle
    pub fn bounds(&self) -> Bounds<Vect::Own> 
    where Vect::Val : HPOrd
    {
        let min = self.a.min(self.b.min(self.c.min(&self.d)));
        let max = self.a.max(self.b.max(self.c.max(&self.d)));

        Bounds::new_unchecked(min, max)
    }

    /// check if rectangle contains point
    /// 
    /// slightly slower then implicit line equation check but more precise
    pub fn contains<V : Vector<Val = Vect::Val>>(&self, pnt : V) -> bool 
    where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
    {
        let ab = self.ab();
        let ad = self.ad();
        let ap = pnt.sub(self.a());

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ad = ap.dot(&ad);

        let zero = Vect::Val::zero();

        let ab_check = zero <= ap_dot_ab && ap_dot_ab <= ab.dot(&ab);
        let ad_check = zero <= ap_dot_ad && ap_dot_ad <= ad.dot(&ad);

        ab_check && ad_check
    }

    // Rectangle methods

    /// create new rectangle of other rectangle
    pub fn of<R : Rectangle<Val = Vect::Val>>(rect : R) -> PRect<Vect::Own> {
        let [a, b, c, d] = rect.pnts();

        let a = Vect::of(a);
        let b = Vect::of(b);
        let c = Vect::of(c);
        let d = Vect::of(d);
    
        PRect::new_unchecked(a, b, c, d)
    }

    /// first point of rectangle
    pub fn a(&self) -> Vect::Own {
        Vect::of(&self.a)
    }

    /// second point of rectangle
    pub fn b(&self) -> Vect::Own {
        Vect::of(&self.b)
    }

    /// third point of rectangle
    pub fn c(&self) -> Vect::Own {
        Vect::of(&self.c)
    }

    /// fourth point of rectangle
    pub fn d(&self) -> Vect::Own {
        Vect::of(&self.d)
    }

    /// direction vector of rectangle with ab() = b() - a()
    pub fn ab(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.b.sub(&self.a)
    }

    /// direction vector of rectangle with ac() = c() - a()
    pub fn ac(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.c.sub(&self.a)
    }

    /// direction vector of rectangle with ad() = d() - a()
    pub fn ad(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.d.sub(&self.a)
    }

    /// points of rectangle with pnts() = \[a(), b(), c(), d()\]
    pub fn pnts(&self) -> [Vect::Own; 4] {
        [self.a(), self.b(), self.c(), self.d()]
    }

    /// vectors of rectangle with vects() = \[a(), ab(), ac(), ad()\]
    pub fn vects(&self) -> [Vect::Own; 4] 
    where Vect::Val : HSub
    {
        [self.a(), self.ab(), self.ac(), self.ad()]
    }

    /// direction vectors of rectangle with dirs() = \[ab(), ac(), ad()\]
    pub fn dirs(&self) -> [Vect::Own; 3] 
    where Vect::Val : HSub
    {
        [self.ab(), self.ac(), self.ad()]
    }
}

impl<Vect : Vector> VRect<Vect> {
    // General methods

    /// create new VRect of 1 position vector "a" and 3 directioni vectors "ab" "ac" "ad" without any checks
    pub fn new_unchecked(a : Vect, ab : Vect, ac : Vect, ad : Vect) -> VRect<Vect> {
        VRect{a : a, ab : ab, ac : ac, ad : ad}
    }

    // Shape methods

    /// add vector to rectangle points (translation)
    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(vect);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    /// sub vector from rectangle points (translation)
    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(vect);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    /// mul vector to rectangle points
    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a  = self.a.mul(&vect);
        let ab = self.ab.mul(&vect);
        let ac = self.ac.mul(&vect);
        let ad = self.ad.mul(&vect);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    /// div vector from rectangle points
    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a  = self.a.div(&vect);
        let ab = self.ab.div(&vect);
        let ac = self.ac.div(&vect);
        let ad = self.ad.div(&vect);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    /// add vector values with value
    fn vadd(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    // sub vector values with value
    fn vsub(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    /// mul vector values with value (scaling)
    fn vmul(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let ab = self.ab.vmul(val);
        let ac = self.ac.vmul(val);
        let ad = self.ad.vmul(val);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    // div vector values with value (scaling)
    fn vdiv(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let ab = self.ab.vdiv(val);
        let ac = self.ac.vdiv(val);
        let ad = self.ad.vdiv(val);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    /// bounding axe aligned rectangle
    pub fn bounds(&self) -> Bounds<Vect::Own> 
    where Vect::Val : HAdd + HPOrd
    {
        let b = self.b();
        let c = self.c();
        let d = self.d();

        let min = self.a.min(b.min(c.min(&d)));
        let max = self.a.max(b.max(c.max(&d)));

        Bounds::new_unchecked(min, max)
    }

    /// check if rectangle contains point
    /// 
    /// slightly slower then implicit line equation check but more precise
    pub fn contains<V : Vector<Val = Vect::Val>>(&self, pnt : V) -> bool 
    where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
    {
        let ab = self.ab();
        let ad = self.ad();
        let ap = pnt.sub(self.a());

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ad = ap.dot(&ad);

        let zero = Vect::Val::zero();

        let ab_check = zero <= ap_dot_ab && ap_dot_ab <= ab.dot(&ab);
        let ad_check = zero <= ap_dot_ad && ap_dot_ad <= ad.dot(&ad);

        ab_check && ad_check
    }

    // Rectangle methods

    /// create new rectangle of other rectangle
    pub fn of<R : Rectangle<Val = Vect::Val>>(rect : R) -> VRect<Vect::Own> {
        let a  = Vect::of(rect.a());
        let ab = Vect::of(rect.ab());
        let ac = Vect::of(rect.ac());
        let ad = Vect::of(rect.ad()); 

        VRect::new_unchecked(a, ab, ac, ad)
    }

    /// first point of rectangle
    pub fn a(&self) -> Vect::Own {
        Vect::of(&self.a)
    }

    /// second point of rectangle
    pub fn b(&self) -> Vect::Own 
    where Vect::Val : HAdd
    {
        self.a.add(&self.ab)
    }

    /// third point of rectangle
    pub fn c(&self) -> Vect::Own 
    where Vect::Val : HAdd
    {
        self.a.add(&self.ac)
    }

    /// fourth point of rectangle
    pub fn d(&self) -> Vect::Own 
    where Vect::Val : HAdd
    {
        self.a.add(&self.ad)
    }

    /// direction vector of rectangle with ab() = b() - a()
    pub fn ab(&self) -> Vect::Own {
        Vect::of(&self.ab)
    }

    /// direction vector of rectangle with ac() = c() - a()
    pub fn ac(&self) -> Vect::Own {
        Vect::of(&self.ac)
    }

    /// direction vector of rectangle with ad() = d() - a()
    pub fn ad(&self) -> Vect::Own {
        Vect::of(&self.ad)
    }

    /// points of rectangle with pnts() = \[a(), b(), c(), d()\]
    pub fn pnts(&self) -> [Vect::Own; 4] 
    where Vect::Val : HAdd
    {
        [self.a(), self.b(), self.c(), self.d()]
    }

    /// vectors of rectangle with vects() = \[a(), ab(), ac(), ad()\]
    pub fn vects(&self) -> [Vect::Own; 4] {
        [self.a(), self.ab(), self.ac(), self.ad()]
    }

    /// direction vectors of rectangle with dirs() = \[ab(), ac(), ad()\]
    pub fn dirs(&self) -> [Vect::Own; 3] {
        [self.ab(), self.ac(), self.ad()]
    }
}

pub trait Rectangle : Shape {
    /// create new rectangle of other rectangle
    fn of<R : Rectangle<Val = Self::Val>>(rect : R) -> Self::Own;

    /// first point of rectangle
    fn a(&self) -> <Self::Vect as Vector>::Own;

    /// second point of rectangle
    fn b(&self) -> <Self::Vect as Vector>::Own;

    /// third point of rectangle
    fn c(&self) -> <Self::Vect as Vector>::Own;

    /// fourth point of rectangle
    fn d(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of rectangle with ab() = b() - a()
    fn ab(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of rectangle with ac() = c() - a()
    fn ac(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of rectangle with ad() = d() - a()
    fn ad(&self) -> <Self::Vect as Vector>::Own;

    /// points of rectangle with pnts() = \[a(), b(), c(), d()\]
    fn pnts(&self) -> [<Self::Vect as Vector>::Own; 4] {
        [self.a(), self.b(), self.c(), self.d()]
    }

    /// vectors of rectangle with vects() = \[a(), ab(), ac(), ad()\]
    fn vects(&self) -> [<Self::Vect as Vector>::Own; 4] {
        [self.a(), self.ab(), self.ac(), self.ad()]
    }

    /// direction vectors of rectangle with dirs() = \[ab(), ac(), ad()\]
    fn dirs(&self) -> [<Self::Vect as Vector>::Own; 3] {
        [self.ab(), self.ac(), self.ad()]
    }

}

macro_rules! impl_rectangle {
    ($Self:ty) => {
        fn of<R : Rectangle<Val = Self::Val>>(rect : R) -> Self::Own {
            <$Self>::of(rect)
        }
    
        fn a(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::a(self)
        }
    
        fn b(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::b(self)
        }
    
        fn c(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::c(self)
        }
    
        fn d(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::d(self)
        }
    
        fn ab(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::ab(self)
        }
    
        fn ac(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::ac(self)
        }
    
        fn ad(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::ad(self)
        }
    };
}

pub(crate) use impl_rectangle;

impl<Vect : Vector> Shape for PRect<Vect> 
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd 
{
    type Val  = Vect::Val;
    type Vect = Vect::Own;
    type Own  = PRect<Vect::Own>;

    impl_shape!(PRect<Vect>);
}

impl<Vect : Vector> Shape for VRect<Vect> 
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
{
    type Val  = Vect::Val;
    type Vect = Vect::Own;
    type Own  = VRect<Vect::Own>;

    impl_shape!(VRect<Vect>);
}

impl<'a, Rect : Rectangle> Rectangle for &'a Rect {
    impl_rectangle!(Rect);
}

impl<Vect : Vector> Rectangle for PRect<Vect> 
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd 
{
    impl_rectangle!(PRect<Vect>);
}

impl<Vect : Vector> Rectangle for VRect<Vect> 
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
{
    impl_rectangle!(VRect<Vect>);
}
