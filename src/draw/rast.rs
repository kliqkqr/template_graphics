use image::{
    Rgb,
    RgbImage
};

use crate::conv::{
    Cast
};

use crate::geom::d2::prim::PRect;
use crate::geom::d2::shape::d2::{
    Bounds, 
    Shape
};

use crate::geom::d2::prim::seg::{
    Segment 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::num::{
    Zero,
    One,
    Two,
    Float
};

use crate::rel::{
    HPOrd
};

pub fn horizontal_flip(image : &mut RgbImage) {
    let (width, height) = image.dimensions();

    for y in 0 .. height / 2 {
        for x in 0 .. width {
            let a = image.get_pixel(x, y).clone();
            let b = image.get_pixel(x, height - y - 1).clone();

            image.put_pixel(x, y, b);
            image.put_pixel(x, height - y - 1, a);
        }
    }
}

pub fn fill_shape_float<Sh : Shape, Func : Fn(<Sh::Vect as Vector>::Own) -> Rgb<u8>>(image : &mut RgbImage, shape : Sh, color : Func) 
where Sh::Val : One + Two + Float + Cast<u32>,
      u32     : Cast<Sh::Val>
{   
    let half = Sh::Val::one() / Sh::Val::two();

    let im_bounds = Bounds::with_zero_unchecked(image.dimensions());
    let sh_bounds = shape.bounds().vmap::<(u32, u32), _>(|val| val.cast());

    let bounds = im_bounds.clamp(sh_bounds);
    let start = bounds.start();
    let end   = bounds.end();

    for x in start.x()..end.x() {
        for y in start.y()..end.y() {
            let point = <Sh::Vect as Vector>::of((u32::cast(x), u32::cast(y))).vadd(half);

            if shape.contains(&point) {
                let pixel = color(point);
                image.put_pixel(x, y, pixel);
            }
        }
    }
}

pub fn draw_seg_float<Seg : Segment, Func : Fn(<Seg::Vect as Vector>::Own) -> Rgb<u8>>(image : &mut RgbImage, seg : Seg, color : Func, width : Seg::Val)
where Seg::Val  : Zero + One + Two + Float + Cast<u32> + HPOrd,
      u32       : Cast<Seg::Val>
{
    let half_width = width / Seg::Val::two();

    let seg_ab = seg.ab();
    let scalar = half_width / seg_ab.len();

    let offset_l = seg_ab.orth_l().vmul(scalar);
    let offset_r = seg_ab.orth_r().vmul(scalar);

    let seg_a = seg.a();
    let seg_b = seg.b();
    
    let a = seg_a.add(&offset_l);
    let b = seg_b.add(&offset_l);
    let c = seg_b.add(&offset_r);
    let d = seg_a.add(&offset_r);

    let rect = PRect::new_unchecked(a, b, c, d);
    fill_shape_float(image, rect, color);
}