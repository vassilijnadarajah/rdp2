// SPDX-License-Identifier: GPL-3.0-only
// (C) 2025 Vassilij Nadarajah
// packages@vassilijnadarajah.de

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Point2D { x: x.into(), y: y.into() }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn norm(self) -> Self {
        let magnitude: f64 = self.abs();
        Point2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point2D {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        (self.x * other.x + self.y * other.y) as f64
    }
}

impl Mul<f64> for Point2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self { 
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub trait AsPoint2D {
    fn to_point2d(&self) -> Point2D;
}

impl AsPoint2D for Point2D {
    fn to_point2d(&self) -> Point2D {
        *self
    }
}

impl<T: Into<f64> + Copy> AsPoint2D for Vec<T> {
    fn to_point2d(&self) -> Point2D {
        assert!(self.len() == 2, "Vec must have exactly two elements.");
        Point2D::new(self[0], self[1])
    }
}

impl<T: Into<f64> + Copy> AsPoint2D for [T; 2] {
    fn to_point2d(&self) -> Point2D {
        Point2D::new(self[0], self[1])
    }
}

impl<T: Into<f64> + Copy> AsPoint2D for (T, T) {
    fn to_point2d(&self) -> Point2D {
        Point2D::new(self.0, self.1)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_abs_of_point2d_vector() {
        let point_1: Point2D = Point2D::new(3,4);
        let point_2: Point2D = Point2D::new(0,0);

        let abs_1 = point_1.abs();
        let abs_2 = point_2.abs();

        assert_eq!(abs_1, 5.0);
        assert_eq!(abs_2, 0.0);
    }

    #[test]
    fn calc_norm_of_point2d_vector() {
        let point_1: Point2D = Point2D::new(3,4);

        let norm_1 = point_1.norm();

        assert_eq!(norm_1, Point2D::new(0.6, 0.8));
    }
}
