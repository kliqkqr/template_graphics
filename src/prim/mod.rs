pub mod d2;
pub mod d3;

pub use d2::{
    Vect   as Vec2,
    PLine as PLine2,
    VLine as VLine2,
    PTri  as PTri2
};

pub use d3::{
    Vect   as Vec3,
    PLine as PLine3,
    VLine as VLine3,
    PTri  as PTri3
};
