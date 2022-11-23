// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPORTS 
// ----------------------------------------------------------------------------------------------------------------------------------------

use std::clone::{
    Clone
};

use std::marker::{
    Copy 
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

use crate::util::num::{
    Num
};

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TYPES 
// ----------------------------------------------------------------------------------------------------------------------------------------

/// 2D vector
pub struct Vect<A : Num>(pub A, pub A);

/// 2D line defined by 2 points
pub struct PLine<A : Num> (pub Vect<A>, pub Vect<A>);

/// 2D line defined by 1 point and 1 direction vector
pub struct VLine<A : Num> {
    pub pos : Vect<A>,
    pub dir : Vect<A>
}

/// 2D line segment defined by 1 point and 1 direction vector
pub struct VLSeg<A : Num> {
    pub pos : Vect<A>,
    pub dir : Vect<A>
}

/// 2D triangle defined by 3 points
pub struct PTri<A : Num> (pub Vect<A>, pub Vect<A>, pub Vect<A>);

/// 2D square defined by 2 points a b where c = b + rotate(90, b - a) and d = a + rotate(90, b - a)
pub struct PSqr<A : Num> (pub Vect<A>, pub Vect<A>);

/// 2D rectangle defined by 2 points a b and a left orthogonal distance ratio where c = b + disr * rotate(90, b - a) and d = a + disr * rotate(90, b - a)
pub struct PDRect<A : Num> {
    pub pnts : (Vect<A>, Vect<A>),
    pub disr : A  
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

impl<A : Num> Vect<A> {
    pub fn new(_0 : A, _1 : A) -> Vect<A> {
        Vect(_0, _1)
    }

    /// determinant of two vectors a b : (a0 * b1 - a1 * b0) 
    pub fn det(&self, other : &Vect<A>) -> A {
        self.0 * other.1 - self.1 * other.0
    }

    /// dot product of two vectors
    pub fn dot(&self, other : &Vect<A>) -> A {
        self.0 * other.0 + self.1 * other.1
    }

    /// checks linear dependency of two vectors by checking if determinant is in range [0 - eps, 0 + eps]
    pub fn lin_dep(&self, other : &Vect<A>, eps : A) -> bool {
        self.det(other).inc_in(A::zero() - eps, A::zero() + eps)
    }

    /// returns determinant if two vectors are linear independent
    pub fn lin_dep_det(&self, other : &Vect<A>, eps : A) -> Option<A> {
        let det = self.det(other); 

        match det.inc_in(A::zero() - eps, A::zero() + eps) {
            false => None,
            true  => Some(det)
        }
    }

    /// rotates vector 90°
    pub fn orth_l(&self) -> Vect<A> {
        Vect::new(-self.1, self.0)
    }

    /// rotates vector 270°
    pub fn orth_r(&self) -> Vect<A> {
        Vect::new(self.1, -self.0)
    }
}

impl<A : Num> PLine<A> {
    pub fn new(a : Vect<A>, b : Vect<A>) -> PLine<A> {
        PLine(a, b)
    }
}

impl<A : Num> VLine<A> {
    pub fn new(pos : Vect<A>, dir : Vect<A>) -> VLine<A> {
        VLine{pos : pos, dir : dir}
    }
}

impl<A : Num> PTri<A> {
    pub fn new(a : Vect<A>, b : Vect<A>, c : Vect<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}

impl<A : Num> PDRect<A> {
    pub fn new(a : Vect<A>, b : Vect<A>, dist_ratio : A) -> PDRect<A> {
        PDRect{pnts : (a, b), disr : dist_ratio}
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TRAITS 
// ----------------------------------------------------------------------------------------------------------------------------------------

pub trait Line<A : Num> {
    /// points that define line
    fn pnts(&self) -> (Vect<A>, Vect<A>);

    /// position vector of line
    fn pos(&self) -> Vect<A> {
        self.pnts().0
    }

    /// direction vector of line
    fn dir(&self) -> Vect<A> {
        self.pnts().1
    }

    /// optional intersection between to lines : a + r * b
    ///
    /// a + r * b = c + s * d
    ///
    /// => r = (det(d, c) + det(a, d)) / det(d, b)
    fn intsec<B : Line<A>>(&self, other : &B, eps : A) -> Option<Vect<A>> {
        let a = self.pos();
        let b = self.dir();
        let c = other.pos();
        let d = other.dir();

        let div = Vect::lin_dep_det(&d, &b, eps)?;
        let r = (Vect::det(&d, &c) + Vect::det(&a, &d)) / div;

        Some(a + b * r)
    }
}

pub trait Triangle<A : Num> {
    fn pnts(&self) -> (Vect<A>, Vect<A>, Vect<A>);
}

pub trait Rectangle<A : Num> {
    /// a side of the rectangle
    /// 
    /// a = side.0, b = side.1
    /// 
    /// c = b + disr * rotate(90, b - a)
    /// 
    /// d = a + disr * rorate(90, b - a)
    fn side(&self) -> (Vect<A>, Vect<A>);

    /// left orthogonal distance ratio to self.side()
    /// 
    /// a = side.0, b = side.1
    /// 
    /// c = b + disr * rotate(90, b - a)
    /// 
    /// d = a + disr * rorate(90, b - a)
    fn disr(&self) -> A;

    /// checks if rectangle contains point  
    fn contains(&self, pnt : &Vect<A>) -> bool {
        let (a, b) = self.side();
        let disr = self.disr();

        let ab = b - a;
        let c  = b + ab.orth_l() * disr;
        let ac = c - a;
        let ap = *pnt - a;

        let ap_dot_ab = ap.dot(&ab);
        let ap_dot_ac = ap.dot(&ac);

        A::zero() < ap_dot_ab && ap_dot_ab < ab.dot(&ab) && A::zero() < ap_dot_ac && ap_dot_ac < ac.dot(&ac)
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------
//   TRAIT IMPLEMENTATIONS 
// ----------------------------------------------------------------------------------------------------------------------------------------

impl<A : Num> Clone for Vect<A> {
    fn clone(&self) -> Vect<A> {
        Vect::new(self.0, self.1)
    }
}

impl<A : Num> Clone for PLine<A> {
    fn clone(&self) -> PLine<A> {
        PLine::new(self.0, self.1)
    }
}

impl<A : Num> Clone for VLine<A> {
    fn clone(&self) -> VLine<A> {
        VLine::new(self.pos, self.dir)
    }
}

impl<A : Num> Clone for PTri<A> {
    fn clone(&self) -> PTri<A> {
        PTri::new(self.0, self.1, self.2)
    }
}

impl<A : Num> Copy for Vect<A> {}

impl<A : Num> Copy for PLine<A> {}

impl<A : Num> Copy for VLine<A> {}

impl<A : Num> Copy for PTri<A> {}

impl<A : Num> Add for Vect<A> {
    type Output = Vect<A>;

    fn add(self, other : Self) -> Vect<A> {
        Vect::new(self.0 + other.0, self.1 + other.1)
    }
}

impl<A : Num> Sub for Vect<A> {
    type Output = Vect<A>;

    fn sub(self, other : Self) -> Vect<A> {
        Vect::new(self.0 - other.0, self.1 - other.1)
    }
}

impl<A : Num> Mul for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : Self) -> Vect<A> {
        Vect::new(self.0 * other.0, self.1 * other.1)
    }
}

impl<A : Num> Mul<A> for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : A) -> Vect<A> {
        Vect::new(self.0 * other, self.1 * other)
    }
}

impl<A : Num> Div for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : Self) -> Vect<A> {
        Vect::new(self.0 / other.0, self.1 / other.1)
    }
}

impl<A : Num> Div<A> for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : A) -> Vect<A> {
        Vect::new(self.0 / other, self.1 / other)
    }
}

impl<A : Num> Neg for Vect<A> {
    type Output = Vect<A>;

    fn neg(self) -> Vect<A> {
        Vect::new(-self.0, -self.1)
    }
}