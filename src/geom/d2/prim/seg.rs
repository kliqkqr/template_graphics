use std::marker::{
    Copy
};

use crate::geom::d2::prim::vect::{
    Vector
};

use crate::num::{
    Zero,
    One
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv
};

use crate::rel::{
    HPEq,
    HPOrd
};

/// 2D line segment defined by 2 points "a" "b"
pub struct PSeg<V : Vector> {
    a : V,
    b : V 
}

/// 2D line segment defined by 1 position vector "a" and 1 direction vector "ab"
pub struct VSeg<V : Vector> {
    a  : V,
    ab : V 
}

impl<V : Vector> PSeg<V> {
    /// create new PSeg of 2 points "a" "b"
    pub fn new(a : V, b : V) -> PSeg<V> {
        PSeg{a : a, b : b}
    }
}

impl<V : Vector> VSeg<V> {
    /// create new VSeg of 1 position vector "a" and 1 direction vector "ab"
    pub fn new(a : V, ab : V) -> VSeg<V> {
        VSeg{a : a, ab : ab}
    }
}

pub trait Segment {
    /// the value type of vector
    type Val  : Copy;
    /// the vector type of Self
    type Vect : Vector<Val = Self::Val>;
    /// type that owns it vectors returned by methods
    type Own  : Segment<Vect = <Self::Vect as Vector>::Own, Own = Self::Own>;

    /// create new line segment of other line segment
    fn of<S : Segment<Val = Self::Val>>(seg : S) -> Self::Own;

    /// first point of line segment
    fn a(&self) -> <Self::Vect as Vector>::Own;

    /// second point of line
    fn b(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of line segment with ab() = b() - a()
    fn ab(&self) -> <Self::Vect as Vector>::Own;

    /// add vector to line segment points (translation)
    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HAdd;

    /// sub vector from line segment points (translation)
    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HSub;

    /// mul vector to line segment points
    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HMul;

    /// div vector from line segment points
    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HDiv;

    /// mul vector values with value (scaling)
    fn vadd(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HAdd;

    // div vector values with value (scaling)
    fn vsub(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HSub;

    /// mul vector values with value (scaling)
    fn vmul(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HMul;

    // div vector values with value (scaling)
    fn vdiv(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HDiv;

    /// points of line segment with [a(), b()] = pnts()
    fn pnts(&self) -> [<Self::Vect as Vector>::Own; 2] {
        [self.a(), self.b()]
    }
    
    /// vectors of line segment with [a(), ab()] = vects()
    fn vects(&self) -> [<Self::Vect as Vector>::Own; 2] {
        [self.a(), self.ab()]
    }

    /// optional intersection between to line segments without epsilon zero checks
    ///
    /// a + r * b = c + s * d where r, s in \[0, 1\]
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec<S : Segment<Val = Self::Val>>(&self, other : S) -> Option<<Self::Vect as Vector>::Own>
    where Self::Val : Zero + One + HAdd + HSub + HMul + HDiv + HPEq + HPOrd
    {   
        let [s_a, s_ab] = self.vects();
        let [o_a, o_ab] = other.vects();

        let div = o_ab.indep_det(&s_ab)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let zero = Self::Val::zero();
        let one  = Self::Val::one();

        match r.inc_in(zero, one) {
            true  => {
                let p = s_a.add(s_ab.vmul(r));
                Some(p)
            },
            false => None   
        }
    }

    /// optional intersection between to line segments with epsilon zero checks
    ///
    /// a + r * b = c + s * d where r, s in \[0, 1\]
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec_eps<S : Segment<Val = Self::Val>>(&self, other : S, eps : Self::Val) -> Option<<Self::Vect as Vector>::Own> 
    where Self::Val : Zero + One + HAdd + HSub + HMul + HDiv + HPOrd
    {   
        let [s_a, s_ab] = self.vects();
        let [o_a, o_ab] = other.vects();

        let div = o_ab.indep_det_eps(&s_ab, eps)?;
        let r   = o_ab.det(&o_a) + s_a.det(&o_ab) / div;

        let zero = Self::Val::zero();
        let one  = Self::Val::one();

        match r.inc_in(zero, one) {
            true  => {
                let p = s_a.add(s_ab.vmul(r));
                Some(p)
            },
            false => None   
        }
    }
}

impl<'a, Seg : Segment> Segment for &'a Seg {
    type Val  = Seg::Val;
    type Vect = Seg::Vect;
    type Own  = Seg::Own;

    fn of<S : Segment<Val = Self::Val>>(seg : S) -> Self::Own {
        Seg::of(seg)
    }

    fn a(&self) -> <Self::Vect as Vector>::Own {
        Seg::a(self)
    }

    fn b(&self) -> <Self::Vect as Vector>::Own {
        Seg::a(self)
    }

    fn ab(&self) -> <Self::Vect as Vector>::Own {
        Seg::ab(self)
    }

    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own
    where Self::Val : HAdd 
    {
        Seg::add(self, vect)
    }

    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HSub 
    {
        Seg::sub(self, vect)
    }

    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HMul 
    {
        Seg::mul(self, vect)
    }

    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Own 
    where Self::Val : HDiv 
    {
        Seg::div(self, vect)
    }

    fn vadd(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HAdd 
    {
        Seg::vadd(self, val)
    }

    fn vsub(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HSub 
    {
        Seg::vsub(self, val)
    }

    fn vmul(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HMul 
    {
        Seg::vmul(self, val)
    }

    fn vdiv(&self, val : Self::Val) -> Self::Own 
    where Self::Val : HDiv 
    {
        Seg::vdiv(self, val)
    }
}

impl<Vect : Vector> Segment for PSeg<Vect> 
where Vect::Val : HAdd + HSub
{
    type Val  = Vect::Val;
    type Vect = Vect;
    type Own  = PSeg<Vect::Own>;

    fn of<S : Segment<Val = Vect::Val>>(seg : S) -> PSeg<Vect::Own> {
        let a = Vect::of(seg.a());
        let b = Vect::of(seg.b());

        PSeg::new(a, b)
    }

    fn a(&self) -> Vect::Own {
        let x = self.a.x();
        let y = self.a.y();

        Vect::of((x, y))
    }

    fn b(&self) -> Vect::Own {
        let x = self.b.x();
        let y = self.b.y();

        Vect::of((x, y))
    }

    fn ab(&self) -> Vect::Own {
        self.b.sub(&self.a)
    }

    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PSeg<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);
        let b = self.b.add(&vect);

        PSeg::new(a, b)
    }

    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PSeg<Vect::Own> 
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);
        let b = self.b.sub(&vect);

        PSeg::new(a, b)
    }

    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PSeg<Vect::Own> 
    where Vect::Val : HMul 
    {
        let a = self.a.mul(&vect);
        let b = self.b.mul(&vect);

        PSeg::new(a, b)
    }

    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PSeg<Vect::Own> 
    where Vect::Val : HDiv 
    {
        let a = self.a.div(&vect);
        let b = self.b.div(&vect);

        PSeg::new(a, b)
    }

    fn vadd(&self, val : Vect::Val) -> PSeg<Vect::Own> 
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);
        let b = self.b.vadd(val);

        PSeg::new(a, b)
    }

    fn vsub(&self, val : Vect::Val) -> PSeg<Vect::Own> 
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);
        let b = self.b.vsub(val);

        PSeg::new(a, b)
    }

