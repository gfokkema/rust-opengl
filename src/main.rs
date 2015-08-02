#![feature(convert)]
#![feature(core_intrinsics)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]

#[macro_use] extern crate glium;
extern crate cgmath;
extern crate regex;
extern crate time;

mod camera;
mod context;
mod mesh;
mod scene;

use std::env;
use std::fs::File;
use std::io::Read;

//fn print_type_of<T>(_: &T) -> () {
//    let type_name =
//        unsafe {
//            std::intrinsics::type_name::<T>()
//        };
//    println!("{}", type_name);
//}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let path = match args.as_slice() {
        [ref e] => e,
        []      => panic!("No mesh specified"),
        _       => panic!("Too many arguments specified"),
    };
    
    let mut file = match File::open(path) {
        Ok(e)   => e,
        Err(_)  => panic!("Invalid mesh specified"),
    };

    let mut obj = String::new();
    match file.read_to_string(&mut obj) {
        Ok(_)   => (),
        Err(_)  => panic!("Error while reading mesh"),
    }
    
    let context = context::Context::new();
    let mesh = obj.parse::<mesh::Mesh>().unwrap();
    let mut scene = scene::Scene::new(mesh);
    loop {
        context.show(&scene);
        
        for ev in context.display.poll_events() {
            if !context.handle_input(&mut scene, ev) { return; }
        }
    }
}
