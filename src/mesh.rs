use genmesh::{Polygon, Triangle, Quad};
use obj;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct Vertex {
  v_position:   [f32; 3],
  v_normal:     [f32; 3],
  v_tex_coords: [f32; 2],
  v_tex_id:      u32,
}
implement_vertex!(Vertex, v_position, v_normal, v_tex_coords, v_tex_id);

pub trait Mesh {
  fn create_vertex(&self, index: &obj::IndexTuple) -> Vertex;
  fn vertex_buffer(&self) -> Vec<Vertex>;
}

impl Mesh for obj::Obj<Rc<obj::Material>> {
  fn create_vertex(&self, index: &obj::IndexTuple) -> Vertex {
    let texture = match index.1 {
      Some(n) => self.texture()[n],
      None    => [0.; 2],
    };
    let normal = match index.2 {
      Some(n) => self.normal()[n],
      None    => [0.; 3],
    };
    Vertex {
      v_position:   self.position()[index.0],
      v_normal:     normal,
      v_tex_coords: texture,
      v_tex_id:     0, // FIXME: hardcoded tex_id, only context knows indices
    }
  }
  
  fn vertex_buffer(&self) -> Vec<Vertex> {
    self.object_iter()
    .flat_map(|o| o.group_iter())
    .flat_map(|g| g.indices.iter())
    .flat_map(|i|
      match i {
        &Polygon::PolyTri(Triangle { x, y, z }) =>
          vec![x, y, z],
        &Polygon::PolyQuad(Quad { x, y, z, w }) =>
          vec![x, y, z, x, y, w],
      })
    .map(|i| self.create_vertex(&i))
    .collect::<Vec<_>>()
  }
}
