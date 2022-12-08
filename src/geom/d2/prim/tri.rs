use std::marker::{
    Copy 
};

use crate::geom::d2::shape::d2::{
    Bounds,
    Shape
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
    HDiv,
    HNeg
};

use crate::rel::{
    HPEq,
    HPOrd
};

/// 2D triangle defined by 3 points "a" "b" "c"
pub struct PTri<V : Vector> {
    a : V,
    b : V,
    c : V 
}

/// 2D triangle defined by 1 position vector "a" and 2 direction vectors "ab" "ac"
pub struct VTri<V : Vector> {
    a  : V,
    ab : V,
    ac : V 
}

impl<Vect : Vector> PTri<Vect> {
    /// create new PTri of 3 points "a" "b" "c"
    pub fn new(a : Vect, b : Vect, c : Vect) -> PTri<Vect> {
        PTri{a : a, b : b, c : c}
    }

    /// create new triangle of other triangle
    pub fn of<T : Triangle<Val = Vect::Val>>(tri : T) -> PTri<Vect::Own> {
        let a = Vect::of(tri.a());
        let b = Vect::of(tri.b());
        let c = Vect::of(tri.c());

        PTri::new(a, b, c)
    }

    /// first point of triangle
    pub fn a(&self) -> Vect::Own {
        let x = self.a.x();
        let y = self.a.y();

        Vect::of((x, y))
    }

    /// second point of triangle
    pub fn b(&self) -> Vect::Own {
        let x = self.b.x();
        let y = self.b.y();

        Vect::of((x, y))
    }

    /// second point of triangle
    pub fn c(&self) -> Vect::Own {
        let x = self.c.x();
        let y = self.c.y();

        Vect::of((x, y))
    }

    /// direction vector of triangle with ab() = b() - a()
    pub fn ab(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.b.sub(&self.a)
    }

    /// direction vector of triangle with ac() = c() - a()
    pub fn ac(&self) -> Vect::Own 
    where Vect::Val : HSub
    {
        self.c.sub(&self.a)
    }

    /// add vector to triangle points (translation)
    pub fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PTri<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);
        let b = self.b.add(&vect);
        let c = self.c.add(&vect);

        PTri::new(a, b, c)
    }

    /// sub vector from triangle points (translation)
    pub fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PTri<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);
        let b = self.b.sub(&vect);
        let c = self.c.sub(&vect);

        PTri::new(a, b, c)
    }

    /// mul vector to triangle points
    pub fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PTri<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.mul(&vect);
        let b = self.b.mul(&vect);
        let c = self.c.mul(&vect);

        PTri::new(a, b, c)
    }

    /// div vector from triangle points
    pub fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> PTri<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.div(&vect);
        let b = self.b.div(&vect);
        let c = self.c.div(&vect);

        PTri::new(a, b, c)
    }

    /// add vector values with value
    pub fn vadd(&self, val : Vect::Val) -> PTri<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);
        let b = self.b.vadd(val);
        let c = self.c.vadd(val);

        PTri::new(a, b, c)
    }

    // sub vector values with value
    pub fn vsub(&self, val : Vect::Val) -> PTri<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);
        let b = self.b.vsub(val);
        let c = self.c.vsub(val);

        PTri::new(a, b, c)
    }

    /// mul vector values with value (scaling)
    pub fn vmul(&self, val : Vect::Val) -> PTri<Vect::Own>
    where Vect::Val : HMul 
    {
        let a = self.a.vmul(val);
        let b = self.b.vmul(val);
        let c = self.c.vmul(val);

        PTri::new(a, b, c)
    }

    // div vector values with value (scaling)
    pub fn vdiv(&self, val : Vect::Val) -> PTri<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a = self.a.vdiv(val);
        let b = self.b.vdiv(val);
        let c = self.c.vdiv(val);

        PTri::new(a, b, c)
    }

    /// points of triangle with pnts() = \[a(), b(), c()\]
    pub fn pnts(&self) -> [Vect::Own; 3] {
        [self.a(), self.b(), self.c()]
    }

    /// vectors of triangle with vects() = \[a(), ab(), ac()\]
    pub fn vects(&self) -> [Vect::Own; 3] 
    where Vect::Val : HSub
    {
        [self.a(), self.ab(), self.ac()]
    }

    /// direction vectors of triangle with dirs() = \[ab(), ac()\]
    pub fn dirs(&self) -> [Vect::Own; 2] 
    where Vect::Val : HSub
    {
        [self.ab(), self.ac()]
    }

    /// check if triangle contains point with baryzentric coordinates
    /// 
    /// slightly slower then implicit line equation check but more precise
    pub fn contains<V : Vector<Val = Vect::Val>>(&self, pnt : V) -> bool 
    where Vect::Val : Zero + HSub + HMul + HPOrd
    {
        let zero = Vect::Val::zero();

        let [ab, ac] = self.dirs();
        let abc = ab.det(&ac);
    
        let start;
        let end;
    
        if abc < zero {
            start = abc;
            end   = zero;
        }
        else {
            start = zero;
            end   = abc;
        }
    
        let a = self.a();
        let b = self.b();
    
        let pa  = a.sub(&pnt);
        let pb  = b.sub(&pnt);
    
        let pab = pa.det(&pb);
    
        if !pab.inc_in(start, end) {
            return false
        }
    
        let c = self.c();
    
        let pc  = c.sub(&pnt);
    
        let pbc = pb.det(&pc);
    
        if !pbc.inc_in(start, end) {
            return false
        }
        
        let pca = pc.det(&pa);
    
        if !pca.inc_in(start, end) {
            return false
        }
    
        true
    }
}

