use svg;

use crate::geom::d2::prim::seg::{
    Segment
};

use crate::geom::d2::prim::vect::{
    Vector 
};

pub fn draw_segment_to_path<Seg : Segment>(segment : Seg) -> svg::node::element::Path 
where Seg::Val : Into<svg::node::element::path::Parameters>
{
    let a = segment.a();
    let b = segment.b();

    let data = svg::node::element::path::Data::new()
        .move_to((a.x(), a.y()))
        .line_to((b.x(), b.y()));

    svg::node::element::Path::new()
        .set("d", data)
}

pub fn draw_segments_to_path<Iter : IntoIterator>(segments : Iter) -> svg::node::element::Path
where Iter::Item                   : Segment,
      <Iter::Item as Segment>::Val : Into<svg::node::element::path::Parameters>
{   
    let mut data = svg::node::element::path::Data::new();

    for segment in segments {
        let a = segment.a();
        let b = segment.b();

        data = data.move_to((a.x(), a.y()));
        data = data.line_to((b.x(), b.y()));
    }

    let path = svg::node::element::Path::new()
        .set("d", data);

    path
}

pub fn draw_segments_to_paths<Iter : IntoIterator>(segments : Iter) -> Vec<svg::node::element::Path> 
where Iter::Item                   : Segment,
      <Iter::Item as Segment>::Val : Into<svg::node::element::path::Parameters>
{
    let mut paths = Vec::new();

    for segment in segments {
        let a = segment.a();
        let b = segment.b();

        let data = svg::node::element::path::Data::new()
            .move_to((a.x(), a.y()))
            .line_to((b.x(), b.y()));

        let path = svg::node::element::Path::new()
            .set("d", data);

        paths.push(path);
    }

    paths
}