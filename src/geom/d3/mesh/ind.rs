// use std::vec::{
//     Vec 
// };

// use itertools::{
//     Itertools 
// };

// use crate::file::stl::{
//     Stl,
//     Tri
// };

// use crate::geom::{
//     d2
// };

// use crate::geom::d3::prim::vect::{
//     Vect 
// };

// use crate::geom::mesh::ind::{
//     IndSeg
// };

// use crate::ops::{
//     HMul
// };

// pub struct IndSegMesh<A> {
//     vertices : Vec<Vect<A>>,
//     segments : Vec<IndSeg>
// }

// impl<A> IndSegMesh<A> {
//     pub fn new_unchecked(vertices : Vec<Vect<A>>, lines : Vec<IndSeg>) -> IndSegMesh<A> {
//         IndSegMesh{vertices : vertices, segments : lines}
//     }

//     pub fn vertices(&self) -> &Vec<Vect<A>> {
//         &self.vertices
//     }

//     pub fn segments(&self) -> &Vec<IndSeg> {
//         &self.segments
//     }

//     pub fn vmul(&self, val : A) -> IndSegMesh<A>
//     where A : Copy + HMul 
//     {
//         let vertices = self.vertices().iter().map(|v| v.vmul(val)).collect();
//         let lines = self.segments().clone();

//         IndSegMesh::new_unchecked(vertices, lines)
//     }

//     pub fn proj_2d<F : Fn(&Vect<A>) -> d2::prim::Vect<A>>(&self, proj : F) -> d2::mesh::ind::IndSegMesh<A> {
//         let vertices = self.vertices.iter().map(proj).collect();
//         let lines = self.segments().clone();

//         d2::mesh::ind::IndSegMesh::new_unchecked(vertices, lines)
//     }
// }

// impl IndSegMesh<f32> {
//     pub fn from_stl(stl : &Stl) -> IndSegMesh<f32> {
//         let mut vertices = Vec::new();
//         let mut lines = Vec::new();

//         for triangle in stl.triangles() {
//             let index_a = vertices.len();
//             let index_b = index_a + 1;
//             let index_c = index_a + 2;

//             vertices.extend(triangle.vertices());

//             let ab = IndSeg::new(index_a, index_b);
//             let bc = IndSeg::new(index_b, index_c);
//             let ca = IndSeg::new(index_c, index_a);

//             lines.push(ab);
//             lines.push(bc);
//             lines.push(ca);
//         }

//         let mut swaps = Vec::new();

//         for (index, vertex) in vertices.iter().enumerate() {
//             if swaps.iter().any(|(j, _)| *j == index) || index == vertices.len() - 1 {
//                 continue
//             }

//             let slice = vertices[index + 1..].as_ref();
//             for (j, v) in slice.iter().enumerate() {
//                 if vertex == v {
//                     swaps.push((index + j + 1, index));
//                 }
//             }
//         }

//         swaps.sort_by(|a, b| b.0.cmp(&a.0));

//         for (from, to) in swaps {
//             vertices.remove(from);

//             for line in &mut lines {
//                 line.swap_shift(from, to);
//             }
//         }

//         let lines = lines
//             .into_iter()
//             .unique()
//             .collect::<Vec<IndSeg>>();

//         IndSegMesh::new_unchecked(vertices, lines)
//     }
// }