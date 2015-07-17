#![feature(convert)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]
#[macro_use]

extern crate glium;
extern crate regex;

mod mesh;

use std::env;
use std::fs::File;
use std::io::Read;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pub position: [f32; 3],
    pub barycentric: [f32; 3],
}
implement_vertex!(Vertex, position, barycentric);

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let path = match args.as_slice() {
        [ref e] => e,
        []      => panic!("No mesh specified"),
        _       => panic!("Too many arguments specified"),
    };
    
    let mut file = match File::open(path) {
        Ok(e)   => e,
        Err(_)  => panic!("Invalid mesh specified"),
    };

    let mut obj = String::new();
    match file.read_to_string(&mut obj) {
        Ok(_)   => (),
        Err(_)  => panic!("Error while reading mesh"),
    }
    
    show(obj.parse::<mesh::Mesh>().unwrap());
}

fn show(obj: mesh::Mesh) {
    let vertex_shader_src = r#"
        #version 140
        
        attribute vec3 position;
        attribute vec3 barycentric;
        
        out vec3 uv;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
            
            uv = barycentric;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 uv;
        
        out vec4 color;

        void main() {
            color = vec4(uv, 1.0);
        }
    "#;
    
    let vertices = obj.triangles.iter()
        .flat_map(|x| {
            let mut vertex = Vec::new();
            for vi in 0..x.vertices.len() {
                let mut barycentric = [ 0.0, 0.0, 0.0 ];
                barycentric[vi] = 1.0;
                vertex.push(Vertex {
                    position: obj.vertices[x.vertices[vi] - 1].position,
                    barycentric: barycentric,
                });
            }
            vertex
        })
        .collect::<Vec<_>>();
    
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(800, 800)
                    .with_depth_buffer(24)
                    .build_glium().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, vertices);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
        depth_test: glium::DepthTest::IfLess,
        depth_write: true,
        .. Default::default()
    };
    
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 24.0);

        let uniforms = uniform! {
            matrix: [ [1.0, 0.0, 0.0, 0.0],
                      [0.0, 1.0, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.0, 0.0, 1.0, 2.0], ],
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,   // the window has been closed by the user
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}
