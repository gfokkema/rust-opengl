extern crate regex;

use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

#[derive(Debug)]
pub struct Face {
    pub vertices: [usize; 3],
}

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Face>,
    pub vertices:  Vec<Vertex>,
}

static WHITESPACE: Regex = regex!(r"[ \t]+");
impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            triangles: Vec::new(),
            vertices: Vec::new()
        }
    }
    
    pub fn from_str(obj: &str) -> Mesh {
        let mut mesh = Mesh::new();
        for line in obj.lines_any().collect::<Vec<&str>>() {
            mesh.parse_line(line.trim());
        }
        mesh
    }
    
    fn parse_line(&mut self, line: &str) {
        match line.split(&WHITESPACE).collect::<Vec<&str>>().as_slice() {
            ["v", rest ..] => self.vertices.push(Mesh::parse_vertex(rest).unwrap()),
            ["f", rest ..] => self.triangles.push(Mesh::parse_face(rest).unwrap()),
            _              => (),
        }
    }
    
    fn parse_vertex(vertex: &[&str]) -> Result<Vertex, &'static str> {
        match vertex {
            [v0, v1, v2] => Ok(
                Vertex {
                    position: [ v0.parse::<f32>().unwrap(),
                                v1.parse::<f32>().unwrap(),
                                v2.parse::<f32>().unwrap(), ],
                }),
            _            => Err("Incorrect input"), 
        }
    }
    
    fn parse_face(face: &[&str]) -> Result<Face, &'static str> {
        match face {
            [v0, v1, v2, ..] => Ok(
                Face {
                    vertices: [ v0.split("/").next().unwrap().parse::<usize>().unwrap(),
                                v1.split("/").next().unwrap().parse::<usize>().unwrap(),
                                v2.split("/").next().unwrap().parse::<usize>().unwrap(), ],
                }),
            _            => Err("Incorrect input"),
        }
    }
}

impl FromStr for Mesh {
    type Err = &'static str;
    
    fn from_str(data: &str) -> Result<Mesh, Self::Err> {
        Ok(Mesh::from_str(data))
    }
}
