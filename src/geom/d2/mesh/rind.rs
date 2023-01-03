use std::vec::{
    Vec
};

use rand::seq::index;

use crate::geom::d2::prim::{
    Segment
};

use crate::geom::d2::prim::seg::{
    PSeg 
};

use crate::geom::d2::prim::vect::{
    Vector 
};

use crate::geom::d2::shape::d2::{
    Bounds 
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
    HPEq,
    HPOrd 
};

struct ContourAdjacentResult<Vect : Vector> {
    index  : usize,
    vector : Vect,
    angle  : Vect::Val,
    length : Option<Vect::Val>,
}
struct ContourIntersectionResult<Vect : Vector> {
    intersection : Vect,
    distance     : Vect::Val,
    left_vertex  : Vect,
    left_index   : usize,
    right_vertex : Vect,
    right_index  : usize
}

enum ContourStepOrigin {
    Mesh{current_index : usize},
    Intersection{last_left_index : usize, right_index : usize, left_index : usize}
}

struct ContourStep<Vect : Vector> {
    last_right_index : usize,
    last_vertex      : Vect,
    current_vertex   : Vect,
    origin           : ContourStepOrigin
}

enum RContourVertexOrigin {
    Mesh{ index : usize },
    Intersection{ left_index : usize, right_index : usize }
}

struct RContourVertex<Vect : Vector> {
    vertex : Vect,
    origin : RContourVertexOrigin
}

pub struct IndSegMesh<Vect : Vector> {
    vertices : Vec<Vect>,
    segments : Vec<IndSeg>
}

impl<Vect : Vector> ContourAdjacentResult<Vect> {
    /// Returns the result with the smaller angle. If both angles are equal returns result with smaller vector length
    pub fn choose(result_a : ContourAdjacentResult<Vect>, result_b : ContourAdjacentResult<Vect>) -> ContourAdjacentResult<Vect> 
    where Vect::Val : Float + HPOrd
    {
        if result_a.angle < result_b.angle {
            result_a 
        }
        else if result_a.angle == result_b.angle {
            let length_a = match result_a.length {
                None => result_a.vector.len(),
                Some(length) => length
            };

            let length_b = match result_b.length {
                None => result_b.vector.len(),
                Some(length) => length 
            };

            if length_a < length_b {
                result_a 
            }
            else {
                result_b
            }
        }   
        else {
            result_b
        }
    }
}

impl<Vect : Vector> ContourIntersectionResult<Vect> {
    fn choose(result_a : ContourIntersectionResult<Vect>, result_b : ContourIntersectionResult<Vect>, direction : Vect) -> ContourIntersectionResult<Vect> 
    where Vect::Val : Float + HPOrd
    {
        if result_a.distance < result_b.distance {
            result_a 
        }
        else if result_a.distance == result_b.distance {
            let vector_a = result_a.right_vertex.sub(&result_a.left_vertex);
            let vector_b = result_b.right_vertex.sub(&result_b.left_vertex);

            let angle_a = direction.angle_l(vector_a);
            let angle_b = direction.angle_l(vector_b);

            if angle_a < angle_b {
                result_a
            }
            else if angle_a == angle_b {
                let distance_a = result_a.intersection.sub(&result_a.left_vertex).len();
                let distance_b = result_b.intersection.sub(&result_b.left_vertex).len();

                if distance_a <= distance_b {
                    result_a
                }
                else {
                    result_b
                }
            }
            else {
                result_b
            }
        }
        else {
            result_b
        }
    }
}

impl<Vect : Vector> IndSegMesh<Vect> {
    pub fn new_unchecked(vertices : Vec<Vect>, segments : Vec<IndSeg>) -> IndSegMesh<Vect> {
        IndSegMesh { vertices: vertices, segments: segments }
    }

