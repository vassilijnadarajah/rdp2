// SPDX-License-Identifier: GPL-3.0-only
// (C) 2025 Vassilij Nadarajah
// packages@vassilijnadarajah.de

pub mod objects;

use objects::{Point2D, AsPoint2D};
use pyo3::prelude::*;

#[pymodule]
fn rdp2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rdp_wrapper, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(name = "rdp")]
#[pyo3(signature = (points, epsilon = 1e-3))]
fn rdp_wrapper(points: Vec<[f64; 2]>, epsilon: f64) -> Option<Vec<[f64; 2]>> {
    rdp(&points, epsilon)
}

/// ## Ramer-Douglas-Peucker algorithm for line simplification
/// 
/// This function implements the Douglas-Peucker algorithm for line simplification.
/// It takes a vector of points and an epsilon value as input.
/// The algorithm removes points from the line that are within the epsilon distance
/// from the line segment formed by the endpoints.
/// It returns a new vector of points representing the simplified line.
/// 
/// ### Parameters
/// - `points`: A vector of `Point2D` representing the original line.
/// - `epsilon`: A `f64` value representing the tolerance for point removal.
/// 
/// ### Returns
/// - `Option<Vec<Point2D>>`: An `Option` containing a vector of `Point2D` representing the simplified line.
/// If the input vector has less than 3 points, it returns `None`.
pub fn rdp<T: AsPoint2D + Copy>(points: &Vec<T>, epsilon: f64) -> Option<Vec<T>>
{
    if points.len() < 3 {
        return None;
    }
    
    let mut smooth_line: Vec<T> = Vec::new();
    let mask_for_point_removal: Vec<bool> = calc_rdp_mask(&points, epsilon);
    for i in 0..points.len() {
        if !mask_for_point_removal[i] {
            smooth_line.push(points[i]);
        }
    }

    return Some(smooth_line);
}

fn calc_perpendicular_distance(point: Point2D, line_start: Point2D, line_end: Point2D) -> f64 {
    // Check if line points are identical
    if line_start == line_end {
        return (point - line_start).abs();
    }

    // distance = |(a-p)-((a-p)*n)n|
    let n: Point2D = (line_end - line_start).norm();
    let ap: Point2D = line_start - point;
    (ap - n * (ap * n)).abs()
}

