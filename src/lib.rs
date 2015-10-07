#![feature(core_intrinsics)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate genmesh;
extern crate image;
extern crate obj;
extern crate time;

pub mod camera;
pub mod context;
pub mod mesh;

pub fn print_type_of<T>(_: &T) -> () {
  let type_name =
    unsafe {
      std::intrinsics::type_name::<T>()
    };
  println!("{}", type_name);
}