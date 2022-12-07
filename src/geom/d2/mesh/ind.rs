// use std::vec::{
//     Vec 
// };

// use std::ops::{
//     Range
// };

// use crate::geom::d2::prim::rect::{
//     BRect
// };

// use crate::geom::d2::prim::vect::{
//     Vect,
//     Vector 
// };

// use crate::geom::mesh::ind::{
//     IndSeg
// };

// use crate::num::{
//     Zero, 
//     One,
//     Two,
//     Float
// };

// use crate::ops::{
//     HAdd,
//     HSub,
//     HMul,
//     HDiv
// };

// use crate::rel::{
//     POrd 
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

//     pub fn vertex_indecis(&self) -> Range<usize> {
//         0..self.vertices().len()
//     }

//     pub fn segments(&self) -> &Vec<IndSeg> {
//         &self.segments
//     }

//     pub fn add_mut(&mut self, vect : &Vect<A>) 
//     where A : Clone + HAdd
//     {
//         for v in &mut self.vertices {
//             v.add_mut(vect);
//         }
//     }

//     pub fn vmul(&self, val : A) -> IndSegMesh<A>
//     where A : Copy + HMul 
//     {
//         let vertices = self.vertices().iter().map(|v| v.vmul(val)).collect();
//         let lines = self.segments().iter().map(|l| l.clone()).collect();

//         IndSegMesh::new_unchecked(vertices, lines)
//     }

//     pub fn vmul_mut(&mut self, val : A) 
//     where A : Clone + HMul 
//     {
//         for v in &mut self.vertices {
//             v.vmul_mut(val.clone())
//         }
//     }

//     pub fn bounds(&self) -> Option<BRect<A>>
//     where A : Clone + POrd
//     {   
//         self.vertices().iter().fold(None, |opt, v| {
//             match opt { 
//                 None => Some(BRect::new_unchecked(v.clone(), v.clone())),
//                 Some(brect) => Some(brect.bounds_with(v))
//             }
//         })
//     }
    
//     pub fn vertex(&self, index : usize) -> &Vect<A> {
//         &self.vertices()[index]
//     }

//     pub fn adjacent_vertices(&self, index : usize) -> Vec<usize> {
//         self.segments()
//             .iter()
//             .filter_map(|seg| {
//                 if seg.a() == seg.b() {
//                     return None 
//                 }

//                 if seg.a() == index {
//                     return Some(seg.b())
//                 }

//                 if seg.b() == index {
//                     return Some(seg.a())
//                 }
                
//                 None
//             })
//             .collect()
//     }

//     pub fn contour(&self) -> Option<Vec<usize>> 
//     where A : Clone + HAdd + HSub + HMul + HDiv + POrd + Float + Zero + One + Two
//     {       
//         // find start vertex index with max x value
//         let mut indecis = self.vertex_indecis();
//         let first = indecis.next()?;

//         let start = indecis
//             .fold(first, |start, index| {
//                 match self.vertex(start).0 < self.vertex(index).0 {
//                     false => start,
//                     true  => index
//                 }
//             });

//         // find next vertex index in contour with min left angle to positive x axis
//         let adj_indecis = self.adjacent_vertices(start);
//         let next = *adj_indecis.first()?;
//         let start_next = self.vertex(next) - self.vertex(start);
//         let angle = Vect::new(A::one(), A::zero()).angle_l(&start_next);

//         let next = adj_indecis
//             .iter()
//             .fold((first, angle), |(next, angle), index| {
//                 let start_index = self.vertex(*index) - self.vertex(start);
//                 let index_angle = Vect::new(A::one(), A::zero()).angle_l(&start_index);

//                 match index_angle < angle {
//                     false => (next, angle),
//                     true  => (*index, index_angle)
//                 }
//             }).0;

//         // find contour points until start vertex is found again
//         let mut contour = vec![start, next];
//         while contour.last()? != contour.first()? {
//             // last 2 found vertix indecis
//             let index_a = contour[contour.len() - 2];
//             let index_b = contour[contour.len() - 1];

//             // last 2 found vertices
//             let a = self.vertex(index_a);
//             let b = self.vertex(index_b);

//             // direction vector of last found vertices
//             let ba = a - b;

//             // vertices adjacent to last found vertex without penultimate vertex a
//             let adjs = self.adjacent_vertices(index_b)
//                 .into_iter()
//                 .filter(|index| *index != index_a)
//                 .collect::<Vec<usize>>();

//             // find next vertex in contour with min left angle to direction vector ba
//             let next = adjs.first()?;
//             let bn = self.vertex(*next) - b;
//             let angle = ba.angle_l(&bn);

//             let next = adjs
//                 .iter()
//                 .fold((next, angle), |(next, angle), index| {
//                     let bi = self.vertex(*index) - b;
//                     let angle_i = ba.angle_l(&bi);

//                     match angle_i < angle {
//                         false => (next, angle),
//                         true  => (index, angle_i)
//                     }
//                 }).0;

//             // add found next vertex to contour
//             contour.push(*next);
//         }

//         Some(contour)
//     }

//     pub fn map<F : Fn(A) -> B, B>(&self, func : F) -> IndSegMesh<B> 
//     where A : Clone
//     {
//         let vertices = self.vertices().iter().map(|vect| Vect::new(func(vect.0.clone()), func(vect.1.clone()))).collect();
//         IndSegMesh::new_unchecked(vertices, self.segments().clone())
//     }
// }