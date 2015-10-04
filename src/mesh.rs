use genmesh::EmitTriangles;
use obj::Obj;
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  position: [f32; 3],
  normal:   [f32; 3],
  texture:  [f32; 2],
}
implement_vertex!(Vertex, position, normal, texture);

pub fn load_mesh(file: File) -> Vec<Vertex> {
  let data = Obj::load(&mut BufReader::new(file));
  let mut vertex_data = Vec::new();

  for object in data.object_iter() {
    for shape in object.group_iter().flat_map(|g| g.indices().iter()) {
      shape.emit_triangles(|t| {
        for v in [t.x, t.y, t.z].iter() {
          let position = data.position()[v.0];
          let texture = v.1.map(|index| data.texture()[index]);
          let normal = v.2.map(|index| data.normal()[index]);

          let texture = texture.unwrap_or([0.0, 0.0]);
          let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

          vertex_data.push(Vertex {
            position: position,
            normal:   normal,
            texture:  texture,
          })
        }
      })
    }
  }
  vertex_data
}
