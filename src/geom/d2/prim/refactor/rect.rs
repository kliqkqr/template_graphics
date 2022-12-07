use std::marker::{
    Copy 
};

use crate::geom::d2::prim::refactor::vect::{
    Vector 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv,
    HNeg
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

impl<V : Vector> PRect<V> {
    /// create new PRect of 4 points "a" "b" "c" "d" without any checks
    pub fn new_unchecked(a : V, b : V, c : V, d : V) -> PRect<V> {
        PRect{a : a, b : b, c : c, d : d}
    }
}

impl<V : Vector> VRect<V> {
    /// create new VRect of 1 position vector "a" and 3 directioni vectors "ab" "ac" "ad" without any checks
    pub fn new_unchecked(a : V, ab : V, ac : V, ad : V) -> VRect<V> {
        VRect{a : a, ab : ab, ac : ac, ad : ad}
    }
}

pub trait Rectangle {
    /// the value type of vector
    type Val  : Copy;
    /// the vector type of Self
    type Vect : Vector<Val = Self::Val>;
    /// type that owns it vectors returned by methods
    type Own  : Rectangle<Vect = <Self::Vect as Vector>::Own, Own = Self::Own>;

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

    /// add vector to triangle points (translation)
    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HAdd;

    /// sub vector from triangle points (translation)
    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HSub;

    /// mul vector to triangle points
    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HMul;

    /// div vector from triangle points
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

    /// checks if rectangle contains points with dot product 
    /// 
    /// rectangle should contain direction vectors ab / ac for optimal performance
    /// 
    /// from: https://math.stackexchange.com/questions/190111/how-to-check-if-a-point-is-inside-a-rectangle
    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool 
    where Self::Val : Zero + HAdd + HSub + HMul + HNeg + HPOrd
    {   
        let ab = self.ab();
        let ac = self.ac();
        let ap = pnt.sub(self.a());

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ac = ap.dot(&ac);

        let ab_check = Self::Val::zero() <= ap_dot_ab && ap_dot_ab <= ab.dot(&ab);
        let ac_check = Self::Val::zero() <= ap_dot_ac && ap_dot_ac <= ac.dot(&ac);

        ab_check && ac_check
    } 
}

impl<'a, Rect : Rectangle> Rectangle for &'a Rect {
    type Val  = Rect::Val;
    type Vect = Rect::Vect;
    type Own  = Rect::Own;

    fn of<R : Rectangle<Val = Rect::Val>>(rect : R) -> Rect::Own {
        Rect::of(rect)
    }

    fn a(&self) -> <Rect::Vect as Vector>::Own {
        Rect::a(self)
    }

    fn b(&self) -> <Rect::Vect as Vector>::Own {
        Rect::b(self)
    }

    fn c(&self) -> <Rect::Vect as Vector>::Own {
        Rect::c(self)
    }

    fn d(&self) -> <Rect::Vect as Vector>::Own {
        Rect::d(self)
    }

    fn ab(&self) -> <Rect::Vect as Vector>::Own {
        Rect::ab(self)
    }

    fn ac(&self) -> <Rect::Vect as Vector>::Own {
        Rect::ac(self)
    }

    fn ad(&self) -> <Rect::Vect as Vector>::Own {
        Rect::ad(self)
    }

    fn add<V : Vector<Val = Rect::Val>>(&self, vect : V) -> Rect::Own
    where Rect::Val : HAdd 
    {
        Rect::add(self, vect)
    }

    fn sub<V : Vector<Val = Rect::Val>>(&self, vect : V) -> Rect::Own
    where Rect::Val : HSub 
    {
        Rect::sub(self, vect)
    }

    fn mul<V : Vector<Val = Rect::Val>>(&self, vect : V) -> Rect::Own
    where Rect::Val : HMul 
    {
        Rect::mul(self, vect)
    }

    fn div<V : Vector<Val = Rect::Val>>(&self, vect : V) -> Rect::Own
    where Rect::Val : HDiv 
    {
        Rect::div(self, vect)
    }

    fn vadd(&self, val : Rect::Val) -> Rect::Own
    where Rect::Val : HAdd 
    {
        Rect::vadd(self, val)
    }

    fn vsub(&self, val : Rect::Val) -> Rect::Own
    where Rect::Val : HSub 
    {
        Rect::vsub(self, val)
    }

    fn vmul(&self, val : Rect::Val) -> Rect::Own
    where Rect::Val : HMul 
    {
        Rect::vmul(self, val)
    }

    fn vdiv(&self, val : Rect::Val) -> Rect::Own
    where Rect::Val : HDiv 
    {
        Rect::vdiv(self, val)
    }
}

impl<Vect : Vector> Rectangle for PRect<Vect> 
where Vect::Val : HSub
{
    type Val  = Vect::Val;
    type Vect = Vect;
    type Own  = PRect<Vect::Own>;

    fn of<R : Rectangle<Val = Vect::Val>>(rect : R) -> PRect<Vect::Own> {
        let [a, b, c, d] = rect.pnts();

        let a = Vect::of(a);
        let b = Vect::of(b);
        let c = Vect::of(c);
        let d = Vect::of(d);
    
        PRect::new_unchecked(Vect::of(a), Vect::of(b), c, d)
    }

    fn a(&self) -> Vect::Own {
        Vect::of(&self.a)
    }

    fn b(&self) -> Vect::Own {
        Vect::of(&self.b)
    }

    fn c(&self) -> Vect::Own {
        Vect::of(&self.c)
    }

    fn d(&self) -> Vect::Own {
        Vect::of(&self.d)
    }

    fn ab(&self) -> Vect::Own {
        self.b.sub(&self.a)
    }

    fn ac(&self) -> Vect::Own {
        self.c.sub(&self.a)
    }

    fn ad(&self) -> Vect::Own {
        self.d.sub(&self.a)
    }

    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);
        let b = self.b.add(&vect);
        let c = self.c.add(&vect);
        let d = self.d.add(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);
        let b = self.b.sub(&vect);
        let c = self.c.sub(&vect);
        let d = self.d.sub(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.mul(&vect);
        let b = self.b.mul(&vect);
        let c = self.c.mul(&vect);
        let d = self.d.mul(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.div(&vect);
        let b = self.b.div(&vect);
        let c = self.c.div(&vect);
        let d = self.d.div(&vect);

        PRect::new_unchecked(a, b, c, d)
    }

    fn vadd(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);
        let b = self.b.vadd(val);
        let c = self.c.vadd(val);
        let d = self.d.vadd(val);

        PRect::new_unchecked(a, b, c, d)
    }

    fn vsub(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);
        let b = self.b.vsub(val);
        let c = self.c.vsub(val);
        let d = self.d.vsub(val);

        PRect::new_unchecked(a, b, c, d)
    }

    fn vmul(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let b = self.b.vmul(val);
        let c = self.c.vmul(val);
        let d = self.d.vmul(val);

        PRect::new_unchecked(a, b, c, d)
    }

    fn vdiv(&self, val : Vect::Val) -> PRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let b = self.b.vdiv(val);
        let c = self.c.vdiv(val);
        let d = self.d.vdiv(val);

        PRect::new_unchecked(a, b, c, d)
    }
}

