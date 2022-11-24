// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPORTS 
// ----------------------------------------------------------------------------------------------------------------------------------------

use std::borrow::{
    Borrow
};

use std::clone::{
    Clone
};

use std::convert::{
    From
};

use std::marker::{
    PhantomData as Unused
};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg
};

use std::option::{
    Option
};

use crate::conv::{
    Easy
};

use crate::num::{
    Zero,
    One
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HDiv,
    HNeg
};

use crate::range::{
    IRange,
    irange
};

use crate::rel::{
    POrd,
    ExtHPOrd
};

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TYPES 
// ----------------------------------------------------------------------------------------------------------------------------------------

// Vectors

/// 2D vector with 2 values "0" "1"
#[derive(Debug)]
pub struct Vect<A>(pub A, pub A);

// Line segments

/// 2D line segment defined by 2 points "0" "1"
#[derive(Debug)]
pub struct PSeg<A>(pub Vect<A>, pub Vect<A>);

/// 2D line segment defined by 1 position vector "pos" and 1 direction vector "dir"
#[derive(Debug)]
pub struct VSeg<A> {
    pub pos : Vect<A>,
    pub dir : Vect<A>
}

// Lines

/// 2D line defined by 1 line segment "seg"
#[derive(Debug)]
pub struct SLine<A, B : Segment<A>> {
    pub seg : B,
    _unused : Unused<A>
}

// Triangles

/// 2D triangle defined by 3 points "0" "1" "2"
#[derive(Debug)]
pub struct PTri<A>(pub Vect<A>, pub Vect<A>, pub Vect<A>);

/// 2D triangle defined by 1 position vector "pos" and 2 direction vectors "dir.0" "dir.1"
#[derive(Debug)]
pub struct VTri<A> {
    pub pos : Vect<A>,
    pub dir : (Vect<A>, Vect<A>)
}


/// 2D triangle defined by 1 position vector and 2 direction vectors where direction vectors are orthogonal
#[derive(Debug)]
pub struct OVTri<A> {
    tri : VTri<A>
}

// Rectangles

/// 2D rectangle defined by 1 line segment "seg" and 1 side length ratio "rat"
/// 
/// where points a b c d are defined as follows
/// 
/// a = seg.points()\[0\]; b = seg.points()\[1\]; c = b + rat * rotate(90°, b - a); d = a + rat * rotate(90°, b - a);
#[derive(Debug)]
pub struct SRect<A, B : Segment<A>> {
    pub seg : B,
    pub rat : A
}

/// 2D rectangle defined by triangle with 1 position vector "pos()" and 2 direction vectors "dirs()" where direction vectors are orthogonal
/// 
/// where points a b c d are defined as follows 
/// 
/// a = pos(); b = a + dirs().0; c = b + dirs().1; d = a + dirs().1;
#[derive(Debug)]
pub struct TRect<A> {
    tri : OVTri<A>,   
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

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

    pub fn orth_l(&self) -> Vect<A> 
    where A : Clone + HNeg
    {
        Vect::new(-self.1.clone(), self.0.clone())
    }

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

impl<A> PSeg<A> {
    pub fn new(a : Vect<A>, b : Vect<A>) -> PSeg<A> {
        PSeg(a, b)
    }
}

impl<A> VSeg<A> {
    pub fn new(pos : Vect<A>, dir : Vect<A>) -> VSeg<A> {
        VSeg{pos : pos, dir : dir}
    }
}

impl<A> PTri<A> {
    fn new(a : Vect<A>, b : Vect<A>, c : Vect<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}

impl<A> VTri<A> {
    fn new(pos : Vect<A>, dir0 : Vect<A>, dir1 : Vect<A>) -> VTri<A> {
        VTri{pos : pos, dir : (dir0, dir1)}
    }
}

impl<A> OVTri<A> {
    pub fn new_unchecked(tri : VTri<A>) -> OVTri<A> {
        OVTri{tri : tri}
    }

    pub fn pos(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.pos.clone()
    }

    pub fn dir0(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.dir.0.clone()
    }

    pub fn dir1(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.dir.1.clone()
    }

    pub fn dirs(&self) -> (Vect<A>, Vect<A>) 
    where A : Clone
    {
        self.tri.dir.clone()
    }
}

impl<A, B : Segment<A>> SRect<A, B> {
    pub fn new(seg : B, rat : A) -> SRect<A, B> {
        SRect{seg : seg, rat : rat}
    }
}

impl<A> TRect<A> {
    pub fn new(ovtri : OVTri<A>) -> TRect<A> {
        TRect{tri : ovtri}
    }

    pub fn pos(&self) -> Vect<A> 
    where A : Clone
    {
        self.tri.pos()
    }

    pub fn dirs(&self) -> (Vect<A>, Vect<A>) 
    where A : Clone
    {
        self.tri.dirs()
    }

    pub fn dir0(&self) -> Vect<A>
    where A : Clone 
    {
        self.tri.dir0()
    }

