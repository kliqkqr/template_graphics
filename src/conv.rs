/// trait for easy / fast conversions
pub trait Easy<A> {
    /// easy / fast conversion
    fn easy(self) -> A;
}

/// trait for conversion from A to Self
pub trait From<A> {
    fn from(_ : A) -> Self;
}

/// trait for conversion from Self to A - inverse of From trait if implemented
pub trait To<A> {
    fn to(self) -> A;
}

impl<A : std::convert::From<B>, B> From<B> for A {
    fn from(b : B) -> Self {
        <A as std::convert::From<B>>::from(b)
    }
}

impl<A, B> To<B> for A where B : From<A> {
    fn to(self) -> B {
        B::from(self)
    }
}