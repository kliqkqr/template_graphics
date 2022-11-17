/// 2D vector
pub struct Vec<A> (
    A,
    A
);

/// 2D line defined by 2 points
pub struct PLine<A> (
    Vec<A>,
    Vec<A>
);

/// 2D line defined by 1 point and 1 vector
pub struct VLine<A> {
    pos : Vec<A>,  // position  point
    dir : Vec<A>   // direction vector
}

/// 2D Triangle defined by 3 points
pub struct PTri<A> (
    Vec<A>,
    Vec<A>,
    Vec<A>
);

impl<A> Vec<A> {
    fn new(x : A, y : A) -> Vec<A> {
        Vec(x, y)
    }
}

impl<A> PLine<A> {
    fn new(a : Vec<A>, b : Vec<A>) -> PLine<A> {
        PLine(a, b)
    }
}

impl<A> VLine<A> {
    fn new(pos : Vec<A>, dir : Vec<A>) -> VLine<A> {
        VLine{pos : pos, dir : dir}
    }
}

impl<A> PTri<A> {
    fn new(a : Vec<A>, b : Vec<A>, c : Vec<A>) -> PTri<A> {
        PTri(a, b, c)
    }
}