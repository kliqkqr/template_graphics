use image::*;

use crate::draw::rast::*;

use crate::file::stl::*;

use crate::geom::d2::prim::PRect;

use crate::geom::d2::prim::seg::{
    PSeg,
    Segment
};

use crate::geom::d2::prim::vect::{
    Vect   as Vect2,
    Vector as Vector2
};

use crate::geom::d3::prim::vect::{
    Vector as Vector3
};

use crate::geom::d3::mesh::rind::{
    IndSegMesh
};

pub fn seg_intsec() {
    let a = (100f32, 100f32);
    let b = (300f32, 300f32);
    let c = (100f32, 300f32);
    let d = (300f32, 100f32);

    let ab = PSeg::new(a, b);
    let cd = PSeg::new(c, d);

    println!("intsec = {:?};", ab.intsec(cd));
}

pub fn fill_rect_shape() {
    let a = (400f32, 400f32);
    let b = (1600f32, 400f32);
    let c = (1600f32, 400f32);
    let d = (400f32, 1600f32);

    let rect = PRect::new_unchecked(a, b, c, d);

    let mut image = RgbImage::new(2000, 2000);
    let color = |_ : Vect2<f32>| Rgb::from([255u8, 255u8, 255u8]);

    fill_shape_float(&mut image, rect, color);

    let save_path = r#"fill_rect_shape.bmp"#;
    let _ = image.save(save_path);
}

pub fn draw_line_segs() {
    let a = (100f32, 100f32);
    let b = (1600f32, 300f32);
    let c = (1400f32, 1800f32);
    let d = (350f32, 1500f32);

    let ab = PSeg::new(a, b);
    let bc = PSeg::new(b, c);
    let cd = PSeg::new(c, d);
    let da = PSeg::new(d, a);

    let mut image = RgbImage::new(2000, 2000);
    let color = |_ : Vect2<f32>| Rgb::from([255u8, 255u8, 255u8]);
    let width = 7f32;

    draw_seg_float(&mut image, ab, color, width);
    draw_seg_float(&mut image, bc, color, width);
    draw_seg_float(&mut image, cd, color, width);
    draw_seg_float(&mut image, da, color, width);

    let save_path = r#"draw_line_segs.bmp"#;
    let _ = image.save(save_path);
}

pub fn stl_to_ind_seg_mesh() -> std::io::Result<()> {
    let stl_path = r#"C:\OneDrive\Code\Bachelor\models\stl\LINK SP-CL 3D\177-200_26.stl"#;
    let stl = Stl::read_binary(stl_path)?;

    let mesh = IndSegMesh::from_stl(&stl);

    let mut mesh = mesh.proj_2d(|vertex| (vertex.x(), vertex.z()));

    let mut image = RgbImage::new(5000, 5000);
    let color = |_ : Vect2<f32>| Rgb::from([255u8, 255u8, 255u8]);
    let width = 3f32;

    let bounds = mesh.bounds();
    let im_bounds = image.dimensions().map::<(f32, f32), _>(|val| val as f32);

    let scales = im_bounds.div(bounds.size());
    let scale = f32::min(scales.x(),scales.y());

    mesh.add_mut(bounds.start().neg());
    mesh.vmul_mut(scale);

    for segment in mesh.segments() {
        let a = mesh.vertex(segment.a());
        let b = mesh.vertex(segment.b());

        let pseg = PSeg::new(a, b);

        draw_seg_float(&mut image, pseg, color, width);
    }

    let save_path = r#"stl_to_ind_seg_mesh.bmp"#;
    let _ = image.save(save_path);

    Ok(())
}

pub fn ing_seg_mesh_contour() -> std::io::Result<()> {
    let stl_path = r#"C:\OneDrive\Code\Bachelor\models\stl\LINK SP-CL 3D\177-202_26.stl"#;
    let stl = Stl::read_binary(stl_path)?;

    let mesh = IndSegMesh::from_stl(&stl);

    let mut mesh = mesh.proj_2d(|vertex| (vertex.x() as f64, vertex.z() as f64));

    let mut image = RgbImage::new(4000, 4000);
    let white = |_ : Vect2<f32>| Rgb::from([255u8, 255u8, 255u8]);
    let red = |_ : Vect2<f32>| Rgb::from([255u8, 0u8, 0u8]);

    let width = 2f32;

    let bounds = mesh.bounds();
    let im_bounds = image.dimensions().map::<(f64, f64), _>(|val| val as f64);

    let scales = im_bounds.div(bounds.size());
    let scale = f64::min(scales.x(),scales.y());

    mesh.add_mut(bounds.start().neg());
    mesh.vmul_mut(scale);

    for segment in mesh.segments() {
        let a = mesh.vertex(segment.a()).map::<(f32, f32), _>(|val| val as f32);
        let b = mesh.vertex(segment.b()).map::<(f32, f32), _>(|val| val as f32);

        let pseg = PSeg::new(a, b);

        draw_seg_float(&mut image, pseg, white, width);
    }

    let contour = mesh.contour().unwrap();

    for index in 0..(contour.len() - 1) {
        let a = (&contour[index]).map::<(f32, f32), _>(|val| val as f32);
        let b = (&contour[index + 1]).map::<(f32, f32), _>(|val| val as f32);

        let pseg = PSeg::new(a, b);

        draw_seg_float(&mut image, pseg, red, width);
    }

    let save_path = r#"ing_seg_mesh_contour.bmp"#;
    let _ = image.save(save_path);

    Ok(())
}