    pub fn dir1(&self) -> Vect<A>
    where A : Clone 
    {
        self.tri.dir1()
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TRAITS 
// ----------------------------------------------------------------------------------------------------------------------------------------

pub trait Segment<A> {
    /// create line segment from another line segment
    fn from<B : Segment<A>>(seg : &B) -> Self;

    /// the 2 points that define the line segment
    /// 
    /// with: self.points()\[0\] = self.pos() and self.dir() = self.points()\[1\] - self.points()\[0\]
    fn points(&self) -> [Vect<A>; 2];

    /// the position vector of the line segment
    /// 
    /// with: self.points()\[0\] = self.pos()
    fn pos(&self) -> Vect<A> {
        let [point, _] = self.points();
        point
    }

    /// the direction vector of the line segment
    /// 
    /// with: self.dir() = self.points()\[1\] - self.points()\[0\]
    fn dir(&self) -> Vect<A>
    where A : HSub
    {
        let [point_0, point_1] = self.points();
        point_1 - point_0
    }

    /// optional intersection between to line segments a + r * b with
    ///
    /// a + r * b = c + s * d
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b) where 0 <= r <= 1
    fn intsec<B : Segment<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> 
    where A : Clone + One + Zero + HAdd + HSub + HMul + HDiv + POrd 
    {
        let a = self.pos();
        let b = self.dir();
        let c = other.pos();
        let d = other.dir();

        let div = d.lin_dep_det(&b, eps)?;
        let r = d.det(&c) + a.det(&d) / div;

        match A::zero() <= r && r <= A::one() {
            true  => Some(a + b * r),
            false => None
        }
    }
}

pub trait Line<A> : Segment<A> {
    fn intsec<B : Line<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> 
    where A : Clone + One + Zero + HAdd + HSub + HMul + HDiv + POrd 
    {
        let a = self.pos();
        let b = self.dir();
        let c = other.pos();
        let d = other.dir();

        let div = d.lin_dep_det(&b, eps)?;
        let r = d.det(&c) + a.det(&d) / div;

        Some(a + b * r)
    }
}

pub trait Triangle<A> {
    /// create triangle from another triangle
    fn from<B : Triangle<A>>(tri : &B) -> Self;

    /// points of the triangle either positive or negative turning
    fn points(&self) -> [Vect<A>; 3];

    /// 1 position vector and 2 direction vectors
    fn pos_dirs(&self) -> (Vect<A>, (Vect<A>, Vect<A>));
}

pub trait Rectangle<A> {
    /// points of the rectangle either positive or negative turning (without overlap of line segments)
    fn points(&self) -> [Vect<A>; 4];

    /// linear span of the rectangle with 1 position vector and 2 orthogonal direction vectors
    fn span(&self) -> (Vect<A>, (Vect<A>, Vect<A>));

    /// componentwise min of rectangle points
    fn min(&self) -> Vect<A> 
    where A : Clone + POrd 
    {
        let [a, b, c, d] = self.points();
        a.min(&b).min(&c).min(&d)
    }

    /// componentwise max of rectangle points
    fn max(&self) -> Vect<A> 
    where A : Clone + POrd 
    {
        let [a, b, c, d] = self.points();
        a.max(&b).max(&c).max(&d)
    }

    /// x- / y-axis alligned bounding box of rectangle
    fn bounds(&self) -> IRange<Vect<A>> 
    where A : Clone + POrd 
    {
        irange(self.min(), self.max())
    }

