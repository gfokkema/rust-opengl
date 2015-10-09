use cgmath::{FixedArray, Matrix3, Matrix4};
use glium::{Depth, DepthTest, Display, DisplayBuild,
            DrawParameters, Program, Surface, VertexBuffer};
use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowBuilder};
use glium::glutin::Event::{KeyboardInput, MouseMoved};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::Texture2dArray;

use camera;
use camera::Direction;
use image;
use mesh::Mesh;
use obj;
use std::rc::Rc;
use std::path::Path;

const VERTEX_SHADER_SRC: &'static str = r#"
  #version 140
  
  attribute vec3 v_position;
  attribute vec3 v_normal;
  attribute vec2 v_tex_coords;
  attribute uint v_tex_id;

       out vec2 tex_coords;
  flat out uint tex_id;
  
  uniform mat4 mvp;
  
  void main() {
    tex_id = v_tex_id;
    tex_coords = v_tex_coords;
    gl_Position = mvp * vec4(v_position, 1.0);
  }
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
  #version 140

       in vec2 tex_coords;
  flat in uint tex_id;

  out vec4 color;
  
  uniform sampler2DArray tex;
	
  void main() {
    color = texture(tex, vec3(tex_coords, tex_id));
  }
"#;

pub struct Context<'a> {
  pub display: Display,
    params:  DrawParameters<'a>,
    program: Program,
    textures: Texture2dArray,
}

impl <'a> Context<'a> {
  pub fn new(size: (i32, i32)) -> Self {
    let display = WindowBuilder::new()
                  .with_dimensions(size.0 as u32, size.1 as u32)
                  .with_depth_buffer(24)
                  .build_glium().unwrap();
    let program = program!(&display, 140 => {
                    vertex:   VERTEX_SHADER_SRC,
                    fragment: FRAGMENT_SHADER_SRC
                  }).unwrap();
    let params = DrawParameters {
      depth: Depth {
        test:  DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      .. Default::default()
    };
    
    
    let images = vec![ image::open(Path::new("./debug_texture.jpg")).unwrap(),
                       image::open(Path::new("./debug_texture.jpg")).unwrap(), ];
    let textures = Texture2dArray::new(&display, images).unwrap();
    
    Context {
      display:  display,
      params:   params,
      program:  program,
      textures: textures,
    }
  }
  
  pub fn draw(&self, camera: &camera::Camera, mesh: &obj::Obj<Rc<obj::Material>>) {
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let model: Matrix4<f32> = Matrix3::from_value(1.0).into();
    let uniforms = uniform! {
      mvp:     *(camera.project * camera.view * model).as_fixed(),
    };
    
    let mut target = self.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    let vertices = VertexBuffer::new(&self.display, &mesh.vertex_buffer()).unwrap();
    target.draw(&vertices, &indices, &self.program, &uniforms, &self.params).unwrap();
    target.finish().unwrap();
  }
  
  pub fn handle_input(&self, camera: &mut camera::Camera, ev: Event) -> bool {
    match ev {
      Event::Closed                 => false,
      Event::KeyboardInput(ElementState::Pressed, _, Some(e))
                                    => { self.handle_keyboard(camera, e) },
      Event::MouseMoved(e)          => { self.handle_mouse(e); true },
      _                             => true,
    }
  }
  
  fn handle_keyboard(&self, camera: &mut camera::Camera, e: VirtualKeyCode) -> bool {
    match e {
      VirtualKeyCode::Escape => false,
      VirtualKeyCode::W    => { camera.move_dir(Direction::Up,    1.0); true },
      VirtualKeyCode::A    => { camera.move_dir(Direction::Left,  1.0); true },
      VirtualKeyCode::S    => { camera.move_dir(Direction::Down,  1.0); true },
      VirtualKeyCode::D    => { camera.move_dir(Direction::Right, 1.0); true },
      _                    => true,
    }
  }
  
  fn handle_mouse(&self, e: (i32, i32)) {
    let dim = get_display_dim(&self.display);
    let center = (dim.0 / 2, dim.1 / 2);
    
    match (e.0 - center.0, e.1 - center.1) {
      (0, 0) => (),
      (x, y) => { set_cursor_pos(&self.display, center);
            println!("{} {}", x, y) },
    }
  }
}

pub fn get_display_dim(display: &Display) -> (i32, i32) {
  match display.get_window().unwrap().get_inner_size() {
    Some(dim) => (dim.0 as i32, dim.1 as i32),
    None => panic!("Couldn't get window dimensions")
  }
}

pub fn set_cursor_pos(display: &Display, pos: (i32, i32)) {
  display.get_window().unwrap().set_cursor_position(pos.0, pos.1).unwrap();
}