pub fn ing_seg_mesh_rrcontour() -> std::io::Result<()> {
    let stl_path = r#"C:\OneDrive\Code\Bachelor\Utah_teapot_(solid).stl"#;
    let stl = Stl::read_binary(stl_path)?;

    let mesh = IndSegMesh::from_stl(&stl);

    let mut mesh = mesh.proj_2d(|vertex| (vertex.x() as f64, vertex.z() as f64)).deduplicate();

    let white = |_ : Vect2<f32>| Rgb::from([255u8, 255u8, 255u8]);
    let red = |_ : Vect2<f32>| Rgb::from([255u8, 0u8, 0u8]);
    let green = |_ : Vect2<f32>| Rgb::from([0u8, 255u8, 0u8]);

    let width = 2f32;

    let bounds = mesh.bounds();
    let im_bounds = (3800, 3800).map::<(f64, f64), _>(|val| val as f64);

    let scales = im_bounds.div(bounds.size());
    let scale = f64::min(scales.x(),scales.y());

    mesh.add_mut(bounds.start().neg());
    mesh.vmul_mut(scale);
    mesh.add_mut((100f64, 100f64));

    let mut count = 0;

    for (i, v) in mesh.vertices().iter().enumerate() {
        for (j, p) in mesh.vertices().iter().enumerate() {
            if i != j {
                if v.equal(p) {
                    count += 1;
                }
            }
        }
    }

    println!("count = {};", count);

    let min = 0;
    let max = 200;

    let contour = mesh.rrcontour(max).unwrap();

    for index in min..(contour.len() - 1) {
        let mut image = RgbImage::new(4000, 4000);

        for segment in mesh.segments() {
            let a = mesh.vertex(segment.a()).map::<(f32, f32), _>(|val| val as f32);
            let b = mesh.vertex(segment.b()).map::<(f32, f32), _>(|val| val as f32);
    
            let pseg = PSeg::new(a, b);
    
            draw_seg_float(&mut image, pseg, white, width);
        }

        let a = (&contour[index]).map::<(f32, f32), _>(|val| val as f32);
        let b = (&contour[index + 1]).map::<(f32, f32), _>(|val| val as f32);

        let ab = PSeg::new(a, b);

        draw_seg_float(&mut image, ab, red, width + 2f32);

        horizontal_flip(&mut image);

        let save_path = format!("rrcontour_examples\\{}.bmp", index);
        let _ = image.save(save_path);
    }

    println!("\n\ncontour");

    for (i, v) in contour.iter().enumerate() {
        if i < min {
            continue;
        }

        println!("[{:0width$}] x = {}; y = {};", i + 1, v.x(), v.y(), width = 4);
    }

    // let mut image = RgbImage::new(4000, 4000);

    // for segment in mesh.segments() {
    //     let a = mesh.vertex(segment.a()).map::<(f32, f32), _>(|val| val as f32);
    //     let b = mesh.vertex(segment.b()).map::<(f32, f32), _>(|val| val as f32);

    //     let pseg = PSeg::new(a, b);

    //     draw_seg_float(&mut image, pseg, white, width);
    // }

    // for index in 0..(contour.len() - 1) {
    //     let a = (&contour[index]).map::<(f32, f32), _>(|val| val as f32);
    //     let b = (&contour[index + 1]).map::<(f32, f32), _>(|val| val as f32);

    //     let ab = PSeg::new(a, b);

    //     draw_seg_float(&mut image, ab, red, width + 4f32);
    // }


    // horizontal_flip(&mut image);

    // let save_path = format!("rrcontour.bmp");
    // let _ = image.save(save_path);

    // println!("\n\ncontour");

    // for (i, v) in contour.iter().enumerate() {
    //     println!("[{:0width$}] x = {}; y = {};", i + 1, v.x(), v.y(), width = 4);
    // }

    Ok(())
}