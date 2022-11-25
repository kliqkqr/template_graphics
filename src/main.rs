use image::*;

use tg::draw::rast::*;
use tg::geom::d2::prim::*;

fn main() {
    let mut image = RgbImage::new(1000, 1000);

    let red = Rgb::from([255u8, 0u8, 0u8]);
    let blue = Rgb::from([0, 0, 255u8]);
    let green = Rgb::from([0, 255u8, 0]);

    let seg = PSeg::new(Vect::new(200f32, 200f32), Vect::new(800f32, 600f32));

    draw_seg(&mut image, &seg, red, 10f32);

    let seg = PSeg::new(Vect::new(100f32, 900f32), Vect::new(1900f32, 100f32));

    draw_seg(&mut image, &seg, blue, -3.14f32);

    let seg = PSeg::new(Vect::new(100f32, 500f32), Vect::new(900f32, 500f32));

    draw_seg(&mut image, &seg, green, 1f32);

    let _ = image.save("test.bmp");
}
