use image::*;

use tg::draw::rast::*;
use tg::geom::d2::prim::*;

fn main() {
    let mut image = RgbImage::new(1000, 1000);

    let a = Vect::new(450f32, 100f32);
    let b = Vect::new(550f32, 100f32);

    let s = PSeg::new(a, b);
    let r = 8f32;
    let rect = SRect::new(s, r);

    let color = Rgb::from([255u8, 0u8, 0u8]);

    fill_rect(&mut image, &rect, color);

    let _ = image.save("test.bmp");
}
