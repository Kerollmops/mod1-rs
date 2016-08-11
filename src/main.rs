#[macro_use] extern crate nom;
#[macro_use] extern crate colorify;

mod surface_points;

use std::env;
use std::fs::File;
use std::io::Read;
use surface_points::SurfacePoints;

fn main() {
    let mut surfaces_points = Vec::new();
    let args = env::args().skip(1);
    for arg in args {
        match File::open(arg.clone()) {
            Ok(mut file) => {
                let mut vec = Vec::new();
                file.read_to_end(&mut vec);
                match SurfacePoints::from_buffer(&vec) {
                    Ok(surface_points) => surfaces_points.push(surface_points),
                    Err(err) => printlnc!(red: "{}: Syntax error", arg)
                }
            },
            Err(err) => printlnc!(red: "{}: {}", arg, err)
        }
    }
    println!("{:?}", surfaces_points);
}
