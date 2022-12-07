fn main() {
    
}

// #![allow(dead_code)]

// use image::*;

// use itertools::Itertools;

// use tg::draw::rast::*;
// use tg::file::stl::*;
// use tg::geom::d2::prim::*;
// use tg::geom::d3::mesh::ind::*;

// use tg::geom::d2::prim::refactor::tri::{
//     Triangle 
// };

// use rand;

// fn rand_tri(max : f32) -> ((f32, f32), (f32, f32), (f32, f32)) {
//     let a = rand::random::<(f32, f32)>();
//     let b = rand::random::<(f32, f32)>();
//     let c = rand::random::<(f32, f32)>();

//     let a = rand_pnt(max);
//     let b = rand_pnt(max);
//     let c = rand_pnt(max);

//     (a, b, c)
// }

// fn rand_pnt(max : f32) -> (f32, f32) {
//     let (x, y) = rand::random::<(f32, f32)>();
//     (x * max, y * max)
// }

// fn main() {
//     test_2d_mesh();

//     let mut image = RgbImage::new(5000, 5000);

//     let white = Rgb::from([255u8, 255, 255]);
//     let black = Rgb::from([0u8, 0, 0]);
//     let red = Rgb::from([255u8, 0, 0]);

//     for (x, y) in (0..image.width()).into_iter().cartesian_product((0..image.height()).into_iter()) {
//         image.put_pixel(x, y, white);
//     }

//     let path = "C:\\OneDrive\\Code\\Bachelor\\models\\stl\\LINK SP-CL 3D\\177-203_26.stl";
//     let stl = Stl::read_binary(path).expect("coudn't read stl");
 
//     let mut mesh = IndSegMesh::from_stl(&stl).proj_2d(|v| Vect::new(v.x(), v.z())).map(|f| f as f64);
//     let bounds = mesh.bounds().unwrap();

//     let dif = bounds.end() - bounds.start();
//     let scale = if dif.0 < dif.1 { 4980f64 / dif.1 } else { 4980f64 / dif.0 };

//     mesh.vmul_mut(scale);

//     let bounds = mesh.bounds().unwrap();

//     mesh.add_mut(&(-bounds.start() + Vect::new(10f64, 10f64)));

//     let contour = mesh.contour().unwrap();

//     for index in 0..(contour.len() - 1) {
//         let a = mesh.vertex(contour[index]).map(|val| *val as f32);
//         let b = mesh.vertex(contour[index + 1]).map(|val| *val as f32);

//         let seg = PSeg::new(a, b);
//         draw_seg(&mut image, &seg, black, 5f32);
//     }

//     let _ = image.save("contour.bmp");
// }

// fn test_draw() {
//     let mut image = RgbImage::new(1000, 1000);

//     let white = Rgb::from([255u8, 255u8, 255u8]);
//     let red = Rgb::from([255u8, 0u8, 0u8]);
//     let blue = Rgb::from([0, 0, 255u8]);
//     let green = Rgb::from([0, 255u8, 0]);

//     let rect = BRect::new(Vect::new(0f32, 0f32), Vect::new(1000f32, 1000f32));

//     fill_rect(&mut image, &rect, white);

//     let seg = PSeg::new(Vect::new(200f32, 200f32), Vect::new(800f32, 600f32));

//     draw_seg(&mut image, &seg, red, 10f32);

//     let seg = PSeg::new(Vect::new(100f32, 900f32), Vect::new(1900f32, 100f32));

//     draw_seg(&mut image, &seg, blue, -3.14f32);

//     let seg = PSeg::new(Vect::new(100f32, 500f32), Vect::new(900f32, 500f32));

//     draw_seg(&mut image, &seg, green, 1f32);

//     let _ = image.save("test.bmp");
// }

// fn test_tri() {
//     let a = (100f32, 100f32);
//     let b = (900f32, 100f32);
//     let c = (900f32, 900f32);
//     let d = (100f32, 900f32);

//     let t0 = (a, b, c);
//     let t1 = (a, d, c);

//     let c = t0.contains_norm((600f32, 300f32));
//     // println!("contains : {}", c);

//     let mut image = RgbImage::new(1000, 1000);

//     let red = Rgb::from([255u8, 0u8, 0u8]);
//     let blue = Rgb::from([0, 0, 255u8]);

//     let mut image = RgbImage::new(1000, 1000);

//     fill_tri_bary(&mut image, &t0, red);
//     fill_tri_bary(&mut image, &t1, blue);

//     let _ = image.save("tri_bary.bmp");

//     let mut image = RgbImage::new(1000, 1000);

//     fill_tri_norm(&mut image, &t0, red);
//     fill_tri_norm(&mut image, &t1, blue);

//     let _ = image.save("tri_norm.bmp");
// }

// fn test_tri_contains() {
//     let mut bhits = 0;
//     let mut nhits = 0;

//     let mut ts = Vec::with_capacity(25_000_000);

//     for _ in 0..25_000_000 {
//         let t = rand_tri(100f32);
//         let mut ps = Vec::with_capacity(100);

//         for _ in 0..100 {
//             ps.push(rand_pnt(100f32));
//         }

//         ts.push((t, ps));
//     }

//     let bnow = std::time::Instant::now();

//     for (t, ps) in &ts {
//         for p in ps {
//             if t.contains_bary(&p) {
//                 bhits += 1;
//             }
//         }
//     }

//     let belaps = bnow.elapsed();

//     let nnow = std::time::Instant::now();

//     for (t, ps) in &ts {
//         for p in ps {
//             if t.contains_norm(&p) {
//                 nhits += 1;
//             }
//         }
//     }

//     let nelaps = nnow.elapsed();
// }

// fn test_2d_mesh() {
//     let mut image = RgbImage::new(5000, 5000);

//     let white = Rgb::from([255u8, 255, 255]);
//     let black = Rgb::from([0u8, 0, 0]);

//     for (x, y) in (0..image.width()).into_iter().cartesian_product((0..image.height()).into_iter()) {
//         image.put_pixel(x, y, white);
//     }

//     let path = "C:\\OneDrive\\Code\\Bachelor\\models\\stl\\LINK SP-CL 3D\\177-200_26.stl";
//     let stl = Stl::read_binary(path).expect("coudn't read stl");
 
//     let mut mesh = IndSegMesh::from_stl(&stl).proj_2d(|v| Vect::new(v.x(), v.z()));
//     let bounds = mesh.bounds().unwrap();

//     println!("bounds : {:?}", bounds);

//     let dif = bounds.end() - bounds.start();
//     let scale = if dif.0 < dif.1 { 4980f32 / dif.1 } else { 4980f32 / dif.0 };

//     mesh.vmul_mut(scale);

//     let bounds = mesh.bounds().unwrap();

//     mesh.add_mut(&(-bounds.start() + Vect::new(10f32, 10f32)));

//     for seg in mesh.segments() {
//         let vertices = mesh.vertices();

//         let a = vertices[seg.a()].clone();
//         let b = vertices[seg.b()].clone();

//         let seg = PSeg::new(a, b);

//         draw_seg(&mut image, &seg, black, 1f32);
//     }

//     let _ = image.save("mesh.bmp");
// }