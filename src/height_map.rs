use ndarray::{ArrayBase, Array, Ix};
use ::surface_points::SurfacePoints;

pub struct HeightMap(Array<u32, (Ix, Ix)>); // FIXME i32 for height ???

fn inverse_distance_weighting(points: &SurfacePoints, array: &mut Array<u32, (Ix, Ix)>) {
    //
}

impl HeightMap {
    pub fn from_surface_points(sp: &SurfacePoints) -> HeightMap {
        let mut array = Array::from_elem((256, 256), 0);
        inverse_distance_weighting(sp, &mut array);
        HeightMap(array)
    }
}


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
