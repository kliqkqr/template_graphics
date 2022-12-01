#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IndSeg {
    a : usize,
    b : usize 
}

impl IndSeg {
    pub fn new(index_a : usize, index_b : usize) -> IndSeg {
        IndSeg{a : index_a, b : index_b}
    }

    pub fn a(&self) -> usize {
        self.a 
    }

    pub fn b(&self) -> usize {
        self.b
    }

    pub fn equiv(&self, other : &IndSeg) -> bool {
        self.a() == other.a() && self.b() == other.b() || self.b() == other.a() && self.a() == other.b()
    }

    pub fn swap(&mut self, from : usize, to : usize) {
        if self.a == from {
            self.a = to;
        }

        if self.b == from {
            self.b = to;
        }
    }

    pub fn swap_shift(&mut self, from : usize, to : usize) {
        if self.a == from {
            self.a = to;
        }

        if self.b == from {
            self.b = to;
        }

        if self.a > from {
            self.a -= 1
        }

        if self.b > from {
            self.b -= 1
        }
    }

    pub fn sort(&mut self) {
        self.a = std::cmp::min(self.a(), self.b());
        self.b = std::cmp::max(self.a(), self.b());
    }
}