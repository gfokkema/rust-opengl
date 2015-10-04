#![feature(convert)]
#![feature(slice_patterns)]

extern crate rust_opengl;

use rust_opengl::*;
use std::env;
use std::fs::File;

fn main() {
  let args: Vec<_> = env::args().skip(1).collect();
  let path = match args.as_slice() {
    [ref e] => e,
    []      => panic!("No mesh specified"),
    _       => panic!("Too many arguments specified"),
  };

  let file = match File::open(path) {
    Ok(e)   => e,
    Err(_)  => panic!("Invalid mesh specified"),
  };

  let size    = (800, 600);
  let mut camera  = camera::Camera::new(size, 60.0);
  let context = context::Context::new(size);
  let mesh    = mesh::load_mesh(file);
  loop {
    context.draw(&camera, &mesh);

    for ev in context.display.poll_events() {
      if !context.handle_input(&mut camera, ev) { return; }
    }
  }
}
