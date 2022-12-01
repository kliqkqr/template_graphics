/// trait for easy / fast conversions
pub trait Easy<A> {
    /// easy / fast conversion
    fn easy(self) -> A;
}

/// trait for duplicating values from reference 
pub trait Dup {
    fn dup(&self) -> Self;
}

/// trait for conversion from A to Self
pub trait Of<A> {
    fn of(_ : A) -> Self;
}

/// trait for conversion from Self to A - inverse of "Of" trait if implemented
pub trait To<A> {
    fn to(self) -> A;
}

/// trait for conversion from Self to &A
pub trait Ref<A> {
    fn r(&self) -> &A;
}

impl<A : std::convert::From<B>, B> Of<B> for A {
    fn of(b : B) -> Self {
        <A as std::convert::From<B>>::from(b)
    }
}

impl<A, B> To<B> for A where B : Of<A> {
    fn to(self) -> B {
        B::of(self)
    }
}

impl<A : Clone> Dup for A {
    fn dup(&self) -> Self {
        self.clone()
    }
}

impl<A> Ref<A> for A {
    fn r(&self) -> &A {
        &self
    }
}

impl<'a, A> Ref<A> for &'a A {
    fn r(&self) -> &A {
        self
    }
}