    pub fn vertices(&self) -> &Vec<Vect> {
        &self.vertices
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

    pub fn contour(&self) -> Option<Vec<Vect::Own>> 
    where Vect::Val : Zero + One + Two + Float + HPOrd + std::fmt::Debug
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

        let mut last_index = Some(start_index);
        let mut curr_index = next_index;

        let last_vertex = Vect::of(self.vertex(start_index));
        let curr_vertex = Vect::of(self.vertex(curr_index));

        let mut contour = vec![last_vertex, curr_vertex];

        loop {
            println!("last_index = {:?};", last_index);

            let last_vertex = &contour[contour.len() - 2];
            let curr_vertex = &contour[contour.len() - 1];

            let adjacent_indecis = self.adjacent_vertex_indecis(curr_index);
            let last_curr_direction = curr_vertex.sub(last_vertex);

            let mut next_index = match last_index {
                None => *adjacent_indecis.first()?,
                Some(last_index) => *adjacent_indecis.iter().find(|index| **index != last_index)?
            };

            let mut next_min_angle = last_curr_direction.angle_l(curr_vertex.sub(self.vertex(next_index)));

            for index in adjacent_indecis {
                if last_index.map_or(false, |last_index| last_index == index) {
                    continue;
                }

                let vertex_curr_direction = curr_vertex.sub(self.vertex(index));
                let angle = last_curr_direction.angle_l(&vertex_curr_direction);

                if angle < next_min_angle {
                    next_index = index;
                    next_min_angle = angle;
                }
            }

            let next_vertex = Vect::of(self.vertex(next_index));
            let curr_next_pseg = PSeg::new(curr_vertex, &next_vertex);
            let curr_next_direction = curr_next_pseg.ab();

            let mut best = None;

            for iseg in self.segments() {       
                if iseg.a() == curr_index || iseg.a() == next_index || iseg.b() == curr_index || iseg.b() == next_index {
                    continue
                }
                
                let pseg = self.point_segment(iseg);

                if let Some(intsec) = curr_next_pseg.intsec(&pseg) {
                    // let curr_a_direction = pseg.a().sub(&curr_vertex);
                    // let curr_b_direction = pseg.b().sub(&curr_vertex);

                    // let angle_a = curr_next_direction.angle_l(curr_a_direction);
                    // let angle_b = curr_next_direction.angle_l(curr_b_direction);

                    let a_curr_direction = curr_vertex.sub(pseg.a());
                    let b_curr_direction = curr_vertex.sub(pseg.b());
                    
                    let angle_a = curr_next_direction.angle_l(a_curr_direction);
                    let angle_b = curr_next_direction.angle_l(b_curr_direction);

                    let (angle, index) = if angle_a < angle_b { (angle_a, iseg.a()) } else { (angle_b, iseg.b()) };

                    best = match best {
                        None => Some((angle, index, intsec)),
                        Some((best_angle, _, _)) => if best_angle < angle { best } else { Some((angle, index, intsec)) }
                    }
                }
            }

            match best {
                None => {
                    last_index = Some(curr_index);
                    curr_index = next_index;
                },
                Some((_, index, intsec)) => {
                    contour.push(Vect::of(&intsec));

                    last_index = None;
                    curr_index = index;
                },
            }

            let next_vertex = Vect::of(self.vertex(curr_index));

            if next_vertex.equal(&contour[0]) {
                break;
            }

            contour.push(next_vertex);

            if contour.len() > 1000 {
                break;
            }
        }  

        Some(contour)
    }

