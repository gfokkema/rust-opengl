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

use glium::{DisplayBuild, DrawParameters, Program, Surface};
use glium::glutin::Event::KeyboardInput;
use glium::glutin::Event::MouseMoved;
use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowBuilder};
use glium::index::{IndicesSource, NoIndices, PrimitiveType};

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
    
    let window = Window::new();
    let mesh = obj.parse::<mesh::Mesh>().unwrap();
    loop {
        window.show(&mesh);
        
        for ev in window.display.poll_events() {
            window.handle_input(ev);
        }
    }
}

struct Window<'a> {
    size:    (i32, i32),
    camera:  camera::Camera,
    display: glium::backend::glutin_backend::GlutinFacade,
//    indices: IndicesSource<'a>,
    params:  DrawParameters<'a>,
    program: Program,
}

impl<'a> Window<'a> {
    fn new() -> Window<'a> {
        let size    = (800, 600);
        let camera  = camera::Camera::new(size, 90.0);
        let display = WindowBuilder::new()
                        .with_dimensions(size.0 as u32, size.1 as u32)
                        .with_depth_buffer(24)
                        .build_glium().unwrap();
        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let params  = glium::DrawParameters {
            // FIXME: Something is wrong in code or in cubes.obj
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,
            depth_test:       glium::DepthTest::IfLess,
            depth_write:      true,
            .. Default::default()
        };
        
        Window {
            size:    size,
            camera:  camera,
            display: display,
//            indices: indices,
            params:  params,
            program: program,
        }
    }
    
    fn show(&self, mesh: &mesh::Mesh) {
        let vbo = glium::VertexBuffer::new(&self.display, mesh.vertices());
        let indices = IndicesSource::NoIndices {
            primitives: PrimitiveType::TrianglesList
        };        
        let model: cgmath::Matrix4<f32> = cgmath::Matrix3::from_value(1.0).into();
        let uniforms = uniform! {
            mvp: self.camera.project * self.camera.view * model,
        };
        
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 24.0);
        target.draw(&vbo, indices, &self.program, &uniforms, &self.params).unwrap();
        target.finish().unwrap();
    }
    
    fn handle_input(&self, ev: Event) -> bool {
        match ev {
            Event::Closed                               => false,
            Event::KeyboardInput(ElementState::Pressed,
                                 _, Some(e))            => self.handle_keyboard(e),
            Event::MouseMoved(e)                        => self.handle_mouse((e.0 - self.size.0 / 2, e.1 - self.size.1 / 2)),
            _                                           => true,
        }
    }
    
    fn handle_keyboard(&self, e: VirtualKeyCode) -> bool {
        println!("{}", time::precise_time_ns());
        match e {
            VirtualKeyCode::Escape => false,
//            VirtualKeyCode::W      => { self.camera.forward(1.0); true },
//            VirtualKeyCode::S      => { self.camera.forward(-1.0); true },
            _                      => true,
        }
    }
    
    fn handle_mouse(&self, e: (i32, i32)) -> bool {
        self.display.get_window().unwrap().set_cursor_position(self.size.0 / 2, self.size.1 / 2).unwrap();
        if e != (0, 0) {
            println!("{:?}", e);
        };
        true
    }
}
