use std::borrow::{
    Borrow
};

use std::clone::{
    Clone
};

use crate::geom::d2::prim::seg::{
    Segment
};

use crate::geom::d2::prim::tri::{
    OVTri
};

use crate::geom::d2::prim::vect::{
    Vect
};

use crate::conv::{
    Easy
};

use crate::num::{
    Zero 
};

use crate::ops::{
    HAdd,
    HSub,
    HMul,
    HNeg 
};

use crate::rel::{
    POrd
};

//  TYPES ---------------------------------------------------------------------------------------------------------------------------------

/// 2D rectangle defined by 1 line segment "seg" and 1 side length ratio "rat"
/// 
/// where points a b c d are defined as follows
/// 
/// a = seg.points()\[0\]; b = seg.points()\[1\]; c = b + rat * rotate(90°, b - a); d = a + rat * rotate(90°, b - a);
#[derive(Debug)]
pub struct SRect<A, B : Segment<A>> {
    seg : B,
    rat : A
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

/// 2D bounding rectangle alligned to x and y axis defined by 2 points "start" "end" where start = min(start, end) and end = max(start, end)
#[derive(Debug)]
pub struct BRect<A> {
    start : Vect<A>,
    end   : Vect<A>
}

//  IMPLS ---------------------------------------------------------------------------------------------------------------------------------

impl<A, B : Segment<A>> SRect<A, B> {
    pub fn new(seg : B, rat : A) -> SRect<A, B> {
        SRect{seg, rat}
    }

    pub fn seg(&self) -> &B {
        &self.seg
    }

    pub fn rat(&self) -> A 
    where A : Clone 
    {
        self.rat.clone()
    }
}

impl<A> TRect<A> {
    pub fn new(ovtri : OVTri<A>) -> TRect<A> {
        TRect{tri : ovtri}
    }

    pub fn pos(&self) -> &Vect<A> 
    where A : Clone
    {
        self.tri.pos()
    }

    pub fn dirs(&self) -> &(Vect<A>, Vect<A>) 
    where A : Clone
    {
        self.tri.dirs()
    }

    pub fn dir0(&self) -> &Vect<A>
    where A : Clone 
    {
        self.tri.dir0()
    }

    pub fn dir1(&self) -> &Vect<A>
    where A : Clone 
    {
        self.tri.dir1()
    }
}

impl<A> BRect<A> {
    pub fn new(a : Vect<A>, b : Vect<A>) -> BRect<A> 
    where A : Clone + POrd
    {
        let start = a.min(&b);
        let end = a.max(&b);

        BRect::new_unchecked(start, end)
    }

    pub fn with_zero(a : Vect<A>) -> BRect<A> 
    where A : Clone + POrd + Zero
    {
        let start = a.min(&Vect::zero());
        let end = a.max(&Vect::zero());

        BRect::new_unchecked(start, end)
    }

    pub fn start_zero_unchecked(end : Vect<A>) -> BRect<A> 
    where A : Zero
    {
        BRect::new_unchecked(Vect::zero(), end)
    }

    pub fn new_unchecked(start : Vect<A>, end : Vect<A>) -> BRect<A> {
        BRect{start : start, end : end}
    }

    pub fn start(&self) -> &Vect<A> {
        &self.start
    }

    pub fn end(&self) -> &Vect<A> {
        &self.end
    }

    pub fn clamp<B : Borrow<BRect<A>>>(&self, other : B) -> BRect<A> 
    where A : Clone + POrd
    {
        let s_s = self.start();
        let s_e = self.end();
        let o_s = other.borrow().start();
        let o_e = other.borrow().end();

        let start = o_s.max(&s_s).min(&s_e);
        let end   = o_e.max(&s_s).min(&s_e);

        BRect::new_unchecked(start, end)
    }

    pub fn map<B : Fn(&Vect<A>) -> Vect<C>, C>(&self, func : B) -> BRect<C> 
    where A : Clone, C : Clone + POrd
    {
        let start = func(&self.start());
        let end   = func(&self.end());

        BRect::new(start, end)
    }
}

//  TRAITS --------------------------------------------------------------------------------------------------------------------------------

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
    fn bounds(&self) -> BRect<A> 
    where A : Clone + POrd
    {
        BRect::new_unchecked(self.min(), self.max())
    }

    /// checks if rectangle contains points
    /// 
    /// from: https://math.stackexchange.com/questions/190111/how-to-check-if-a-point-is-inside-a-rectangle
    fn contains<'a, B>(&'a self, pnt : &Vect<A>) -> bool 
    where A        : Clone + Zero + HAdd + HSub + HMul + HNeg + POrd,
          B        : Borrow<OVTri<A>>,
          &'a Self : Easy<B> 
    {
        let ovtri = self.easy();

        let a = ovtri.borrow().pos();
        let ap = pnt - &a;
        let (ab, ac) = ovtri.borrow().dirs();

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ac = ap.dot(&ac);

        let ab_check = A::zero() <= ap_dot_ab && ap_dot_ab <= ab.dot(&ab);
        let ac_check = A::zero() <= ap_dot_ac && ap_dot_ac <= ac.dot(&ac);

        ab_check && ac_check
    }
}

//  TRAIT IMPLS ---------------------------------------------------------------------------------------------------------------------------

impl<A, B : Rectangle<A>> From<&B> for TRect<A> {
    fn from(rect : &B) -> Self {
        let ovtri = OVTri::from(rect);
        TRect::new(ovtri)
    }
}

impl<'a, A> Easy<&'a OVTri<A>> for &'a TRect<A> {
    fn easy(self) -> &'a OVTri<A> {
        &self.tri
    }
}

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

        let a = self.pos().clone();
        let b = &a + ab;
        let c = &b + ad;
        let d = &a + &ad;

        [a, b, c, d]
    }

    fn span(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        (self.pos().clone(), self.dirs().clone())
    }
}

impl<A : Clone + HAdd + HSub + Zero> Rectangle<A> for BRect<A> {
    fn points(&self) -> [Vect<A>; 4] {
        let a = self.start().clone();
        let c = self.end().clone();

        let ac = &c - &a;

        let b = Vect::new(a.0.clone() + ac.0, a.0.clone());
        let d = Vect::new(a.0.clone(), a.1.clone() + ac.1);

        [a, b, c, d]
    }

    fn span(&self) -> (Vect<A>, (Vect<A>, Vect<A>)) {
        let a = self.start().clone();
        let c = self.end().clone();

        let ac = &c - &a;

        let ab = Vect::new(ac.0, A::zero());
        let ad = Vect::new(A::zero(), ac.1);

        (a, (ab, ad))
    }
}