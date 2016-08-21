use ndarray::{Array, Ix};
use ::surface_points::SurfacePoints;

#[derive(Debug)]
pub struct HeightMap(Array<f32, (Ix, Ix)>); // FIXME i32 for height ???

#[inline]
fn weight(dist: f32) -> f32 {
    if dist == 0.0 {
        1.0
    } else {
        1.0 / (dist * dist)
    }
}

#[inline]
fn distance((a_x, a_y): (f32, f32), (b_x, b_y): (f32, f32)) -> f32 {
    let dist_x = a_x - b_x;
    let dist_y = a_y - b_y;
    let dist = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
    println!("({:?}, {:?}): dist: {}", (a_x, a_y), (b_x, b_y), dist);
    dist
}

fn inverse_distance_weighting(points: &SurfacePoints, array: &mut Array<f32, (Ix, Ix)>) {
    let array_rows = array.cols() as f32;
    let array_cols = array.rows() as f32;
    for (idx, height) in array.indexed_iter_mut() {
        let idx = (idx.0 as f32, idx.1 as f32);
        let (sum_numerator, mut sum_denominator) = points.iter().fold((0.0, 0.0), |acc, &p| {
            let dist = distance((p.x as f32, p.y as f32), idx);
            let weight = weight(dist);
            println!("({:?}), weight: {}", p.z, weight);
            (acc.0 + if dist != 0.0 { weight * p.z as f32 } else { p.z as f32 }
             , acc.1 + weight)
        });
        // sum_denominator += weight(idx.0);                            // dist (x, 0)
        // sum_denominator += weight(idx.1);                            // dist (0, y)
        // sum_denominator += weight((array_rows - 1.0 - idx.0).abs()); // dist (x_max, 0)
        // sum_denominator += weight((array_cols - 1.0 - idx.1).abs()); // dist (0, y_max)
        *height = sum_numerator / sum_denominator;
        println!("{:?}/{:?}, height: {:?}", sum_numerator, sum_denominator, *height);
        println!("-------------");
    }
}

impl HeightMap {
    pub fn from_surface_points(sp: &SurfacePoints) -> HeightMap {
        let mut array = Array::from_elem((4, 8), 0.0);
        inverse_distance_weighting(sp, &mut array);
        HeightMap(array)
    }
}

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
