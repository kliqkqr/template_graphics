#![allow(dead_code)]

/// module for conversions
pub mod conv;

/// module for drawing functionality
pub mod draw;

/// module for file functionality
pub mod file;

/// module for geometry
pub mod geom;

/// module for number operations
pub mod num;

/// module for arithmetic operations
pub mod ops;

/// module for iteration operations
pub mod range;

/// module for relations
pub mod rel;

/// module for tests
pub mod test;

use crate::geom::d2::prim::{
    Vector as Vector2
};

use crate::geom::d3::prim::{
    Vector as Vector3
};

#[repr(C)]
pub struct StielDaten2D {
    contour_values     : *mut f64,
    contour_values_len : usize,
    point_values       : *mut f64,
    point_values_len   : usize
}

#[no_mangle]
pub unsafe extern "C" fn rust_test(byte : u8, nat : usize, nat32 : u32) {
    println!("rust byte={byte} nat={nat} nat32={nat32}");
}

#[no_mangle]
pub unsafe extern "C" fn rust_generateTStielDaten2D(
    vertex_values      : *mut f64, 
    vertex_values_len  : usize,
    semgent_values     : *mut usize,
    segment_values_len : usize,
    point_values       : *mut f64,
    point_values_len   : usize,
    inklination        : f64,
    anteversion        : f64,
    stiel_daten        : *mut StielDaten2D
    ) -> bool
{   
    if vertex_values_len % 3 != 0 || segment_values_len % 2 != 0 || point_values_len % 3 != 0 {
        return false;
    }

    let vertices_len = vertex_values_len  / 3;
    let segments_len = segment_values_len / 2;
    let points_len   = point_values_len   / 3;

    let mut vertices = Vec::with_capacity(vertices_len);
    let mut segments = Vec::with_capacity(segments_len);
    let mut points   = Vec::with_capacity(points_len);

    for i in 0..vertices_len {
        let offset = i as isize * 3;

        let x = *vertex_values.offset(offset + 0);
        let y = *vertex_values.offset(offset + 1);
        let z = *vertex_values.offset(offset + 2);

        let vertex = (x, y, z);
        vertices.push(vertex);
    }

    for i in 0..segments_len {
        let offset = i as isize * 2;

        let a = *semgent_values.offset(offset + 0);
        let b = *semgent_values.offset(offset + 1);

        let segment = crate::geom::mesh::ind::IndSeg::new(a, b);
        segments.push(segment);
    }

    for i in 0..points_len {
        let offset = i as isize * 3;

        let x = *point_values.offset(offset + 0);
        let y = *point_values.offset(offset + 1);
        let z = *point_values.offset(offset + 2);

        let point = (x, y, z);
        points.push(point);
    }

    let indexed_mesh_3d = crate::geom::d3::mesh::ind::IndSegMesh::new_unchecked(vertices, segments);
    let indexed_mesh_3d = indexed_mesh_3d.rotate_y(inklination).rotate_z(anteversion);

    let indexed_mesh_2d = indexed_mesh_3d.proj_2d(|vertex| (vertex.x(), vertex.z()));

    let Some(contour) = indexed_mesh_2d.rrcontour(10_000) else {
        return false
    };

    let contour_len = contour.len() * 2;

    let contour_values = libc::malloc(std::mem::size_of::<f64>() * contour_len) as *mut f64;
    let points_values  = 0 as *mut f64;

    for i in 0..contour.len() {
        let offset = i as isize * 2;
        let vertex = contour[i];

        *contour_values.offset(offset + 0) = vertex.x();
        *contour_values.offset(offset + 1) = vertex.y();
    }

    *stiel_daten = StielDaten2D {
        contour_values:     contour_values,
        contour_values_len: contour_len,
        point_values:       points_values, 
        point_values_len:   0     
    };

    true 
}

#[no_mangle]
pub unsafe extern "C" fn rust_loadFromBinaryStl(
    vertex_values      : *mut *mut f64, 
    vertex_values_len  : *mut usize,
    segment_values     : *mut *mut usize,
    segment_values_len : *mut usize,
    path_chars         : *mut u8,
    path_chars_len     : usize
    ) -> bool
{  
    let path_slice = std::slice::from_raw_parts_mut(path_chars, path_chars_len);

    let Ok(path) = std::str::from_utf8(path_slice) else {
        return false;
    };

    let Ok(stl) = crate::file::stl::Stl::read_binary(path) else {
        return false;
    };

    let indexed_mesh = crate::geom::d3::mesh::ind::IndSegMesh::from_stl(&stl);

    let v_len = indexed_mesh.vertices().len() * 3;
    let s_len = indexed_mesh.segments().len() * 2;

    let vertex_values_ptr  = libc::malloc(std::mem::size_of::<f64>()   * v_len) as *mut f64;
    let segment_values_ptr = libc::malloc(std::mem::size_of::<usize>() * s_len) as *mut usize;

    for i in 0..indexed_mesh.vertices().len() {
        let vertex = &indexed_mesh.vertices()[i];
        
        let i = i as isize * 3;
        *vertex_values_ptr.offset(i + 0) = vertex.x() as f64;
        *vertex_values_ptr.offset(i + 1) = vertex.y() as f64;
        *vertex_values_ptr.offset(i + 2) = vertex.z() as f64;
    }

    for i in 0..indexed_mesh.segments().len() {
        let segment = &indexed_mesh.segments()[i];

        let i = i as isize * 2;
        *segment_values_ptr.offset(i + 0) = segment.a();
        *segment_values_ptr.offset(i + 1) = segment.b();
    }

    *vertex_values_len  = v_len;
    *segment_values_len = s_len;

    *vertex_values  = vertex_values_ptr;
    *segment_values = segment_values_ptr;

    true
}