pub use std::ops::{
    Range,
    RangeInclusive as IRange
};

pub fn range<A>(start : A, end : A) -> Range<A> {
    start..end
}

pub fn irange<A>(start  : A, end : A) -> IRange<A> {
    start..=end
}