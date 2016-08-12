use ndarray::{ArrayBase, Array, Ix};
use ::surface_points::SurfacePoints;

pub struct HeightMap(Array<u32, (Ix, Ix)>); // FIXME i32 for height ???

#[inline]
fn weight(dist: f32) -> f32 {
    if dist == 0.0 {
        1.0
    } else {
        1.0 / (dist * dist)
    }
}

fn distance((a_x, a_y): (f32, f32), (b_x, b_y): (f32, f32)) -> f32 {
    (((a_x - b_x) * (a_x - b_x)) + ((a_y - b_y) * (a_y - b_y))).sqrt()
}

fn inverse_distance_weighting(points: &SurfacePoints, array: &mut Array<u32, (Ix, Ix)>) {
        for (i, height) in array.indexed_iter_mut() {
            let (sum_numerator, sum_denominator) = points.iter().fold((0.0, 0.0), |acc, &p| {
                let pos = (i.0 as f32, i.1 as f32);
                let weight = weight(distance((p.x as f32, p.y as f32), pos)) * p.z as f32;
                (acc.0 + weight, acc.1 + weight)
            });
            // *height = 1;
        }

        // size_t      x;
        // size_t      y;
        // float2      pos;
        // float       sum_numerator;
        // float       sum_denominator;

        // x = get_global_id(0);
        // y = get_global_id(1);
        // pos.x = x;
        // pos.y = y;
        // sum_numerator = 0.f;
        // sum_denominator = 0.f;
        // for (uint i = 0; i < nbr_control_points; ++i)
        // {
        //     float       dist;
        //     float       weight;
        //
        //     dist = distance(control_points[i].xy, pos);
        //     weight = get_weight(dist);
        //     sum_numerator += weight * control_points[i].z;
        //     sum_denominator += weight;
        // }
        //
        // //*
        // float       edge_dist;
        // float       edge_weight;
        //
        // edge_dist = distance(0.f, pos.x);
        // edge_weight = get_weight(edge_dist);
        // sum_denominator += edge_weight;
        //
        // edge_dist = distance(map_size.x - 1, pos.x);
        // edge_weight = get_weight(edge_dist);
        // sum_denominator += edge_weight;
        //
        // edge_dist = distance(map_size.y - 1, pos.y);
        // edge_weight = get_weight(edge_dist);
        // sum_denominator += edge_weight;
        //
        // edge_dist = distance(0.f, pos.y);
        // edge_weight = get_weight(edge_dist);
        // sum_denominator += edge_weight;
        // //*/
        //
        // map_heights[(y * map_size.x) + x] = sum_numerator / sum_denominator;
}

impl HeightMap {
    pub fn from_surface_points(sp: &SurfacePoints) -> HeightMap {
        let mut array = Array::from_elem((256, 256), 0);
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