    pub fn rrcontour(&self, max : usize)  -> Option<Vec<Vect::Own>>
    where Vect::Val : Float + HPOrd + std::fmt::Debug
    {   
        // Result from vertices loop. Contains vertex with min x coordinate and it's index
        let mut min_x_result = None;

        // iterator over all vertices to find min x coordinate vertex
        for (index, vertex) in self.vertices().iter().enumerate() {
            min_x_result = match min_x_result {
                None => Some((index, vertex)),
                Some((best_index, best_vertex)) => if best_vertex.x() <= vertex.x() { Some((best_index, best_vertex)) } else { Some((index, vertex)) }
            }
        }

        // return None if mesh contains no vertices
        let (first_index, first_vertex) = min_x_result?;

        let mut step = ContourStep {
            last_right_index : first_index,
            last_vertex      : first_vertex.add((-Vect::Val::one(), Vect::Val::one())),
            current_vertex   : Vect::of(self.vertex(first_index)),
            origin           : ContourStepOrigin::Mesh { 
                current_index : first_index
            }
        };

        // contour that is returned at end
        let mut contour = vec![Vect::of(first_vertex)];

        // loop until last found vertix is first vertex
        loop {
            let mut commands : Vec<String> = Vec::new();

            let vs = |v : Vect::Own| format!("({:?}, {:?})", v.x(), v.y());

            let last_vertex    = &step.last_vertex;
            let current_vertex = &step.current_vertex;

            let cmd = format!("L = {}", vs(Vect::of(last_vertex)));
            commands.push(cmd);

            let cmd = format!("C = {}", vs(Vect::of(current_vertex)));
            commands.push(cmd);

            // vector from current vertex to last vertex
            let current_last_vector = last_vertex.sub(current_vertex);

            let adjacent_index = match step.origin {
                ContourStepOrigin::Mesh { current_index } => {
                    let mut index = 0;

                    // result from adjacent vertex loop
                    let mut adjacent_result_option = None;

                    // iterate over all adjacent vertices to find vertex with smallest left angle to current_last_vector
                    for adjacent_vertex_index in self.adjacent_vertex_indecis(current_index) {  
                        index += 1;


                        // ignore last found adjacent vertex
                        if step.last_right_index == adjacent_vertex_index {
                            continue;
                        }

                        // vertex adjacent to current vertex
                        let adjacent_vertex = self.vertex(adjacent_vertex_index);

                        // vector from current vertex to adjacent vertex
                        let current_adjacent_vector = adjacent_vertex.sub(current_vertex);

                        // left angle between vector from current vertex to last vertex and current vertex to adjacent vertex
                        let left_adjacent_angle = current_last_vector.angle_l(&current_adjacent_vector);

                        // create new result
                        let result = ContourAdjacentResult {
                            index  : adjacent_vertex_index,
                            vector : current_adjacent_vector,
                            angle  : left_adjacent_angle,
                            length : None
                        };

                        let cmd = format!("A{} = {}", index, vs(Vect::of(&adjacent_vertex)));
                        commands.push(cmd);

                        let cmd = format!("A{}S = Segment(C, A{})", index, index);
                        commands.push(cmd);

                        // update result option with result with smaller angle / smaller vector length if angles are equal
                        adjacent_result_option = match adjacent_result_option {
                            None => Some(result),
                            Some(best_result) => Some(ContourAdjacentResult::choose(result, best_result))
                        }
                    }

                    adjacent_result_option?.index
                },
                ContourStepOrigin::Intersection{left_index, ..} => left_index,
            };

            let mut index = 0;

            let cmd = format!("A = {}", vs(Vect::of(self.vertex(adjacent_index))));
            commands.push(cmd);

            let cmd = format!("L'C = Segment(L, C)");
            commands.push(cmd);

            let cmd = format!("C'A = Segment(C, A)");
            commands.push(cmd);

            // adjacent vertex
            let adjacent_vertex = self.vertex(adjacent_index);

            // vector from adjacent_vertex to current_vertex
            let adjacent_current_vector = current_vertex.sub(adjacent_vertex);

            // line segment from current vertex to adjacent result vertex
            let current_adjacent_segment = PSeg::new(Vect::of(current_vertex), Vect::of(adjacent_vertex));

            // result from intersection segment loop
            let mut intersection_result_option = None;

            // iterator over all segments to find segment which intersects the current_adjacent_segment with smallest left angle to current_last_vector
            for indexed_segment in self.segments() {

                // skip segment if one of it's indecis is adjacent_index
                if indexed_segment.contains_index(adjacent_index) { 
                    continue;
                }

                let skip = match step.origin {
                    // skip segment if origin is mesh and one of it's indecis is current_index
                    ContourStepOrigin::Mesh{current_index} => indexed_segment.contains_index(current_index),

                    // skip segment if origin is intersection and it's equivalent to last segment that intersected step.current_vertex or if one of it's indecis is right_index
                    ContourStepOrigin::Intersection { last_left_index, right_index, ..} => {
                        indexed_segment.contains_index(right_index) || 
                        indexed_segment.contains_index(step.last_right_index) && indexed_segment.contains_index(last_left_index)
                    },
                };

                if skip {
                    continue;
                }

                // point segment defined by indexed segment
                let point_segment = self.point_segment(indexed_segment);

                // check if segment between current vertex and adjacent result vertex intersects the segment
                if let Some(intersection) = current_adjacent_segment.intsec(&point_segment) {
                    index += 1;

                    // vector from current vertex to intersection vertex
                    let current_intersection_vector = intersection.sub(current_vertex);

                    // distance from current vertex to intersection vertex
                    let current_intersection_length = current_intersection_vector.len();

                    // vector from adjacent vertex to point_segment vertex a
                    let adjacent_a_vector = point_segment.a().sub(adjacent_vertex);

                    // vector from adjacent vertex to point_segment vertex b
                    let adjacent_b_vector = point_segment.b().sub(adjacent_vertex);

                    // left angle between vector from adjacent vertex to current vertex and adjacent vertex to point segment vertex a 
                    let left_a_angle = adjacent_current_vector.angle_l(&adjacent_a_vector);

                    // left angle between vector from adjacent vertex to current vertex and adjacent vertex to point segment vertex b
                    let left_b_angle = adjacent_current_vector.angle_l(&adjacent_b_vector);

                    // sort vertices by left / right of current_adjacent_segment
                    let (left_index, left_vertex, right_index, right_vertex) = match left_a_angle <= left_b_angle {
                        false => (indexed_segment.b(), point_segment.b(), indexed_segment.a(), point_segment.a()),
                        true  => (indexed_segment.a(), point_segment.a(), indexed_segment.b(), point_segment.b())
                    };

                    let adjacent_left_vector = left_vertex.sub(adjacent_vertex);

                    let is_left = adjacent_current_vector.left(adjacent_left_vector);

                    match step.origin {
                        ContourStepOrigin::Mesh { current_index } => {
                            println!("last mesh last = {}; curr = {}; adj = {}; seg_a = {}; seg_b = {}; a_ang = {:?}; b_ang = {:?}; is_left = {};", step.last_right_index, current_index, adjacent_index, indexed_segment.a(),indexed_segment.b(), left_a_angle, left_b_angle, is_left)
                        },
                        ContourStepOrigin::Intersection { last_left_index, right_index, left_index } => {
                            println!("curr ints l'r = {}; l'l = {}; r = {}; l = {}; adj = {}; seg_a = {}; seg_b = {}; a_ang = {:?}; b_ang = {:?}; is_left = {};", step.last_right_index, last_left_index, right_index, left_index, adjacent_index, indexed_segment.a(),indexed_segment.b(), left_a_angle, left_b_angle, is_left)
                        },
                    }

                    let cmd = format!("S{}L = {}", index, vs(Vect::of(self.vertex(left_index))));
                    commands.push(cmd);

                    let cmd = format!("S{}R = {}", index, vs(Vect::of(self.vertex(right_index))));
                    commands.push(cmd);

                    let cmd = format!("S{}I = {}", index, vs(Vect::of(&intersection)));
                    commands.push(cmd);

                    let cmd = format!("S{} = Segment(S{}L, S{}R)", index, index, index);
                    commands.push(cmd);

                    if !is_left {
                        continue;
                    }

                    // create intersection result
                    let result = ContourIntersectionResult {
                        intersection : intersection,
                        distance     : current_intersection_length,
                        left_vertex  : left_vertex,
                        left_index   : left_index,
                        right_vertex : right_vertex,
                        right_index  : right_index
                    };

                    intersection_result_option = match intersection_result_option {
                        None => Some(result),
                        Some(best_result) => 
                            Some(ContourIntersectionResult::choose(result, best_result, current_adjacent_segment.ab()))
                    };
                }
            }
            
            // rlast = rcurrent;

            // update loop variables according to found results
            if let Some(intersection_result) = intersection_result_option {
                println!("contour.len = {:?}; intersection;", contour.len() + 1);
                println!("intersection = {}", vs(Vect::of(&intersection_result.intersection)));
                commands.iter_mut().for_each(|cmd| *cmd = format!("\"{}\"", cmd));
                let str = commands.join(", ");
                println!("Execute[{{{}}}]\n\n", str);


                let last_right_index = match step.origin {
                    ContourStepOrigin::Mesh{current_index} => current_index,
                    ContourStepOrigin::Intersection{right_index, ..} => right_index,
                };

                step = ContourStep {
                    last_right_index : last_right_index,
                    last_vertex      : step.current_vertex,
                    current_vertex   : intersection_result.intersection,
                    origin: ContourStepOrigin::Intersection {
                        last_left_index : adjacent_index,
                        right_index     : intersection_result.right_index,
                        left_index      : intersection_result.left_index,
                    }
                }
            }
            else {
                println!("contour.len = {:?}; adjacent;", contour.len() + 1);
                println!("adjacent = {}", vs(Vect::of(&adjacent_vertex)));
                commands.iter_mut().for_each(|cmd| *cmd = format!("\"{}\"", cmd));
                let str = commands.join(", ");
                println!("Execute[{{{}}}]\n\n", str);

                let last_right_index = match step.origin {
                    ContourStepOrigin::Mesh{current_index} => current_index,
                    ContourStepOrigin::Intersection{right_index, ..} => right_index,
                };

                step = ContourStep {
                    last_right_index : last_right_index,
                    last_vertex      : step.current_vertex,
                    current_vertex   : Vect::of(self.vertex(adjacent_index)),
                    origin: ContourStepOrigin::Mesh{current_index : adjacent_index}
                }
            }

            if contour[0].equal(&step.current_vertex) {
                break;
            }

            if contour.len() >= max {
                break;
            }

            contour.push(Vect::of(&step.current_vertex))
        }

        Some(contour)
    }

    #[warn(deprecated)]
    pub fn deduplicate(&self) -> IndSegMesh<Vect::Own> 
    where Vect::Val : HPEq
    {
        let mut vertices = Vec::new();
        let mut segments = Vec::new();

        let mut maybe_push_vertex_and_get_index = |vertex : Vect::Own| {
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

        for indexed_segment in self.segments() {
            let a = Vect::of(self.vertex(indexed_segment.a()));
            let b = Vect::of(self.vertex(indexed_segment.b()));

            let index_a = maybe_push_vertex_and_get_index(a);
            let index_b = maybe_push_vertex_and_get_index(b);

            let segment = IndSeg::new(index_a, index_b);

            maybe_push_segment(segment);
        }

        IndSegMesh::new_unchecked(vertices, segments)
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