impl<Vect : Vector> Rectangle for VRect<Vect> 
where Vect::Val : HAdd
{
    type Val  = Vect::Val;
    type Vect = Vect;
    type Own  = VRect<Vect::Own>;

    fn of<R : Rectangle<Val = Vect::Val>>(rect : R) -> VRect<Vect::Own> {
        let a  = Vect::of(rect.a());
        let ab = Vect::of(rect.ab());
        let ac = Vect::of(rect.ac());
        let ad = Vect::of(rect.ad()); 

        VRect::new_unchecked(a, ab, ac, ad)
    }

    fn a(&self) -> Vect::Own {
        Vect::of(&self.a)
    }

    fn b(&self) -> Vect::Own {
        self.a.add(&self.ab)
    }

    fn c(&self) -> Vect::Own {
        self.a.add(&self.ac)
    }

    fn d(&self) -> Vect::Own {
        self.a.add(&self.ad)
    }

    fn ab(&self) -> Vect::Own {
        Vect::of(&self.ab)
    }

    fn ac(&self) -> Vect::Own {
        Vect::of(&self.ac)
    }

    fn ad(&self) -> Vect::Own {
        Vect::of(&self.ad)
    }

    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(vect);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(vect);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a  = self.a.mul(&vect);
        let ab = self.ab.mul(&vect);
        let ac = self.ac.mul(&vect);
        let ad = self.ad.mul(&vect);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a  = self.a.div(&vect);
        let ab = self.ab.div(&vect);
        let ac = self.ac.div(&vect);
        let ad = self.ad.div(&vect);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    fn vadd(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    fn vsub(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);

        VRect::new_unchecked(a, self.ab(), self.ac(), self.ad())
    }

    fn vmul(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let ab = self.ab.vmul(val);
        let ac = self.ac.vmul(val);
        let ad = self.ad.vmul(val);

        VRect::new_unchecked(a, ab, ac, ad)
    }

    fn vdiv(&self, val : Vect::Val) -> VRect<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let ab = self.ab.vdiv(val);
        let ac = self.ac.vdiv(val);
        let ad = self.ad.vdiv(val);

        VRect::new_unchecked(a, ab, ac, ad)
    }
}
