use image::{
    Rgb,
    RgbImage
};

use crate::geom::d2::prim::{
    Vect,
    TRect,
    SRect,
    BRect,
    Segment,
    Rectangle
};

use crate::range::{
    range
};

use crate::geom::d2::prim::refactor::vect::{
    Vector
};

pub fn fill_rect<A : Rectangle<f32> + std::fmt::Debug>(image : &mut RgbImage, rect : &A, color : Rgb<u8>) {
    let (width, height) = image.dimensions();
    let dims = BRect::start_zero_unchecked(Vect::new(width, height));

    let bounds = rect.bounds().map(|v| v.map(|f| *f as u32));
    let bounds = dims.clamp(bounds);

    let start = bounds.start();
    let end   = bounds.end();

    let rect = TRect::from(rect);

    for x in range(start.0, end.0) {
        for y in range(start.1, end.1) {
            let point = Vect::new(x as f32, y as f32) + 0.5f32;

            if rect.contains::<&TRect<f32>>(&point) {
                image.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_seg<A : Segment<f32>>(image : &mut RgbImage, seg : &A, color : Rgb<u8>, width : f32) {
    let seg_dir = seg.dir();
    let seg_len = seg_dir.len();
    let offset_ratio = (width / 2.0) / seg_len;
    let seg_offset = seg_dir.orth_r() * offset_ratio;

    let offset_seg = seg.add(&seg_offset);
    let rect_ratio = width / seg_len;
    let rect = SRect::new(offset_seg, rect_ratio);

    fill_rect(image, &rect, color);
}

pub fn fill_tri_bary<T>(image : &mut RgbImage, tri : &T, color : Rgb<u8>)
where T : crate::geom::d2::prim::refactor::tri::Triangle<Val = f32>
{
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let point = (x as f32, y as f32).vadd(0.5f32);

            if tri.contains_bary(&point) {
                image.put_pixel(x, y, color);
            }
        }
    }
}

pub fn fill_tri_norm<T>(image : &mut RgbImage, tri : &T, color : Rgb<u8>)
where T : crate::geom::d2::prim::refactor::tri::Triangle<Val = f32>
{
    let (width, height) = image.dimensions();

    for x in 0..width {
        for y in 0..height {
            let point = (x as f32, y as f32).vadd(0.5f32);

            if tri.contains_norm(&point) {
                image.put_pixel(x, y, color);
            }
        }
    }
}