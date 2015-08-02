use cgmath::{Matrix3, Matrix4};

use glium::{BackfaceCullingMode, DepthTest, Display, DisplayBuild,
            DrawParameters, Program, Surface, VertexBuffer};
use glium::glutin::Event::{KeyboardInput, MouseMoved};
use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowBuilder};
use glium::index::{IndicesSource, NoIndices, PrimitiveType};

use camera::Direction;
use scene;

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

pub struct Context<'a> {
    pub display: Display,
        params:  DrawParameters<'a>,
    pub program: Program,
}

impl <'a> Context<'a> {
    pub fn new(size: (i32, i32)) -> Self {
        let display = WindowBuilder::new()
            .with_dimensions(size.0 as u32, size.1 as u32)
            .with_depth_buffer(24)
            .build_glium().unwrap();
        let program = Program::from_source(
            &display, vertex_shader_src, fragment_shader_src, None
        ).unwrap();
        let params = DrawParameters {
            // FIXME: Something is wrong in code or in cubes.obj
            backface_culling:     BackfaceCullingMode::CullingDisabled,
            depth_test:           DepthTest::IfLess,
            depth_write:          true,
            .. Default::default()
        };
        
        Context {
            display: display,
            params:  params,
            program: program,
        }
    }
    
    pub fn draw(&self, scene: &scene::Scene) {
        let vbo = VertexBuffer::new(&self.display, scene.get_vertex_array());
        let indices = IndicesSource::NoIndices {
            primitives: PrimitiveType::TrianglesList
        };
        let model: Matrix4<f32> = Matrix3::from_value(1.0).into();
        let uniforms = uniform! {
            mvp: scene.camera.project * scene.camera.view * model,
        };
        
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 24.0);
        target.draw(&vbo, indices, &self.program, &uniforms, &self.params).unwrap();
        target.finish().unwrap();
    }
    
    pub fn handle_input(&self, scene: &mut scene::Scene, ev: Event) -> bool {
        match ev {
            Event::Closed                               => false,
            Event::KeyboardInput(ElementState::Pressed,
                                 _, Some(e))            => { self.handle_keyboard(scene, e) },
            Event::MouseMoved(e)                        => { self.handle_mouse(scene, e); true },
            _                                           => true,
        }
    }
    
    fn handle_keyboard(&self, scene: &mut scene::Scene, e: VirtualKeyCode) -> bool {
        match e {
            VirtualKeyCode::Escape => false,
            VirtualKeyCode::W      => { scene.camera.move_dir(Direction::Up,    1.0); true },
            VirtualKeyCode::A      => { scene.camera.move_dir(Direction::Left,  1.0); true },
            VirtualKeyCode::S      => { scene.camera.move_dir(Direction::Down,  1.0); true },
            VirtualKeyCode::D      => { scene.camera.move_dir(Direction::Right, 1.0); true },
            _                      => true,
        }
    }
    
    fn handle_mouse(&self, scene: &mut scene::Scene, e: (i32, i32)) {
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