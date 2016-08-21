#[macro_use] extern crate nom;
#[macro_use] extern crate colorify;
extern crate ndarray;

mod surface_points;
mod height_map;

use std::env;
use std::fs::File;
use std::io::Read;
use surface_points::SurfacePoints;
use height_map::HeightMap;

fn get_surfaces_points() -> Vec<SurfacePoints> {
    let mut surfaces_points = Vec::new();
    let args = env::args().skip(1);
    for arg in args {
        match File::open(arg.clone()) {
            Ok(mut file) => {
                let mut vec = Vec::new();
                if let Err(err) = file.read_to_end(&mut vec) {
                    printlnc!(red: "{:?}", err);
                    return surfaces_points;
                }
                match SurfacePoints::from_buffer(&vec) {
                    Ok(surface_points) => surfaces_points.push(surface_points),
                    Err(err) => printlnc!(red: "{}: {}", arg, err)
                }
            },
            Err(err) => printlnc!(red: "{}: {}", arg, err)
        }
    }
    surfaces_points
}

fn main() {
    let surfaces_points = get_surfaces_points();
    // TODO manage all surfaces points
    for surface_points in surfaces_points {
        let height_map = HeightMap::from_surface_points(&surface_points);
        println!("{:?}", surface_points);
        println!("{:?}", height_map);
    }
}
