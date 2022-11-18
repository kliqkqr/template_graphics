use tg::file::stl::Stl;

use tg::prim::d3::Vect;

fn vect_to_string(vect : Vect<f32>) -> String {
    format!("({:.5}, {:.5}, {:.5})", vect.0, vect.1, vect.2)
}

fn main() {
    let stl_path = r#"C:\OneDrive\Code\Bachelor\models\stl\LINK SP-CL 3D\177-200_26.stl"#;
    let stl = match Stl::read_binary(stl_path) {
        Ok(stl) => stl,
        Err(err) => {
            println!("{}", err.to_string());
            return;
        }
    };

    println!("head: {}", stl.head().iter().map(|byte| *byte as char).collect::<String>());
    println!("size: {}", stl.triangles().len());

    for (i, t) in stl.triangles().iter().enumerate() {
        let normal= t.normal();
        let vertices = t.vertices();
        let attr= t.attr_byte_count();

        println!("");
        println!("indx: {}", i);
        println!("norm: {}", vect_to_string(normal));
        println!("v[0]: {}", vect_to_string(vertices.0));
        println!("v[1]: {}", vect_to_string(vertices.1));
        println!("v[2]: {}", vect_to_string(vertices.2));
        println!("attr: {}", attr);
    }
}
