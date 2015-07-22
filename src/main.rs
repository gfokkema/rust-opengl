#![feature(convert)]
#![feature(core_intrinsics)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]

#[macro_use] extern crate glium;
extern crate cgmath;
extern crate regex;
extern crate time;

mod camera;
mod mesh;

use std::env;
use std::fs::File;
use std::io::Read;

use glium::{DisplayBuild, Surface};
use glium::glutin::Event::KeyboardInput;
use glium::glutin::Event::MouseMoved;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

const vertex_shader_src: &'static str = r#"
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

const fragment_shader_src: &'static str = r#"
    #version 140
	
    in vec3 uv;
    
    out vec4 color;
	
    void main() {
        color = vec4(uv, 1.0);
    }
"#;


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
    let vbo     = glium::VertexBuffer::new(&display, vertices);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let params  = glium::DrawParameters {
        // FIXME: Something is wrong in code or in cubes.obj
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        depth_test:       glium::DepthTest::IfLess,
        depth_write:      true,
        .. Default::default()
    };
    
    let dim = (800, 600);
    let center = (400, 300);
    
    let mut camera = camera::Camera::new(dim, 90.0);
    
    loop {
        let model: cgmath::Matrix4<f32> = cgmath::Matrix3::from_value(1.0).into();
        let uniforms = uniform! {
            mvp: camera.project * camera.view * model,
        };
        
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 24.0);
        target.draw(&vbo, &indices, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();
        
        display.get_window().unwrap().set_cursor_position(center.0, center.1).unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed                               => return,
                Event::KeyboardInput(ElementState::Pressed,
                                     _, Some(e))            => camera.forward(-1.0), // if !handle_keyboard(e) { return },
                Event::MouseMoved(e)                        => if !handle_mouse((e.0 - center.0, e.1 - center.1)) { return },
                _                                           => (),
            }
        }
    }
}

fn handle_keyboard(e: VirtualKeyCode) -> bool {
    println!("{}", time::precise_time_ns());
    match e {
        VirtualKeyCode::Escape => false,
        VirtualKeyCode::W      => { println!("W pressed"); true },
        _                      => true,
    }
}

fn handle_mouse(e: (i32, i32)) -> bool {
    if e != (0, 0) {
        println!("{:?}", e);
    };
    true
}
