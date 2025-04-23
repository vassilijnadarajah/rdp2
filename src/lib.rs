mod objects;

use objects::Point2d;

pub fn rdp(points: &Vec<Point2d>, epsilon: f64) -> Option<Vec<Point2d>> {
    if points.len() < 3 {
        return None;
    }
    
    let mut smooth_line: Vec<Point2d> = Vec::new();
    let mask_for_point_removal: Vec<bool> = calc_rdp_mask(&points, epsilon);
    for i in 0..points.len() {
        if !mask_for_point_removal[i] {
            smooth_line.push(points[i]);
        }
    }

    return Some(smooth_line);
}

fn calc_perpendicular_distance(point: Point2d, line_start: Point2d, line_end: Point2d) -> f64 {
    // distance = |(a-p)-((a-p)*n)n)|
    let n: Point2d = (line_end - line_start).norm();
    let ap: Point2d = line_start - point;
    (ap - n * (ap * n)).abs()
}

fn calc_rdp_mask(points: &Vec<Point2d>, epsilon: f64) -> Vec<bool>
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

            let distance: f64 = calc_perpendicular_distance(points[i], points[start_index], points[last_index]);
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
        let point: Point2d = Point2d::new(1.0, 1.0);
        let line_start: Point2d = Point2d::new(0.0, 0.0);
        let line_end: Point2d = Point2d::new(2.0, 0.0);

        // Act
        let result: f64 = calc_perpendicular_distance(point, line_start, line_end);

        // Assert
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calc_rdp_for_line_of_array_points() {
        // Arrange
        let line: Vec<[f64; 2]> = vec![
            [1.0,1.0],
            [2.0,2.0],
            [3.0,3.0],
            [4.0,4.0],
            [5.0,5.0],
            [6.0,6.0],
            [7.0,7.0],
            [8.0,8.0],
            [9.0,9.0],
        ];

        // Act
        // let smooth_line = rdp_test(line, 1.5);

        // Assert
        // assert!(smooth_line.len() == 1);
    }

    #[test]
    fn calc_rdp_for_line_of_point2d() {
        // Arrange
        let line = vec![
            Point2d::new(44, 95),
            Point2d::new(26, 91),
            Point2d::new(22, 90),
            Point2d::new(21, 90),
            Point2d::new(19, 89),
            Point2d::new(17, 89),
            Point2d::new(15, 87),
            Point2d::new(15, 86),
            Point2d::new(16, 85),
            Point2d::new(20, 83),
            Point2d::new(26, 81),
            Point2d::new(28, 80),
            Point2d::new(30, 79),
            Point2d::new(32, 74),
            Point2d::new(32, 72),
            Point2d::new(33, 71),
            Point2d::new(34, 70),
            Point2d::new(38, 68),
            Point2d::new(43, 66),
            Point2d::new(49, 64),
            Point2d::new(52, 63),
            Point2d::new(52, 62),
            Point2d::new(53, 59),
            Point2d::new(54, 57),
            Point2d::new(56, 56),
            Point2d::new(57, 56),
            Point2d::new(58, 56),
            Point2d::new(59, 56),
            Point2d::new(60, 56),
            Point2d::new(61, 55),
            Point2d::new(61, 55),
            Point2d::new(63, 55),
            Point2d::new(64, 55),
            Point2d::new(65, 54),
            Point2d::new(67, 54),
            Point2d::new(68, 54),
            Point2d::new(76, 53),
            Point2d::new(82, 52),
            Point2d::new(84, 52),
            Point2d::new(87, 51),
            Point2d::new(91, 51),
            Point2d::new(93, 51),
            Point2d::new(95, 51),
            Point2d::new(98, 50),
            Point2d::new(105, 50),
            Point2d::new(113, 49),
            Point2d::new(120, 48),
            Point2d::new(127, 48),
            Point2d::new(131, 47),
            Point2d::new(134, 47),
            Point2d::new(137, 47),
            Point2d::new(139, 47),
            Point2d::new(140, 47),
            Point2d::new(142, 47),
            Point2d::new(145, 46),
            Point2d::new(148, 46),
            Point2d::new(152, 46),
            Point2d::new(154, 46),
            Point2d::new(155, 46),
            Point2d::new(159, 46),
            Point2d::new(160, 46),
            Point2d::new(165, 46),
            Point2d::new(168, 46),
            Point2d::new(169, 45),
            Point2d::new(171, 45),
            Point2d::new(173, 45),
            Point2d::new(176, 45),
            Point2d::new(182, 45),
            Point2d::new(190, 44),
            Point2d::new(204, 43),
            Point2d::new(204, 43),
            Point2d::new(207, 43),
            Point2d::new(215, 40),
            Point2d::new(215, 38),
            Point2d::new(215, 37),
            Point2d::new(200, 37),
            Point2d::new(195, 41),
        ];
        let expected_line = vec![
            Point2d::new(44, 95),
            Point2d::new(17, 89),
            Point2d::new(15, 86),
            Point2d::new(30, 79),
            Point2d::new(32, 72),
            Point2d::new(34, 70),
            Point2d::new(52, 63),
            Point2d::new(54, 57),
            Point2d::new(56, 56),
            Point2d::new(87, 51),
            Point2d::new(131, 47),
            Point2d::new(207, 43),
            Point2d::new(215, 40),
            Point2d::new(215, 37),
            Point2d::new(200, 37),
            Point2d::new(195, 41),
        ];

        
        // Act
        let result = rdp(&line, 1.5);
        
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
