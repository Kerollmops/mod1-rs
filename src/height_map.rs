use ndarray::{Array, Ix};
use ::surface_points::{SurfacePoints, SurfacePoint};

#[derive(Debug)]
pub struct HeightMap(Array<f32, (Ix, Ix)>);

#[inline]
fn weight(dist: f32) -> f32 {
    if dist == 0.0 { 1.0 } else { 1.0 / (dist * dist) }
}

#[inline]
fn distance((a_x, a_y): (f32, f32), (b_x, b_y): (f32, f32)) -> f32 {
    let dist_x = a_x - b_x;
    let dist_y = a_y - b_y;
    ((dist_x * dist_x) + (dist_y * dist_y)).sqrt()
}

fn inverse_distance_weighting(points: &SurfacePoints, array: &mut Array<f32, (Ix, Ix)>) {
    let (rows, cols) = (array.rows() as f32, array.cols() as f32);
    for (pos, height) in array.indexed_iter_mut() {
        let pos = (pos.0 as f32, pos.1 as f32);
        let (numerator, mut denominator) = points.iter().fold((0.0, 0.0), |acc, &point| {
            let SurfacePoint{ x, y, z } = point;
            let dist = distance((x as f32, y as f32), pos);
            let weight = weight(dist);
            let add = if dist != 0.0 { weight * z as f32 } else { z as f32 };
            (acc.0 + add, acc.1 + weight)
        });
        denominator += weight(pos.0);              // dist (x, 0)
        denominator += weight(pos.1);              // dist (0, y)
        denominator += weight(rows - 1.0 - pos.0); // dist (x_max, 0)
        denominator += weight(cols - 1.0 - pos.1); // dist (0, y_max)
        *height = numerator / denominator;
    }
}

impl HeightMap {
    pub fn from_surface_points(sp: &SurfacePoints) -> HeightMap {
        let mut array = Array::from_elem((4, 8), 0.0);
        inverse_distance_weighting(sp, &mut array);
        HeightMap(array)
    }
}