impl<Vect : Vector> VTri<Vect> {
    /// create new VTri of 1 position vector "a" and 2 direction vectors "ab" "ac"
    pub fn new(a : Vect, ab : Vect, ac : Vect) -> VTri<Vect> {
        VTri{a : a, ab : ab, ac : ac}
    }

    fn of<T : Triangle<Val = Vect::Val>>(tri : T) -> VTri<Vect::Own> {
        let a  = Vect::of(tri.a());
        let ab = Vect::of(tri.ab());
        let ac = Vect::of(tri.ac());

        VTri::new(a, ab, ac)
    }

    fn a(&self) -> Vect::Own {
        let x = self.a.x();
        let y = self.a.y();

        Vect::of((x, y))
    }

    fn b(&self) -> Vect::Own 
    where Vect::Val : HAdd
    {
        self.a.add(&self.ab)
    }

    fn c(&self) -> Vect::Own 
    where Vect::Val : HAdd
    {
        self.a.add(&self.ac)
    }

    fn ab(&self) -> Vect::Own {
        let x = self.ab.x();
        let y = self.ab.y();

        Vect::of((x, y))
    }

    fn ac(&self) -> Vect::Own {
        let x = self.ac.x();
        let y = self.ac.y();

        Vect::of((x, y))
    }

    fn add<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VTri<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.add(&vect);

