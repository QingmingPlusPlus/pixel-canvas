//! 2D 向量类型
//!
//! 提供基础的 2D 向量运算

use std::ops::{Add, Mul, Sub};

/// 2D 向量
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// 创建新向量
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 零向量
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// 单位向量 (1, 1)
    #[inline]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// 向量长度的平方（避免开方运算）
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// 向量长度
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// 归一化向量
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }

    /// 点积
    #[inline]
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// 2D 叉积（返回标量，表示 z 分量）
    #[inline]
    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(1.0, 2.0);

        assert_eq!(a + b, Vec2::new(4.0, 6.0));
        assert_eq!(a - b, Vec2::new(2.0, 2.0));
        assert_eq!(a * 2.0, Vec2::new(6.0, 8.0));
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert!((a.dot(&b)).abs() < 1e-6); // 垂直向量点积为0
    }
}
