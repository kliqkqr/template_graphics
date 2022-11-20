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

use crate::util::num::{
    Num
};

/// 2D vector
pub struct Vect<A : Num> (
    pub A,
    pub A
);

/// 2D line defined by 2 points
pub struct PLine<A : Num> (
    pub Vect<A>,
    pub Vect<A>
);

/// 2D line defined by 1 point and 1 vector
pub struct VLine<A : Num> {
    pub pos : Vect<A>,  // position  point
    pub dir : Vect<A>   // direction vector
}

/// 2D Triangle defined by 3 points
pub struct PTri<A : Num> (
    pub Vect<A>,
    pub Vect<A>,
    pub Vect<A>
);

/// 2D Rectangle defined by 2 points and a left orthogonal distance
pub struct DPRect<A : Num> {
    pub pnts : (Vect<A>, Vect<A>),
    pub dist : A  
}

impl<A : Num> Vect<A> {
    pub fn new(x : A, y : A) -> Vect<A> {
        Vect(x, y)
    }

    pub fn ortho_left(&self) -> Vect<A> {
        Vect::new(-self.1, self.0)
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

impl<A : Num> DPRect<A> {
    pub fn new(a : Vect<A>, b : Vect<A>, dist : A) -> DPRect<A> {
        DPRect{pnts : (a, b), dist : dist}
    }

    pub fn contour(&self) -> [Vect<A>; 4] {
        let (a, b) = self.pnts;
        let ortho_ab = (b - a).ortho_left();
        let c = a + ortho_ab;
        let d = b + ortho_ab;

        [a, b, c, d]
    }
}

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

impl<A : Num + Copy> Copy for Vect<A> {}

impl<A : Num + Copy> Copy for PLine<A> {}

impl<A : Num + Copy> Copy for VLine<A> {}

impl<A : Num + Copy> Copy for PTri<A> {}

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
        Vect::new(self.0 * other.clone(), self.1 * other)
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
        Vect::new(self.0 / other.clone(), self.1 / other)
    }
}
