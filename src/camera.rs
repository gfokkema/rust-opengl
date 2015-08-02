use cgmath::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Camera {
    pub pos:   Vector3<f32>,
    pub up:    Vector3<f32>,
    pub dir:   Vector3<f32>,
    pub right: Vector3<f32>,
    pub project: Matrix4<f32>,
    pub view:    Matrix4<f32>,
}

impl Camera {
    pub fn new(size: (i32, i32), fov: f32) -> Camera {
        Camera {
            pos:   vec3(0.0, 0.0, 0.0),
            up:    vec3(0.0, 1.0, 0.0),
            dir:   vec3(0.0, 0.0, 1.0),
            right: vec3(1.0, 0.0, 0.0),
            project: perspective(deg(fov), size.0 as f32 / size.1 as f32, 0.1, 100.0),
            view:    Matrix4::identity(),
        }
    }
    
    pub fn move_dir(&mut self, dir: Direction, dt: f32) {
        match dir {
            Direction::Up    => self.pos = self.pos + self.dir.mul_s(dt),
            Direction::Down  => self.pos = self.pos - self.dir.mul_s(dt),
            Direction::Left  => self.pos = self.pos + self.right.mul_s(dt),
            Direction::Right => self.pos = self.pos - self.right.mul_s(dt),
        }
        self.view = Matrix4::from_translation(&self.pos);
    }
}