    /// checks if rectangle contains points
    /// 
    /// from: https://math.stackexchange.com/questions/190111/how-to-check-if-a-point-is-inside-a-rectangle
    fn contains<'a, B>(&'a self, pnt : &Vect<A>) -> bool 
    where A        : Clone + Zero + HAdd + HSub + HMul + HNeg + POrd,
          B        : Borrow<OVTri<A>>,
          &'a Self : Easy<B> 
    {
        // let seg = self.base_seg::<PSeg<A>>();
        // let rat = self.base_rat();

        // let [a, b] = seg.points();
        // let ab = seg.dir();
        // let c = b + ab.orth_l() * rat;
        // let ac = c - a.clone();
        // let ap = pnt.clone() - a;

        // let ap_dot_ab = ap.dot(&ab);
        // let ap_dot_ac = ap.dot(&ac);

        // A::zero() < ap_dot_ab && ap_dot_ab < ab.dot(&ab) && A::zero() < ap_dot_ac && ap_dot_ac < ac.dot(&ac)

        // let [a, b, c, d] = self.points();

        // let [a, b, c] = self.triangle::<VTri<A>>().points();

        // let ab = b - a.clone();
        // let ad = d - a.clone();
        // let am = pnt.clone() - a;

        // let ab = seg.

        // let am_dot_ab = am.dot(&ab);
        // let am_dot_ad = am.dot(&ad);

        // let left  = A::zero() <= am_dot_ab && am_dot_ab <= ab.dot(&ab);
        // let right = A::zero() <= am_dot_ad && am_dot_ad <= ad.dot(&ad);

        // left && right

        let ovtri = self.easy();

        let a = ovtri.borrow().pos();
        let ap = pnt.clone() - a;
        let (ab, ac) = ovtri.borrow().dirs();

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ac = ap.dot(&ac);

        let left  = A::zero() <= ap_dot_ab && ap_dot_ab <= ab.dot(&ab);
        let right = A::zero() <= ap_dot_ac && ap_dot_ac <= ac.dot(&ac);

        left && right
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   STD TRAIT IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

// Clone

impl<A : Clone> Clone for Vect<A> {
    fn clone(&self) -> Self {
        Vect::new(self.0.clone(), self.1.clone())
    }
}

impl<A : Clone> Clone for PSeg<A> {
    fn clone(&self) -> PSeg<A> {
        PSeg::new(self.0.clone(), self.1.clone())
    }
}

impl<A : Clone> Clone for VSeg<A> {
    fn clone(&self) -> VSeg<A> {
        VSeg::new(self.pos.clone(), self.dir.clone())
    }
}

// ToString

impl<A : ToString> ToString for Vect<A> {
    fn to_string(&self) -> String {
        format!("Vect({}, {})", self.0.to_string(), self.1.to_string())
    }
} 

// From

impl<A, B : Rectangle<A>> From<&B> for OVTri<A> {
    fn from(rect : &B) -> Self {
        let (pos, dir) = rect.span();
        let tri = VTri::new(pos, dir.0, dir.1);

        OVTri::new_unchecked(tri)
    }
}

impl<A, B : Rectangle<A>> From<&B> for TRect<A> {
    fn from(rect : &B) -> Self {
        let ovtri = OVTri::from(rect);
        TRect::new(ovtri)
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

// Neg

impl<A : Neg<Output = A>> Neg for Vect<A> {
    type Output = Vect<A>;

    fn neg(self) -> Vect<A> {
        Vect::new(-self.0, -self.1)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   CRATE TRAIT IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

impl<'a, A> Easy<&'a OVTri<A>> for &'a TRect<A> {
    fn easy(self) -> &'a OVTri<A> {
        &self.tri
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   PRIMITIVE TRAIT IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

// Segment

impl<A : Clone> Segment<A> for PSeg<A> {
    fn from<B : Segment<A>>(seg : &B) -> PSeg<A> {
        let [a, b] = seg.points();
        PSeg::new(a, b)
    }

    fn points(&self) -> [Vect<A>; 2] {
        [self.0.clone(), self.1.clone()]
    }
}

impl<A : Clone + HAdd + HSub> Segment<A> for VSeg<A> {
    fn from<B : Segment<A>>(seg : &B) -> VSeg<A> {
        VSeg::new(seg.pos(), seg.dir())
    }

    fn points(&self) -> [Vect<A>; 2] {
        let a = self.pos.clone();
        let b = a.clone() + self.dir.clone();

        [a, b]
    }

    fn pos(&self) -> Vect<A> {
        self.pos.clone()
    }

    fn dir(&self) -> Vect<A> {
        self.dir.clone()
    }
}

// Triangle

impl<A : Clone + HSub> Triangle<A> for PTri<A> {
    fn from<B : Triangle<A>>(tri : &B) -> Self {
        let [a, b, c] = tri.points();
        PTri::new(a, b, c)
    }

    fn points(&self) -> [Vect<A>; 3] {
        [self.0.clone(), self.1.clone(), self.2.clone()]
    }

    fn pos_dirs(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        let pos  = self.0.clone();
        let dirs = (self.1.clone() - self.0.clone(), self.2.clone() - self.0.clone());

        (pos, dirs)
    }
}

impl<A : Clone + HAdd> Triangle<A> for VTri<A> {
    fn from<B : Triangle<A>>(tri : &B) -> Self {
        let (pos, dirs) = tri.pos_dirs();
        VTri::new(pos, dirs.0, dirs.1)
    }

    fn points(&self) -> [Vect<A>; 3] {
        let a = self.pos.clone();
        let b = self.pos.clone() + self.dir.0.clone();
        let c = self.pos.clone() + self.dir.1.clone();

        [a, b, c]
    }

    fn pos_dirs(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        (self.pos.clone(), self.dir.clone())
    }
}

// Rectangle

impl<A : Clone + HAdd + HSub + HMul + HNeg, B : Segment<A>> Rectangle<A> for SRect<A, B> {
    fn points(&self) -> [Vect<A>; 4] {
        let dir = self.seg.dir();
        let [a, b] = self.seg.points();

        let v = dir.orth_l() * self.rat.clone();

        let c = b.clone() + v.clone();
        let d = a.clone() + v;
        
        [a, b, c, d]
    }

    fn span(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        let pos  = self.seg.pos();
        let dir0 = self.seg.dir();
        let dir1 = self.seg.dir().orth_l() * self.rat.clone();

       (pos, (dir0, dir1))
    }
}

impl<A : Clone + HAdd> Rectangle<A> for TRect<A> {
    fn points(&self) -> [Vect<A>; 4] {
        let (ab, ad) = self.dirs();

        let a = self.pos();
        let b = a.clone() + ab;
        let c = b.clone() + ad.clone();
        let d = a.clone() + ad;

        [a, b, c, d]
    }

    fn span(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        (self.pos(), self.dirs())
    }
}