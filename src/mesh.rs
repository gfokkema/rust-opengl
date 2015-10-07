use genmesh::{Polygon, Triangle, Quad};
use obj;
use std::rc::Rc;

pub trait Indices {
  fn indices(&self) -> Vec<u16>;
}

impl Indices for obj::Obj<Rc<obj::Material>> {
  fn indices(&self) -> Vec<u16> {
    self.object_iter()
    .flat_map(|x| x.group_iter())
    .flat_map(|g| g.indices())
    .flat_map(|i|
      match i {
        &Polygon::PolyTri(Triangle {
          x: (a,_,_), y: (b,_,_), z: (c,_,_)
        }) =>
          vec![ a as u16, b as u16, c as u16 ],
        &Polygon::PolyQuad(Quad {
          x: (a,_,_), y: (b,_,_), z: (c,_,_), w: (d,_,_)
        }) =>
          vec![ a as u16, b as u16, c as u16,
                a as u16, b as u16, d as u16 ],
      })
    .collect::<Vec<_>>()
  }
}