        VTri::new(a, self.ab(), self.ac())
    }

    fn sub<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VTri<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.sub(&vect);

        VTri::new(a, self.ab(), self.ac())
    }

    fn mul<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VTri<Vect::Own>
    where Vect::Val : HMul 
    {
        let a  = self.a.mul(&vect);
        let ab = self.ab.mul(&vect);
        let ac = self.ac.mul(&vect);

        VTri::new(a, ab, ac)
    }

    fn div<V : Vector<Val = Vect::Val>>(&self, vect : V) -> VTri<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a  = self.a.div(&vect);
        let ab = self.ab.div(&vect);
        let ac = self.ac.div(&vect);

        VTri::new(a, ab, ac)
    }

    fn vadd(&self, val : Vect::Val) -> VTri<Vect::Own>
    where Vect::Val : HAdd 
    {
        let a = self.a.vadd(val);

        VTri::new(a, self.ab(), self.ac())
    }

    fn vsub(&self, val : Vect::Val) -> VTri<Vect::Own>
    where Vect::Val : HSub 
    {
        let a = self.a.vsub(val);

        VTri::new(a, self.ab(), self.ac())
    }

    fn vmul(&self, val : Vect::Val) -> VTri<Vect::Own>
    where Vect::Val : HMul 
    {
        let a  = self.a.vmul(val);
        let ab = self.ab.vmul(val);
        let ac = self.ac.vmul(val);

        VTri::new(a, ab, ac)
    }

    fn vdiv(&self, val : Vect::Val) -> VTri<Vect::Own>
    where Vect::Val : HDiv 
    {
        let a  = self.a.vdiv(val);
        let ab = self.ab.vdiv(val);
        let ac = self.ac.vdiv(val);

        VTri::new(a, ab, ac)
    }

    /// points of triangle with pnts() = \[a(), b(), c()\]
    fn pnts(&self) -> [Vect::Own; 3] 
    where Vect::Val : HAdd
    {
        [self.a(), self.b(), self.c()]
    }

    /// vectors of triangle with vects() = \[a(), ab(), ac()\]
    fn vects(&self) -> [Vect::Own; 3] {
        [self.a(), self.ab(), self.ac()]
    }

    /// direction vectors of triangle with dirs() = \[ab(), ac()\]
    fn dirs(&self) -> [Vect::Own; 2] {
        [self.ab(), self.ac()]
    }

    /// check if triangle contains point with baryzentric coordinates
    /// 
    /// slightly slower then implicit line equation check but more precise
    fn contains<V : Vector<Val = Vect::Val>>(&self, pnt : V) -> bool 
    where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
    {
        let zero = Vect::Val::zero();

        let [ab, ac] = self.dirs();
        let abc = ab.det(&ac);
    
        let start;
        let end;
    
        if abc < zero {
            start = abc;
            end   = zero;
        }
        else {
            start = zero;
            end   = abc;
        }
    
        let a = self.a();
        let b = self.b();
    
        let pa  = a.sub(&pnt);
        let pb  = b.sub(&pnt);
    
        let pab = pa.det(&pb);
    
        if !pab.inc_in(start, end) {
            return false
        }
    
        let c = self.c();
    
        let pc  = c.sub(&pnt);
    
        let pbc = pb.det(&pc);
    
        if !pbc.inc_in(start, end) {
            return false
        }
        
        let pca = pc.det(&pa);
    
        if !pca.inc_in(start, end) {
            return false
        }
    
        true
    }
}

pub trait Triangle : Shape {
    // /// the value type of vector
    // type Val  : Copy;
    // /// the vector type of Self
    // type Vect : Vector<Val = Self::Val>;
    /// type that owns it vectors returned by methods
    type Own  : Triangle<Vect = <Self::Vect as Vector>::Own, Own = Self::Own>;

    /// create new triangle of other triangle
    fn of<T : Triangle<Val = Self::Val>>(tri : T) -> Self::Own;

    /// first point of triangle
    fn a(&self) -> <Self::Vect as Vector>::Own;

    /// second point of triangle
    fn b(&self) -> <Self::Vect as Vector>::Own;

