use std::marker::{
    Copy 
};

use crate::geom::d2::prim::refactor::vect::{
    Vector 
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
    HPEq,
    HPOrd
};

pub trait Triangle {
    type Val  : Copy;
    type Vect : Vector<Val = Self::Val, Out = Self::Vect>;
    type Out  : Triangle<Vect = Self::Vect>;

    /// create new trinagle of 3 points
    fn of_pnts(a : Self::Vect, b : Self::Vect, c : Self::Vect) -> Self::Out;

    /// create new triangle of 1 position vector and 2 direction vectors
    fn of_vects(a : Self::Vect, ab : Self::Vect, ac : Self::Vect) -> Self::Out;

    /// first point of triangle
    fn a(&self) -> Self::Vect;

    /// second point of triangle
    fn b(&self) -> Self::Vect;

    /// thrird point of triangle
    fn c(&self) -> Self::Vect;

    /// direction vector of triangle with ab() = b() - a()
    fn ab(&self) -> Self::Vect;

    /// direction vector of triangle with ac() = c() - a()
    fn ac(&self) -> Self::Vect;

    /// add vector to triangle points (translation)
    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// sub vector from triangle points (translation)
    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// mul vector to triangle points
    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// div vector from triangle points
    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out;

    /// mul vector values with value (scaling)
    fn vadd(&self, val : Self::Val) -> Self::Out;

    // div vector values with value (scaling)
    fn vsub(&self, val : Self::Val) -> Self::Out;

    /// mul vector values with value (scaling)
    fn vmul(&self, val : Self::Val) -> Self::Out;

    // div vector values with value (scaling)
    fn vdiv(&self, val : Self::Val) -> Self::Out;

    /// points of triangle with pnts() = \[a(), b(), c()\]
    fn pnts(&self) -> [Self::Vect; 3] {
        [self.a(), self.b(), self.c()]
    }

    /// direction vectors of triangle with dirs() = \[b() - a(), c() - a()\] = [ab(), ac()]
    fn dirs(&self) -> [Self::Vect; 2] {
        [self.ab(), self.ac()]
    }

    fn contains_bary<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool 
    where Self::Val : Zero + HSub + HMul + HPEq + HPOrd 
    {  
        let zero = Self::Val::zero();

        let [ab, ac]  = self.dirs();
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

        let b = self.b();
        let c = self.c();

        let pb  = b.sub(&pnt);
        let pc  = c.sub(&pnt);
        let pbc = pb.det(&pc);
        
        if !pbc.inc_in(start, end) {
            return false
        }
        
        let a = self.a();

        let pa  = a.sub(&pnt);
        let pca = pc.det(&pa);
        
        if !pca.inc_in(start, end) {
            return false
        }

        let pab = pa.det(&pb);

        if !pab.inc_in(start, end) {
            return false
        }

        true
    }

    fn contains_norm<V : Vector<Val = Self::Val>>(&self, pnt : V) -> bool 
    where Self::Val : Zero + HAdd + HSub + HMul + HPOrd + HNeg + std::fmt::Debug
    {   
        let zero = Self::Val::zero();
        let [ab, ac] = self.dirs();

        let turn = ab.det(&ac);

        let a = self.a();
        let b;
        let c;

        if turn < zero {
            b = self.c();
            c = self.b();
        }
        else {
            b = self.b();
            c = self.c();
        }

        let n_ab = Self::Vect::of_vals(a.y() - b.y(), b.x() - a.x());      
        if n_ab.dot(&pnt) < n_ab.dot(&a) {
            return false;
        }

        let n_bc = Self::Vect::of_vals(b.y() - c.y(), c.x() - b.x());
        if n_bc.dot(&pnt) < n_bc.dot(&b) {
            return false;
        }

        let n_ca = Self::Vect::of_vals(c.y() - a.y(), a.x() - c.x());
        if n_ca.dot(&pnt) < n_ca.dot(&c) {
            return false;
        }

        true
    }
}

impl<A : Copy + HSub> Triangle for ((A, A), (A, A), (A, A)) {
    type Val  = A;

    type Vect = (A, A);

    type Out  = Self;

    fn of_pnts(a : Self::Vect, b : Self::Vect, c : Self::Vect) -> Self::Out {
        todo!()
    }

    fn of_vects(a : Self::Vect, ab : Self::Vect, ac : Self::Vect) -> Self::Out {
        todo!()
    }

    fn a(&self) -> Self::Vect {
        self.0
    }

    fn b(&self) -> Self::Vect {
        self.1
    }

    fn c(&self) -> Self::Vect {
        self.2
    }

    fn ab(&self) -> Self::Vect {
        self.1.sub(&self.0)
    }

    fn ac(&self) -> Self::Vect {
        self.2.sub(&self.0)
    }

    fn add<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out {
        todo!()
    }

    fn sub<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out {
        todo!()
    }

    fn mul<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out {
        todo!()
    }

    fn div<V : Vector<Val = Self::Val>>(&self, vect : V) -> Self::Out {
        todo!()
    }

    fn vadd(&self, val : Self::Val) -> Self::Out {
        todo!()
    }

    fn vsub(&self, val : Self::Val) -> Self::Out {
        todo!()
    }

    fn vmul(&self, val : Self::Val) -> Self::Out {
        todo!()
    }

    fn vdiv(&self, val : Self::Val) -> Self::Out {
        todo!()
    }
}