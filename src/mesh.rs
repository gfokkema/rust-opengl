extern crate regex;

use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn split(x: &str) -> Option<Vertex> {
    let re = regex!(r"[ \t]+");
    match re.split(x).collect::<Vec<&str>>().as_slice() {
        ["v", v0, v1, v2] => Some(Vertex { position: [v0.parse::<f32>().unwrap(),
                                                      v1.parse::<f32>().unwrap(),
                                                      v2.parse::<f32>().unwrap()] }),
//        ["f", v0, v1, v2] => triangles.push([v0.parse::<u32>().unwrap(),
//                                             v1.parse::<u32>().unwrap(),
//                                             v2.parse::<u32>().unwrap()]),
        _                 => None,
    }
}

pub fn open(path: &str) -> Vec<Vertex> {
    let mut file = match File::open(path) {
        Ok(e)  => e,
        Err(_) => panic!("Invalid mesh specified"),
    };

    let mut mesh = String::new();
    match file.read_to_string(&mut mesh) {
        Ok(_)  => (),
        Err(_) => panic!("Error while reading mesh"),
    }

    mesh.lines_any().filter_map(split).collect()
}
