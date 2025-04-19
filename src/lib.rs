mod objects;

use objects::Point2d;

fn calc_perpendicular_distance(point: Point2d, line_start: Point2d, line_end: Point2d) -> f64 {
    // distance = |(a-p)-((a-p)*n)n)|
    let n: Point2d = (line_end - line_start).norm();
    let ap: Point2d = line_start - point;
    (ap - n * (ap * n)).abs()
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
}