fn calc_rdp_mask<T: AsPoint2D + Copy>(points: &Vec<T>, epsilon: f64) -> Vec<bool>
{
    let mut indices: Vec<(usize, usize)> = Vec::new();
    indices.push((0, points.len() - 1));
    let global_start_index: usize = 0;
    let mut mask_for_point_removal: Vec<bool> = vec![false; points.len()];

    while let Some((start_index, last_index)) = indices.pop() {
        let mut max_distance: f64 = 0.0;
        let mut index = start_index;

        for i in (index + 1)..last_index {
            if mask_for_point_removal[i - global_start_index] {
                continue;
            }

            let distance: f64 = calc_perpendicular_distance(
                points[i].to_point2d(), 
                points[start_index].to_point2d(), 
                points[last_index].to_point2d()
            );
            if distance > max_distance {
                index = i;
                max_distance = distance;
            }
        }

        if max_distance > epsilon {
            indices.push((start_index, index));
            indices.push((index, last_index));
        } else {
            for i in start_index + 1..last_index {
                mask_for_point_removal[i - global_start_index] = true;
            }
        }
    }

    return mask_for_point_removal
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_perpendicular_distance_of_point_from_line() {
        // Arrange
        let point: Point2D = Point2D::new(1.0, 1.0);
        let line_start: Point2D = Point2D::new(0.0, 0.0);
        let line_end: Point2D = Point2D::new(2.0, 0.0);

        // Act
        let result: f64 = calc_perpendicular_distance(point, line_start, line_end);

        // Assert
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calc_rdp_for_line_of_array_points() {
        let line = vec![
            [44, 95],
            [26, 91],
            [22, 90],
            [21, 90],
            [19, 89],
            [17, 89],
            [15, 87],
            [15, 86],
            [16, 85],
            [20, 83],
            [26, 81],
            [28, 80],
            [30, 79],
            [32, 74],
            [32, 72],
            [33, 71],
            [34, 70],
            [38, 68],
            [43, 66],
            [49, 64],
            [52, 63],
            [52, 62],
            [53, 59],
            [54, 57],
            [56, 56],
            [57, 56],
            [58, 56],
            [59, 56],
            [60, 56],
            [61, 55],
            [61, 55],
            [63, 55],
            [64, 55],
            [65, 54],
            [67, 54],
            [68, 54],
            [76, 53],
            [82, 52],
            [84, 52],
            [87, 51],
            [91, 51],
            [93, 51],
            [95, 51],
            [98, 50],
            [105, 50],
            [113, 49],
            [120, 48],
            [127, 48],
            [131, 47],
            [134, 47],
            [137, 47],
            [139, 47],
            [140, 47],
            [142, 47],
            [145, 46],
            [148, 46],
            [152, 46],
            [154, 46],
            [155, 46],
            [159, 46],
            [160, 46],
            [165, 46],
            [168, 46],
            [169, 45],
            [171, 45],
            [173, 45],
            [176, 45],
            [182, 45],
            [190, 44],
            [204, 43],
            [204, 43],
            [207, 43],
            [215, 40],
            [215, 38],
            [215, 37],
            [200, 37],
            [195, 41],
        ];
        let expected_line = vec![
            [44, 95],
            [17, 89],
            [15, 86],
            [30, 79],
            [32, 72],
            [34, 70],
            [52, 63],
            [54, 57],
            [56, 56],
            [87, 51],
            [131, 47],
            [207, 43],
            [215, 40],
            [215, 37],
            [200, 37],
            [195, 41],
        ];

        // Act
        let result = rdp(&line, 1.0);

        // Assert
        assert!(result.is_some());
        let smooth_line = result.unwrap();
        assert!(smooth_line.len() == 16);
        for i in 0..smooth_line.len() {
            assert!(expected_line[i] == smooth_line[i]);
            assert!(expected_line[i] == smooth_line[i]);
        }
    }

    #[test]
    fn calc_rdp_for_line_of_point2d() {
        // Arrange
        let line = vec![
            Point2D::new(44, 95),
            Point2D::new(26, 91),
            Point2D::new(22, 90),
            Point2D::new(21, 90),
            Point2D::new(19, 89),
            Point2D::new(17, 89),
            Point2D::new(15, 87),
            Point2D::new(15, 86),
            Point2D::new(16, 85),
            Point2D::new(20, 83),
            Point2D::new(26, 81),
            Point2D::new(28, 80),
            Point2D::new(30, 79),
            Point2D::new(32, 74),
            Point2D::new(32, 72),
            Point2D::new(33, 71),
            Point2D::new(34, 70),
            Point2D::new(38, 68),
            Point2D::new(43, 66),
            Point2D::new(49, 64),
            Point2D::new(52, 63),
            Point2D::new(52, 62),
            Point2D::new(53, 59),
            Point2D::new(54, 57),
            Point2D::new(56, 56),
            Point2D::new(57, 56),
            Point2D::new(58, 56),
            Point2D::new(59, 56),
            Point2D::new(60, 56),
            Point2D::new(61, 55),
            Point2D::new(61, 55),
            Point2D::new(63, 55),
            Point2D::new(64, 55),
            Point2D::new(65, 54),
            Point2D::new(67, 54),
            Point2D::new(68, 54),
            Point2D::new(76, 53),
            Point2D::new(82, 52),
            Point2D::new(84, 52),
            Point2D::new(87, 51),
            Point2D::new(91, 51),
            Point2D::new(93, 51),
            Point2D::new(95, 51),
            Point2D::new(98, 50),
            Point2D::new(105, 50),
            Point2D::new(113, 49),
            Point2D::new(120, 48),
            Point2D::new(127, 48),
            Point2D::new(131, 47),
            Point2D::new(134, 47),
            Point2D::new(137, 47),
            Point2D::new(139, 47),
            Point2D::new(140, 47),
            Point2D::new(142, 47),
            Point2D::new(145, 46),
            Point2D::new(148, 46),
            Point2D::new(152, 46),
            Point2D::new(154, 46),
            Point2D::new(155, 46),
            Point2D::new(159, 46),
            Point2D::new(160, 46),
            Point2D::new(165, 46),
            Point2D::new(168, 46),
            Point2D::new(169, 45),
            Point2D::new(171, 45),
            Point2D::new(173, 45),
            Point2D::new(176, 45),
            Point2D::new(182, 45),
            Point2D::new(190, 44),
            Point2D::new(204, 43),
            Point2D::new(204, 43),
            Point2D::new(207, 43),
            Point2D::new(215, 40),
            Point2D::new(215, 38),
            Point2D::new(215, 37),
            Point2D::new(200, 37),
            Point2D::new(195, 41),
        ];
        let expected_line = vec![
            Point2D::new(44, 95),
            Point2D::new(17, 89),
            Point2D::new(15, 86),
            Point2D::new(30, 79),
            Point2D::new(32, 72),
            Point2D::new(34, 70),
            Point2D::new(52, 63),
            Point2D::new(54, 57),
            Point2D::new(56, 56),
            Point2D::new(87, 51),
            Point2D::new(131, 47),
            Point2D::new(207, 43),
            Point2D::new(215, 40),
            Point2D::new(215, 37),
            Point2D::new(200, 37),
            Point2D::new(195, 41),
        ];

        
        // Act
        let result = rdp(&line, 1.0);
        
        // Assert
        assert!(result.is_some());
        let smooth_line = result.unwrap();
        assert!(smooth_line.len() == 16);
        for i in 0..smooth_line.len() {
            assert!(expected_line[i] == smooth_line[i]);
            assert!(expected_line[i] == smooth_line[i]);
        }
    }
}
