use std::vec::{
    Vec 
};


use crate::file::stl::{
    Stl 
};

use crate::geom::{
    d2
};

use crate::geom::d3::prim::vect::{
    Vect,
    Vector 
};

use crate::geom::mesh::ind::{
    IndSeg 
};

pub struct IndSegMesh<Vect : Vector> {
    vertices : Vec<Vect>,
    segments : Vec<IndSeg>
}

impl<Vect : Vector> IndSegMesh<Vect> {
    pub fn new_unchecked(vertices : Vec<Vect>, segments : Vec<IndSeg>) -> IndSegMesh<Vect> {
        IndSegMesh{vertices : vertices, segments : segments}
    }

    pub fn vertices(&self) -> &Vec<Vect> {
        &self.vertices 
    }

    pub fn segments(&self) -> &Vec<IndSeg> {
        &self.segments
    }
}

impl IndSegMesh<Vect<f32>> {
    pub fn from_stl(stl : &Stl) -> IndSegMesh<Vect<f32>> {
        let mut vertices = Vec::new();
        let mut segments = Vec::new();

        let mut maybe_push_vertex_and_get_index = |vertex : Vect<f32>| {
            match vertices.iter().position(|v| vertex.equal(v)) {
                None => {
                    let index = vertices.len();
                    vertices.push(vertex);
                    index 
                },
                Some(index) => index
            }
        };

        let mut maybe_push_segment = |segment : IndSeg| {
            if !segments.iter().any(|s| segment.equiv(s)) {
                segments.push(segment)
            }
        };

        for triangle in stl.triangles() {
            let [a, b, c] = triangle.vertices();

            let a = maybe_push_vertex_and_get_index(a);
            let b = maybe_push_vertex_and_get_index(b);
            let c = maybe_push_vertex_and_get_index(c);

            let ab = IndSeg::new(a, b);
            let bc = IndSeg::new(b, c);
            let ca = IndSeg::new(c, a);

            maybe_push_segment(ab);
            maybe_push_segment(bc);
            maybe_push_segment(ca);
        }

        IndSegMesh::new_unchecked(vertices, segments)
    }
}

impl<Vect : Vector> IndSegMesh<Vect> {
    pub fn proj_2d<V : d2::prim::Vector, Func : Fn(&Vect) -> V>(&self, proj : Func) -> d2::mesh::ind::IndSegMesh<V> {
        let vertices = self.vertices.iter().map(|vertex| proj(vertex)).collect();
        let segments = self.segments.clone();

        d2::mesh::ind::IndSegMesh::new_unchecked(vertices, segments)
    }
}