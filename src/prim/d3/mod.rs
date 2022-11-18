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
    Div
};


// 3D vector
pub struct Vect<A> (
    pub A,
    pub A,
    pub A
);

// 3D line defined by 2 points
pub struct PLine<A> (
    pub Vect<A>,
    pub Vect<A>
);

// 3D line defined by 1 point and 1 vector
pub struct VLine<A> {
    pub pos : Vect<A>,  // position  point
    pub dir : Vect<A>   // direction vector
}

// 3D Triangle defined by 3 points
pub struct PTri<A> (
    pub Vect<A>,
    pub Vect<A>,
    pub Vect<A>
);

impl<A> Vect<A> {
    pub fn new(x : A, y : A, z : A) -> Vect<A> {
        Vect(x, y, z)
    }
}

impl<A> PLine<A> {
    pub fn new(a : Vect<A>, b : Vect<A>) -> PLine<A> {
        PLine(a, b)
    }
}

impl<A> VLine<A> {
    pub fn new(pos : Vect<A>, dir : Vect<A>) -> VLine<A> {
        VLine{pos : pos, dir : dir}
    }
}

impl<A> PTri<A> {
    pub fn new(a : Vect<A>, b : Vect<A>, c : Vect<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}

impl<A : Clone> Clone for Vect<A> {
    fn clone(&self) -> Vect<A> {
        Vect::new(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<A : Clone> Clone for PLine<A> {
    fn clone(&self) -> PLine<A> {
        PLine::new(self.0.clone(), self.1.clone())
    }
}

impl<A : Clone> Clone for VLine<A> {
    fn clone(&self) -> VLine<A> {
        VLine::new(self.pos.clone(), self.dir.clone())
    }
}

impl<A : Clone> Clone for PTri<A> {
    fn clone(&self) -> PTri<A> {
        PTri::new(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<A : Copy> Copy for Vect<A> {}

impl<A : Copy> Copy for PLine<A> {}

impl<A : Copy> Copy for VLine<A> {}

impl<A : Copy> Copy for PTri<A> {}

impl<A : Add<Output = A>> Add for Vect<A> {
    type Output = Vect<A>;

    fn add(self, other : Self) -> Vect<A> {
        Vect::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<A : Sub<Output = A>> Sub for Vect<A> {
    type Output = Vect<A>;

    fn sub(self, other : Self) -> Vect<A> {
        Vect::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<A : Mul<Output = A>> Mul for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : Self) -> Vect<A> {
        Vect::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<A : Mul<Output = A> + Clone> Mul<A> for Vect<A> {
    type Output = Vect<A>;

    fn mul(self, other : A) -> Vect<A> {
        Vect::new(self.0 * other.clone(), self.1 * other.clone(), self.2 * other)
    }
}

impl<A : Div<Output = A>> Div for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : Self) -> Vect<A> {
        Vect::new(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl<A : Div<Output = A> + Clone> Div<A> for Vect<A> {
    type Output = Vect<A>;

    fn div(self, other : A) -> Vect<A> {
        Vect::new(self.0 / other.clone(), self.1 / other.clone(), self.2 / other)
    }
}
