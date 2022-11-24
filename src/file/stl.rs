use std::{
    fs,
    io
};

use std::convert::{
    AsRef
};

use std::path::{
    Path
};

use std::vec::{
    Vec
};

use byteorder::{
    LittleEndian, ReadBytesExt
};

use crate::geom::d3::prim::{
    Vect
};

pub struct Tri {
    normal          : Vect<f32>,
    vertices        : (Vect<f32>, Vect<f32>, Vect<f32>),
    attr_byte_count : u16 
}

pub struct Stl {
    head      : Vec<u8>,
    triangles : Vec<Tri>
}

impl Tri {
    pub fn new(normal : Vect<f32>, vertices : (Vect<f32>, Vect<f32>, Vect<f32>), attr_byte_count : u16) -> Tri {
        Tri{normal : normal, vertices : vertices, attr_byte_count : attr_byte_count}
    }

    pub fn normal(&self) -> Vect<f32> {
        self.normal.clone()
    }

    pub fn vertices(&self) -> (Vect<f32>, Vect<f32>, Vect<f32>) {
        self.vertices.clone()
    }

    pub fn attr_byte_count(&self) -> u16 {
        self.attr_byte_count
    }
}

impl Stl {
    fn new(head : Vec<u8>, triangles : Vec<Tri>) -> Stl {
        Stl{head : head, triangles : triangles}
    }

    fn read_vect_from_bytes(bytes : &[u8]) -> io::Result<Vect<f32>> {
        if bytes.len() <= 12 {
            let err = io::Error::new(io::ErrorKind::InvalidData, "tg.file.stl.Stl.read_vect_from_bytes: invalid bytes length");
            return Err(err)
        }

        let x = bytes[0..].as_ref().read_f32::<LittleEndian>()?;
        let y = bytes[4..].as_ref().read_f32::<LittleEndian>()?;
        let z = bytes[8..].as_ref().read_f32::<LittleEndian>()?;

        Ok(Vect::new(x, y, z))
    }

    pub fn read_binary<A : AsRef<Path>>(path : A) -> io::Result<Stl> {
        let bytes = fs::read(path)?;

        if bytes.len() <= 84 || (bytes.len() - 84) % 50 != 0 {
            let err = io::Error::new(io::ErrorKind::InvalidData, "tg.file.stl.Stl.read_binary: invalid file size");
            return Err(err)
        }

        let head_bytes = bytes[0..80].to_vec();
        let head_ascii = head_bytes[0..6]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();

        if head_ascii == "solid " {
            let err = io::Error::new(io::ErrorKind::InvalidData, "tg.file.stl.Stl.read_binary: ascii file");
            return Err(err)
        }

        let triangle_count = (bytes.len() - 84) / 50;
        let mut triangles = Vec::with_capacity(triangle_count);

        for i in 0..triangle_count {
            let start = 84 + i * 50;
            let bytes = &bytes[start..];

            let normal = Stl::read_vect_from_bytes(bytes)?;

            let vertices = (
                Stl::read_vect_from_bytes(bytes[12..].as_ref())?,
                Stl::read_vect_from_bytes(bytes[24..].as_ref())?,
                Stl::read_vect_from_bytes(bytes[36..].as_ref())?
            );

            let attr_byte_count = bytes[48..].as_ref().read_u16::<LittleEndian>()?;

            let triangle = Tri::new(normal, vertices, attr_byte_count);
            triangles.push(triangle);
        }

        let stl = Stl::new(head_bytes, triangles);
        Ok(stl)
    }

    pub fn head(&self) -> &Vec<u8> {
        &self.head
    }

    pub fn triangles(&self) -> &Vec<Tri> {
        &self.triangles
    }
}