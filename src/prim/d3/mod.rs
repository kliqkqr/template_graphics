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
pub struct Vec<A> (
    A,
    A,
    A
);

// 3D line defined by 2 points
pub struct PLine<A> (
    Vec<A>,
    Vec<A>
);

// 3D line defined by 1 point and 1 vector
pub struct VLine<A> {
    pos : Vec<A>,  // position  point
    dir : Vec<A>   // direction vector
}

// 3D Triangle defined by 3 points
pub struct PTri<A> (
    Vec<A>,
    Vec<A>,
    Vec<A>
);

impl<A> Vec<A> {
    pub fn new(x : A, y : A, z : A) -> Vec<A> {
        Vec(x, y, z)
    }
}

impl<A> PLine<A> {
    pub fn new(a : Vec<A>, b : Vec<A>) -> PLine<A> {
        PLine(a, b)
    }
}

impl<A> VLine<A> {
    pub fn new(pos : Vec<A>, dir : Vec<A>) -> VLine<A> {
        VLine{pos : pos, dir : dir}
    }
}

impl<A> PTri<A> {
    pub fn new(a : Vec<A>, b : Vec<A>, c : Vec<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}

impl<A : Clone> Clone for Vec<A> {
    fn clone(&self) -> Vec<A> {
        Vec::new(self.0.clone(), self.1.clone(), self.2.clone())
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

impl<A : Copy> Copy for Vec<A> {}

impl<A : Copy> Copy for PLine<A> {}

impl<A : Copy> Copy for VLine<A> {}

impl<A : Copy> Copy for PTri<A> {}

impl<A : Add<Output = A>> Add for Vec<A> {
    type Output = Vec<A>;

    fn add(self, other : Self) -> Vec<A> {
        Vec::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<A : Sub<Output = A>> Sub for Vec<A> {
    type Output = Vec<A>;

    fn sub(self, other : Self) -> Vec<A> {
        Vec::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<A : Mul<Output = A>> Mul for Vec<A> {
    type Output = Vec<A>;

    fn mul(self, other : Self) -> Vec<A> {
        Vec::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<A : Mul<Output = A> + Clone> Mul<A> for Vec<A> {
    type Output = Vec<A>;

    fn mul(self, other : A) -> Vec<A> {
        Vec::new(self.0 * other.clone(), self.1 * other.clone(), self.2 * other)
    }
}

impl<A : Div<Output = A>> Div for Vec<A> {
    type Output = Vec<A>;

    fn div(self, other : Self) -> Vec<A> {
        Vec::new(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl<A : Div<Output = A> + Clone> Div<A> for Vec<A> {
    type Output = Vec<A>;

    fn div(self, other : A) -> Vec<A> {
        Vec::new(self.0 / other.clone(), self.1 / other.clone(), self.2 / other)
    }
}
