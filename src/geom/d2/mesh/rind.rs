use std::vec::{
    Vec
};

use crate::geom::d2::prim::Segment;
use crate::geom::d2::shape::d2::{
    Bounds 
};

use crate::geom::d2::prim::seg::{
    PSeg 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::geom::mesh::ind::{
    IndSeg
};

use crate::num::{
    Zero,
    One,
    Two,
    Float
};

use crate::ops::{
    HAdd,
    HMul
};

use crate::rel::{
    HPOrd 
};

pub struct IndSegMesh<Vect : Vector> {
    vertices : Vec<Vect>,
    segments : Vec<IndSeg>
}

impl<Vect : Vector> IndSegMesh<Vect> {
    pub fn new_unchecked(vertices : Vec<Vect>, segments : Vec<IndSeg>) -> IndSegMesh<Vect> {
        IndSegMesh { vertices: vertices, segments: segments }
    }

    pub fn vertex(&self, index : usize) -> &Vect {
        &self.vertices[index]
    }

    pub fn segments(&self) -> &Vec<IndSeg> {
        &self.segments
    }

    pub fn point_segment(&self, seg : &IndSeg) -> PSeg<Vect::Own> {
        let a = Vect::of(self.vertex(seg.a()));
        let b = Vect::of(self.vertex(seg.b()));

        PSeg::new(a, b)
    }

    pub fn adjacent_vertex_indecis(&self, index : usize) -> Vec<usize> {
        let  mut adjacent_vertex_indecis = Vec::new();

        for segment in &self.segments {
            if segment.a() == segment.b() {
                continue;
            }

            if segment.a() == index {
                adjacent_vertex_indecis.push(segment.b());
                continue;
            }

            if segment.b() == index {
                adjacent_vertex_indecis.push(segment.a());
                continue;
            }
        }

        adjacent_vertex_indecis
    }

    pub fn bounds(&self) -> Bounds<Vect::Own> 
    where Vect::Val : HPOrd
    {
        let mut start = Vect::of(&self.vertices[0]);
        let mut end = Vect::of(&self.vertices[0]);

        for vertex in &self.vertices {
            start = start.min(vertex);
            end   = end.max(vertex);
        }

        Bounds::new_unchecked(start, end)
    }

    pub fn contour(&self) -> Option<Vec<usize>> 
    where Vect::Val : Zero + One + Two + Float + HPOrd
    {   
        let mut start_index = 0;
        let mut start_max_x = self.vertices.first()?.x();

        for index in 0..self.vertices.len() {
            let vertex = self.vertex(index);

            if vertex.x() > start_max_x {
                start_index = index;
                start_max_x = vertex.x();
            }
        }

        let adjacent_indecis = self.adjacent_vertex_indecis(start_index);
        let x_axis_direction = (Vect::Val::one(), Vect::Val::zero());
        let start_vertex = self.vertex(start_index);

        let mut next_index = *adjacent_indecis.first()?;
        let mut next_min_angle = x_axis_direction.angle_l(self.vertex(next_index).sub(start_vertex));

        for index in adjacent_indecis {
            let angle = x_axis_direction.angle_l(self.vertex(index).sub(start_vertex));

            if angle < next_min_angle {
                next_index = index;
                next_min_angle = angle;
            }
        }

        let mut last_index = start_index;
        let mut curr_index = next_index;

        let mut contour_indecis = vec![start_index, next_index];

        loop {
            let last_vertex = self.vertex(last_index);
            let curr_vertex = self.vertex(curr_index);

            let adjacent_indecis = self.adjacent_vertex_indecis(curr_index);
            let last_curr_direction = curr_vertex.sub(last_vertex);

            let mut next_index = *adjacent_indecis.iter().find(|index| **index != last_index).unwrap();
            let mut next_min_angle = last_curr_direction.angle_l(curr_vertex.sub(self.vertex(next_index)));

            for index in adjacent_indecis {
                if index == last_index {
                    continue;
                }

                let vertex_curr_direction = curr_vertex.sub(self.vertex(index));
                let angle = last_curr_direction.angle_l(&vertex_curr_direction);

                if angle < next_min_angle {
                    next_index = index;
                    next_min_angle = angle;
                }
            }

            let curr_next_pseg = PSeg::new(curr_vertex, self.vertex(next_index));
            let curr_next_direction = curr_next_pseg.ab();

            let mut best = None;

            for iseg in self.segments() {       
                if iseg.a() == curr_index || iseg.a() == next_index || iseg.b() == curr_index || iseg.b() == next_index {
                    continue
                }
                
                let pseg = self.point_segment(iseg);

                if let Some(intsec) = curr_next_pseg.intsec(&pseg) {
                    let curr_a_direction = pseg.a().sub(curr_vertex);
                    let curr_b_direction = pseg.b().sub(curr_vertex);

                    let angle_a = curr_next_direction.angle_l(curr_a_direction);
                    let angle_b = curr_next_direction.angle_l(curr_b_direction);

                    let (angle, index) = if angle_a < angle_b { (angle_a, iseg.a()) } else { (angle_b, iseg.b()) };

                    best = match best {
                        None => Some((angle, index, intsec)),
                        Some((best_angle, _, _)) => if best_angle < angle { best } else { Some((angle, index, intsec)) }
                    }
                }
            }

            match best {
                None => (),
                Some((_, index, intsec)) => todo!(),
            }

            if next_index == contour_indecis[0] {
                break;
            }

            contour_indecis.push(next_index);

            last_index = curr_index;
            curr_index = next_index;
        }

        // loop {
        //     let last_vertex = self.vertex(last_index);
        //     let curr_vertex = self.vertex(curr_index);

        //     let adjacent_indecis = self.adjacent_vertex_indecis(curr_index)
        //         .into_iter()
        //         .filter(|index| !contour_indecis.contains(index))
        //         .collect::<Vec<usize>>();

        //     let last_curr_direction = curr_vertex.sub(last_vertex);

        //     let mut next_index = match adjacent_indecis.iter().find(|index| **index != last_index) {
        //         None => return Some(contour_indecis),
        //         Some(index) => *index
        //     };

        //     let mut next_min_angle = last_curr_direction.angle_l(curr_vertex.sub(self.vertex(next_index)));

        //     for index in &adjacent_indecis {
        //         let index = *index;

        //         if index == last_index {
        //             continue;
        //         }

        //         let vertex_curr_direction = curr_vertex.sub(self.vertex(index));
        //         let angle = last_curr_direction.angle_l(&vertex_curr_direction);

        //         if angle < next_min_angle {
        //             next_index = index;
        //             next_min_angle = angle;
        //         }
        //     }

        //     if next_index == contour_indecis[0] {
        //         break;
        //     }

        //     contour_indecis.push(next_index);

        //     last_index = curr_index;
        //     curr_index = next_index;
        // }

        Some(contour_indecis)
    }
}

impl<Vect : Vector<Own = Vect>> IndSegMesh<Vect> {
    pub fn add_mut<V : Vector<Val = Vect::Val>>(&mut self, vect : V)
    where Vect::Val : HAdd
    {
        for v in &mut self.vertices {
            *v = v.add(&vect)
        }
    }

    pub fn vmul_mut(&mut self, val : Vect::Val) 
    where Vect::Val : HMul
    {
        for v in &mut self.vertices {
            *v = v.vmul(val);
        }
    }
}