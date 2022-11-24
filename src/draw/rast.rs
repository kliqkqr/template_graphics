use image::{
    Rgb,
    RgbImage
};

use crate::geom::d2::prim::{
    Vect,
    TRect,
    Rectangle
};

use crate::range::{
    range
};

pub fn fill_rect<A : Rectangle<f32> + std::fmt::Debug>(image : &mut RgbImage, rect : &A, color : Rgb<u8>) {
    let (width, height) = image.dimensions();
    let dims = Vect::new(width, height);

    let min = rect.min().map(|f| f.floor().max(0f32) as u32).max(&Vect::zero());
    let max = rect.max().map(|f| f.ceil().max(0f32) as u32).max(&dims);

    let rect = TRect::from(rect);

    for x in range(min.0, max.0) {
        for y in range(min.1, max.1) {
            let pnt = Vect::new(x as f32, y as f32) + 0.5f32;

            if rect.contains(&pnt) {
                image.put_pixel(x, y, color);
            }
        }
    }
}