    /// third point of triangle
    fn c(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of triangle with ab() = b() - a()
    fn ab(&self) -> <Self::Vect as Vector>::Own;

    /// direction vector of triangle with ac() = c() - a()
    fn ac(&self) -> <Self::Vect as Vector>::Own;

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

    /// points of triangle with pnts() = \[a(), b(), c()\]
    fn pnts(&self) -> [<Self::Vect as Vector>::Own; 3] {
        [self.a(), self.b(), self.c()]
    }

    /// vectors of triangle with vects() = \[a(), ab(), ac()\]
    fn vects(&self) -> [<Self::Vect as Vector>::Own; 3] {
        [self.a(), self.ab(), self.ac()]
    }

    /// direction vectors of triangle with dirs() = \[ab(), ac()\]
    fn dirs(&self) -> [<Self::Vect as Vector>::Own; 2] {
        [self.ab(), self.ac()]
    }

    /// check if triangle contains point with baryzentric coordinates
    /// 
    /// slightly slower then implicit line equation check but more precise
    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool 
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd + HNeg
    {   
        contains_with_barycentric_coords(self, pnt)
    }
}

macro_rules! impl_triangle {
    ($Self:ty) => {
        fn of<T : Triangle<Val = Self::Val>>(tri : T) -> Self::Own {
            <$Self>::of(tri)
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
    
        fn ab(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::ab(self)
        }
    
        fn ac(&self) -> <Self::Vect as Vector>::Own {
            <$Self>::ab(self)
        }
    
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
    };
}

impl<Vect : Vector> Shape for PTri<Vect> 
where Vect::Val : Zero + HSub + HMul + HPOrd
{
    type Val  = Vect::Val;
    type Vect = Vect;

    fn bounds(&self) -> Bounds<Vect::Own> {
        let min = self.a.min(self.b.min(&self.c));
        let max = self.a.max(self.b.max(&self.c));

        Bounds::new_unchecked(min, max)
    }

    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool {
        self.contains(pnt)
    }
}

impl<Vect : Vector> Shape for VTri<Vect>
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
{
    type Val  = Vect::Val;
    type Vect = Vect;

    fn bounds(&self) -> Bounds<Vect::Own> {
        let b = self.b();
        let c = self.c();

        let min = self.a.min(b.min(&c));
        let max = self.a.max(b.max(&c));

        Bounds::new_unchecked(min, max)
    }

    fn contains<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool {
        self.contains(pnt)
    }
}

impl<'a, Tri : Triangle> Triangle for &'a Tri {
    type Own  = Tri::Own;

    impl_triangle!(Tri);
}

impl<Vect : Vector> Triangle for PTri<Vect> 
where Vect::Val : Zero + HSub + HMul + HPOrd
{
    type Own  = PTri<Vect::Own>;

    impl_triangle!(PTri<Vect>);
}

impl<Vect : Vector> Triangle for VTri<Vect>
where Vect::Val : Zero + HAdd + HSub + HMul + HPOrd
{
    type Own  = VTri<Vect::Own>;

    impl_triangle!(VTri<Vect>);
}

/// check if triangle contains point with baryzentric coordinates
/// 
/// slightly slower then implicit line equation check but more precise
pub fn contains_with_barycentric_coords<V : Vector, T : Triangle<Val = V::Val>>(tri : &T, pnt : V) -> bool
where V::Val : Zero + HSub + HMul + HPEq + HPOrd,
      T : ?Sized
{
    let zero = V::Val::zero();

    let [ab, ac] = tri.dirs();
    let abc = ab.det(&ac);

    let start;
    let end;

    if abc < zero {
        start = abc;
        end   = zero;
    }
    else {
        start = zero;
        end   = abc;
    }

    let a = tri.a();
    let b = tri.b();

    let pa  = a.sub(&pnt);
    let pb  = b.sub(&pnt);

    let pab = pa.det(&pb);

    if !pab.inc_in(start, end) {
        return false
    }

    let c = tri.c();

    let pc  = c.sub(&pnt);

    let pbc = pb.det(&pc);

    if !pbc.inc_in(start, end) {
        return false
    }
    
    let pca = pc.det(&pa);

    if !pca.inc_in(start, end) {
        return false
    }

    true
}

/// check if triangle contains point with implicit line equation e(p) = dot(n, p) + d = 0
/// 
/// slighty faster then baryzentric coordinate check but less precise
pub fn contains_with_implicit_line_equation<V : Vector, T : Triangle<Val = V::Val>>(tri : &T, pnt : V) -> bool
where V::Val : Zero + HAdd + HSub + HMul + HPOrd + HNeg,
      T : ?Sized
{
    let zero = V::Val::zero();
    let [a, ab, ac] = tri.vects();

    let turn = ab.det(&ac);

    let b;
    let c;

    if turn < zero {
        b = tri.c();
        c = tri.b();
    }
    else {
        b = tri.b();
        c = tri.c();
    }

    let n_ab = T::Vect::of((a.y() - b.y(), b.x() - a.x()));      
    if n_ab.dot(&pnt) < n_ab.dot(&a) {
        return false;
    }

    let n_bc = T::Vect::of((b.y() - c.y(), c.x() - b.x()));
    if n_bc.dot(&pnt) < n_bc.dot(&b) {
        return false;
    }

    let n_ca = T::Vect::of((c.y() - a.y(), a.x() - c.x()));
    if n_ca.dot(&pnt) < n_ca.dot(&c) {
        return false;
    }

    true
}

