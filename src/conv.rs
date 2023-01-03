/// trait for conversion from A to Self
pub trait Of<A> {
    fn of(_ : A) -> Self;
}

/// trait for conversion from Self to A - inverse of "Of" trait if implemented
pub trait To<A> {
    fn to(self) -> A;
}

pub trait Cast<A> {
    fn cast(self) -> A;
}

impl<A, B : Of<A>> To<B> for A {
    fn to(self) -> B {
        B::of(self)
    }
} 

impl<A> Cast<A> for A {
    fn cast(self) -> A {
        self
    }
}

macro_rules! impl_cast {
    (from $From:ty, to $To:ty) => {
        impl Cast<$To> for $From {
            fn cast(self) -> $To {
                self as $To
            }
        }
    };
}

impl_cast!(from f32, to u32);
impl_cast!(from u32, to f32);