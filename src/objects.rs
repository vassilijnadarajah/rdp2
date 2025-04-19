use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn norm(self) -> Self {
        let magnitude: f64 = self.abs();
        Point2d {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

}

impl Add for Point2d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point2d {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        (self.x * other.x + self.y * other.y) as f64
    }
}

impl Mul<f64> for Point2d {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self { 
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
