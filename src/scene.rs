use camera;
use mesh;

pub struct Scene {
    pub camera:  camera::Camera,
        mesh:    mesh::Mesh,
}

impl Scene {
    pub fn new(camera: camera::Camera, mesh: mesh::Mesh) -> Self {
        Scene {
            camera: camera,
            mesh:   mesh,
        }
    }
    
    pub fn get_vertex_array(&self) -> Vec<mesh::Vertex> {
        self.mesh.get_vertex_array()
    }
}
