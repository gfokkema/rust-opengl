#![feature(convert)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]
#[macro_use]

extern crate glium;
extern crate regex;

mod mesh;

use std::env;
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

fn main() {
	let args: Vec<_> = env::args().skip(1).collect();
	let path = match args.as_slice() {
		[ref e] => e,
		[]      => { println!("No mesh specified"); return },
		_       => { println!("Too many arguments specified"); return },
	};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
	let data = mesh::open(path);
	println!("{:?}", data);

    let vertex_buffer = glium::VertexBuffer::new(&display, data);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

		uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let mut t = -0.5;

    loop {
    	t += 0.002;
    	if t > 0.5 {
    		t = -0.5;
    	}

    	let mut target = display.draw();
    	target.clear_color(0.0, 0.0, 0.0, 1.0);

		let uniforms = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			]
		};

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
    	target.finish().unwrap();

	    // listing the events produced by the window and waiting to be received
	    for ev in display.poll_events() {
	        match ev {
	            Event::Closed => return,   // the window has been closed by the user
	            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
	            _ => ()
	        }
	    }
	}
}