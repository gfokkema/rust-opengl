#![feature(convert)]
#![feature(core_intrinsics)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]

#[macro_use] extern crate glium;
extern crate cgmath;
extern crate regex;

mod mesh;

use std::env;
use std::fs::File;
use std::io::Read;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

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
        
        uniform mat4 mvp;
        
        void main() {
            gl_Position = mvp * vec4(position, 1.0);
            
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
                    .with_dimensions(800, 600)
                    .with_depth_buffer(24)
                    .build_glium().unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, vertices);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let params = glium::DrawParameters {
        // FIXME: Something is wrong in code or in cubes.obj
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        depth_test: glium::DepthTest::IfLess,
        depth_write: true,
        .. Default::default()
    };
    
    let project = cgmath::perspective::<f32, _>(cgmath::deg(90.0), 800.0 / 600.0, 0.1, 100.0);
    
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 24.0);

        let view:  cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(&cgmath::vec3(0.0, 0.0, -4.0));
        let model: cgmath::Matrix4<f32> = cgmath::Matrix3::from_value(2.0).into();
        let uniforms = uniform! {
            mvp: project * view * model,
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
