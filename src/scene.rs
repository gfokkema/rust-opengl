use camera;
use mesh;

pub struct Scene {
    pub camera:  camera::Camera,
    pub mesh:    mesh::Mesh,    
}

impl Scene {
    pub fn new(mesh: mesh::Mesh) -> Self {
        let camera  = camera::Camera::new((800, 600), 90.0);
        
        Scene {
            camera: camera,
            mesh:   mesh,
        }
    }
}