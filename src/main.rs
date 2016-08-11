#[macro_use] extern crate nom;

mod surface_points;

use surface_points::SurfacePoints;

fn main() {
    let test = b"   (10 ,11 ,12) (1,5,9)(2,3,5)";
    let surface_points = SurfacePoints::from_buffer(test);
    println!("{:?}", surface_points);
}