    fn vmul(&self, val : Vect::Val) -> PSeg<Vect::Own> 
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let b = self.b.vmul(val);

        PSeg::new(a, b)
    }

    fn vdiv(&self, val : Vect::Val) -> PSeg<Vect::Own> 
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let b = self.b.vdiv(val);

        PSeg::new(a, b)
    }
}

impl<Vect : Vector> Segment for VSeg<Vect> 
where Vect::Val : HAdd + HSub
{
    type Val  = Vect::Val;
    type Vect = Vect;
    type Own  = VSeg<Vect::Own>;

    fn of<S : Segment<Val = Vect::Val>>(seg : S) -> VSeg<Vect::Own> {
        let a  = Vect::of(seg.a());
        let ab = Vect::of(seg.ab());

        VSeg::new(a, ab)
    }

    fn a(&self) -> Vect::Own {
        let x = self.a.x();
        let y = self.a.y();

        Vect::of((x, y))
    }

    fn b(&self) -> Vect::Own {
        self.a.add(&self.ab)
    }

    fn ab(&self) -> Vect::Own {
        let x = self.ab.x();
        let y = self.ab.y();
        
        Vect::of((x, y))
    }

    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VSeg<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);

        VSeg::new(a, self.ab())
    }

    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VSeg<Vect::Own> 
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);

        VSeg::new(a, self.ab())
    }

    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VSeg<Vect::Own> 
    where Vect::Val : HMul 
    {
        let a  = self.a.mul(&vect);
        let ab = self.ab.mul(&vect);

        VSeg::new(a, ab)
    }

    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VSeg<Vect::Own> 
    where Vect::Val : HDiv 
    {
        let a  = self.a.div(&vect);
        let ab = self.ab.div(&vect);

        VSeg::new(a, ab)
    }

    fn vadd(&self, val : Vect::Val) -> VSeg<Vect::Own> 
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);

        VSeg::new(a, self.ab())
    }

    fn vsub(&self, val : Vect::Val) -> VSeg<Vect::Own> 
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);

        VSeg::new(a, self.ab())
    }

    fn vmul(&self, val : Vect::Val) -> VSeg<Vect::Own> 
    where Vect::Val : HMul 
    {
        let a  = self.a.vmul(val);
        let ab = self.ab.vmul(val);

        VSeg::new(a, ab)
    }

    fn vdiv(&self, val : Vect::Val) -> VSeg<Vect::Own> 
    where Vect::Val : HDiv 
    {
        let a  = self.a.vdiv(val);
        let ab = self.ab.vdiv(val);

        VSeg::new(a, ab)
    }
}

