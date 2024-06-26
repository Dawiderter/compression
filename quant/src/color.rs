use std::ops::{Add, Div};

#[derive(Debug, Clone, Copy)]
pub struct ColorVector {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorVector {
    pub fn from_bytes(bytes: (u8, u8, u8)) -> Self {
        Self {
            r: bytes.0 as f32,
            g: bytes.1 as f32,
            b: bytes.2 as f32,
        }
    }

    pub fn to_bytes(self) -> (u8, u8, u8) {
        (self.r as u8, self.g as u8, self.b as u8)
    }

    pub fn clamp(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 255.0),
            g: self.g.clamp(0.0, 255.0),
            b: self.b.clamp(0.0, 255.0),
        }
    }

    pub fn manhattan(self, other: Self) -> f32 {
        (self.r - other.r).abs() + (self.g - other.g).abs() + (self.b - other.b).abs()
    }

    pub fn eucl_sq(self, other: Self) -> f32 {
        (self.r - other.r).powi(2) + (self.g - other.g).powi(2) + (self.b - other.b).powi(2)
    }

    pub fn len_sq(self) -> f32 {
        (self.r).powi(2) + (self.g).powi(2) + (self.b).powi(2)
    }

    pub fn len_manh(self) -> f32 {
        (self.r).abs() + (self.g).abs() + (self.b).abs()
    }
}

impl Add for ColorVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Div<f32> for ColorVector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
