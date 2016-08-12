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
                file.read_to_end(&mut vec);
                match SurfacePoints::from_buffer(&vec) {
                    Ok(surface_points) => surfaces_points.push(surface_points),
                    Err(err) => printlnc!(red: "{}: Syntax error", arg)
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
    let height_map = HeightMap::from_surface_points(&surfaces_points[0]);
    // println!("{:?}", surfaces_points);
}
//
// float           get_weight(float dist)
// {
//     return ((dist == 0) ? (1.f) : (1.f / (dist * dist)));
// }
//
// __kernel void   adjust_map( __global    float   *map_heights,
//                                         uint2   map_size,
//                             __global    float3  *control_points,
//                                         uint    nbr_control_points)
// {
//     size_t      x;
//     size_t      y;
//     float2      pos;
//     float       sum_numerator;
//     float       sum_denominator;
//
//     // MOVING MAP !!! ????
//     x = get_global_id(0);
//     y = get_global_id(1);
//     pos.x = x;
//     pos.y = y;
//     sum_numerator = 0.f;
//     sum_denominator = 0.f;
//     for (uint i = 0; i < nbr_control_points; ++i)
//     {
//         float       dist;
//         float       weight;
//
//         dist = distance(control_points[i].xy, pos);
//         weight = get_weight(dist);
//         sum_numerator += weight * control_points[i].z;
//         sum_denominator += weight;
//     }
//
//     //*
//     float       edge_dist;
//     float       edge_weight;
//
//     edge_dist = distance(0.f, pos.x);
//     edge_weight = get_weight(edge_dist);
//     sum_denominator += edge_weight;
//
//     edge_dist = distance(map_size.x - 1, pos.x);
//     edge_weight = get_weight(edge_dist);
//     sum_denominator += edge_weight;
//
//     edge_dist = distance(map_size.y - 1, pos.y);
//     edge_weight = get_weight(edge_dist);
//     sum_denominator += edge_weight;
//
//     edge_dist = distance(0.f, pos.y);
//     edge_weight = get_weight(edge_dist);
//     sum_denominator += edge_weight;
//     //*/
//
//     map_heights[(y * map_size.x) + x] = sum_numerator / sum_denominator;
// }
