use cgmath::{Matrix3, Matrix4};

use glium::{BackfaceCullingMode, DepthTest, Display, DisplayBuild,
            DrawParameters, Program, Surface, VertexBuffer};
use glium::glutin::Event::{KeyboardInput, MouseMoved};
use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowBuilder};
use glium::index::{IndicesSource, NoIndices, PrimitiveType};

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
    pub fn new() -> Self {
        let display = WindowBuilder::new()
            .with_dimensions(800, 600)
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
    
    pub fn show(&self, scene: &scene::Scene) {
        let vbo = VertexBuffer::new(&self.display, scene.mesh.vertices());
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
            Event::MouseMoved(e)                        => { self.handle_mouse(scene, (e.0 - 800 / 2, e.1 - 600 / 2)) },
            _                                           => true,
        }
    }
    
    fn handle_keyboard(&self, scene: &mut scene::Scene, e: VirtualKeyCode) -> bool {
        match e {
            VirtualKeyCode::Escape => false,
            VirtualKeyCode::W      => { scene.camera.forward(1.0); true },
            VirtualKeyCode::A      => { scene.camera.forward(1.0); true },
            VirtualKeyCode::S      => { scene.camera.forward(-1.0); true },
            VirtualKeyCode::D      => { scene.camera.forward(-1.0); true },
            _                      => true,
        }
    }
    
    fn handle_mouse(&self, scene: &mut scene::Scene, e: (i32, i32)) -> bool {
//        self.display.get_window().unwrap().set_cursor_position(800 / 2, 600 / 2).unwrap();
        if e != (0, 0) {
            println!("{:?}", e);
        };
        true
    }
}