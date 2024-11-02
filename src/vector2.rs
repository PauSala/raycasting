use std::ops::{Add, Mul, Sub};

use crate::utils::{COL_SIZE, ROW_SIZE};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Vector2 {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector2 {
                x: self.x,
                y: self.y,
            }
        } else {
            Vector2 {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }

    pub fn perpendicular(&self) -> Self {
        Vector2 {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        let dot_product = self.dot(other);
        let magnitudes = self.magnitude() * other.magnitude();
        (dot_product / magnitudes).acos()
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Vector2 {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }

    pub fn to_pixel_coords(&self) -> Vector2 {
        Vector2 {
            x: self.x * COL_SIZE as f32,
            y: self.y * ROW_SIZE as f32,
        }
    }

    pub fn to_grid_coords(&self) -> Vector2 {
        Vector2 {
            x: self.x / COL_SIZE as f32,
            y: self.y / ROW_SIZE as f32,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

pub fn point_perpendicular(p1: Vector2, p2: Vector2, distance: f32) -> Vector2 {
    let direction = (p2 - p1).norm();
    let perpendicular = direction.perpendicular();
    p2 + Vector2 {
        x: perpendicular.x * distance,
        y: perpendicular.y * distance,
    }
}

pub fn point_at_angle(p1: Vector2, p2: Vector2, angle: f32) -> Vector2 {
    let direction = (p2 - p1).norm();
    let rotated = direction.rotate(angle);
    p2 + rotated
}

pub fn is_vertical_side(intersection: Vector2, grid_size: f32) -> bool {
    let x_dist = (intersection.x % grid_size).min(grid_size - (intersection.x % grid_size));
    let y_dist = (intersection.y % grid_size).min(grid_size - (intersection.y % grid_size));
    x_dist < y_dist
}
