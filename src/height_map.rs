use ndarray::{Array, Ix};
use ::surface_points::SurfacePoints;

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
    let array_rows = array.cols() as f32;
    let array_cols = array.rows() as f32;
    for (idx, height) in array.indexed_iter_mut() {
        let idx = (idx.0 as f32, idx.1 as f32);
        let (numerator, mut denominator) = points.iter().fold((0.0, 0.0), |acc, &p| {
            let dist = distance((p.x as f32, p.y as f32), idx);
            let weight = weight(dist);
            println!("({:?}), weight: {}", p.z, weight);
            let add = if dist != 0.0 { weight * p.z as f32 } else { p.z as f32 };
            (acc.0 + add, acc.1 + weight)
        });
        // FIXME uncomment this
        // denominator += weight(idx.0);                            // dist (x, 0)
        // denominator += weight(idx.1);                            // dist (0, y)
        // denominator += weight((array_rows - 1.0 - idx.0).abs()); // dist (x_max, 0)
        // denominator += weight((array_cols - 1.0 - idx.1).abs()); // dist (0, y_max)
        *height = numerator / denominator;
        println!("{:?}/{:?}, height: {:?}", numerator, denominator, *height);
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
