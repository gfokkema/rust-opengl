use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    barycentric: [f32; 3],
}
implement_vertex!(Vertex, position, barycentric);

#[derive(Debug)]
pub struct Mesh {
    triangles: Vec<[usize; 3]>,
    vertices:  Vec<[f32; 3]>,
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
        for line in obj.lines().collect::<Vec<&str>>() {
            mesh.parse_line(line.trim());
        }
        mesh
    }
    
    fn parse_line(&mut self, line: &str) {
        match line.split(&WHITESPACE).collect::<Vec<&str>>().as_slice() {
            ["v", rest ..] => self.vertices.push(parse_vertex(rest).unwrap()),
            ["f", rest ..] => self.triangles.push(parse_face(rest).unwrap()),
            _              => (),
        }
    }
    
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        self.triangles.iter().flat_map(|x| {
            let mut vertex = Vec::new();
            for vi in 0 .. x.len() {
                let mut barycentric = [ 0.0, 0.0, 0.0 ];
                barycentric[vi] = 1.0;
                vertex.push(Vertex {
                    position: self.vertices[x[vi] - 1],
                    barycentric: barycentric,
                });
            }
            vertex
        })
        .collect::<Vec<_>>()
    }
}

impl FromStr for Mesh {
    type Err = &'static str;
    
    fn from_str(data: &str) -> Result<Mesh, Self::Err> {
        Ok(Mesh::from_str(data))
    }
}

fn parse_vertex(vertex: &[&str]) -> Result<[f32; 3], &'static str> {
    match vertex {
        [v0, v1, v2, ..] =>
            Ok( [ v0.parse::<f32>().unwrap(),
                  v1.parse::<f32>().unwrap(),
                  v2.parse::<f32>().unwrap(), ], ),
        _                => Err("Incorrect input"),
    }
}

fn parse_face(face: &[&str]) -> Result<[usize; 3], &'static str> {
    match face {
        [v0, v1, v2, ..] =>
            Ok( [ v0.split("/").next().unwrap().parse::<usize>().unwrap(),
                  v1.split("/").next().unwrap().parse::<usize>().unwrap(),
                  v2.split("/").next().unwrap().parse::<usize>().unwrap(), ], ),
        _                => Err("Incorrect input"),
    }
}
