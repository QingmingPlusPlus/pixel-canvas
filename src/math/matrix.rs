//! 3x3 齐次变换矩阵
//!
//! 用于 2D 变换操作的 3x3 矩阵实现。
//! 通过 trait 抽象接口，便于后续替换为优化实现。

use super::Vec2;

/// 矩阵操作 trait - 抽象接口便于后续升级
pub trait MatrixOperations: Clone {
    /// 创建单位矩阵
    fn identity() -> Self;

    /// 矩阵乘法
    fn multiply(&self, other: &Self) -> Self;

    /// 变换一个 2D 点
    fn transform_point(&self, point: Vec2) -> Vec2;

    /// 创建平移矩阵
    fn translation(tx: f32, ty: f32) -> Self;

    /// 创建旋转矩阵（弧度）
    fn rotation(angle: f32) -> Self;

    /// 创建缩放矩阵
    fn scaling(sx: f32, sy: f32) -> Self;
}

/// 3x3 齐次变换矩阵
///
/// 矩阵布局（行优先）:
/// ```text
/// | m[0] m[1] m[2] |   | a  b  tx |
/// | m[3] m[4] m[5] | = | c  d  ty |
/// | m[6] m[7] m[8] |   | 0  0  1  |
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3x3 {
    /// 矩阵数据，行优先存储
    data: [f32; 9],
}

impl Matrix3x3 {
    /// 从数组创建矩阵
    #[inline]
    pub fn from_array(data: [f32; 9]) -> Self {
        Self { data }
    }

    /// 获取矩阵元素
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * 3 + col]
    }

    /// 设置矩阵元素
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * 3 + col] = value;
    }

    /// 获取原始数据引用
    #[inline]
    pub fn as_array(&self) -> &[f32; 9] {
        &self.data
    }

    /// 计算矩阵行列式（用于判断是否可逆）
    pub fn determinant(&self) -> f32 {
        let m = &self.data;
        m[0] * (m[4] * m[8] - m[5] * m[7]) - m[1] * (m[3] * m[8] - m[5] * m[6])
            + m[2] * (m[3] * m[7] - m[4] * m[6])
    }

    /// 计算逆矩阵
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-10 {
            return None;
        }

        let m = &self.data;
        let inv_det = 1.0 / det;

        Some(Self::from_array([
            (m[4] * m[8] - m[5] * m[7]) * inv_det,
            (m[2] * m[7] - m[1] * m[8]) * inv_det,
            (m[1] * m[5] - m[2] * m[4]) * inv_det,
            (m[5] * m[6] - m[3] * m[8]) * inv_det,
            (m[0] * m[8] - m[2] * m[6]) * inv_det,
            (m[2] * m[3] - m[0] * m[5]) * inv_det,
            (m[3] * m[7] - m[4] * m[6]) * inv_det,
            (m[1] * m[6] - m[0] * m[7]) * inv_det,
            (m[0] * m[4] - m[1] * m[3]) * inv_det,
        ]))
    }
}

impl MatrixOperations for Matrix3x3 {
    #[inline]
    fn identity() -> Self {
        Self::from_array([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }

    fn multiply(&self, other: &Self) -> Self {
        let a = &self.data;
        let b = &other.data;

        Self::from_array([
            // 第一行
            a[0] * b[0] + a[1] * b[3] + a[2] * b[6],
            a[0] * b[1] + a[1] * b[4] + a[2] * b[7],
            a[0] * b[2] + a[1] * b[5] + a[2] * b[8],
            // 第二行
            a[3] * b[0] + a[4] * b[3] + a[5] * b[6],
            a[3] * b[1] + a[4] * b[4] + a[5] * b[7],
            a[3] * b[2] + a[4] * b[5] + a[5] * b[8],
            // 第三行
            a[6] * b[0] + a[7] * b[3] + a[8] * b[6],
            a[6] * b[1] + a[7] * b[4] + a[8] * b[7],
            a[6] * b[2] + a[7] * b[5] + a[8] * b[8],
        ])
    }

    #[inline]
    fn transform_point(&self, point: Vec2) -> Vec2 {
        let m = &self.data;
        Vec2::new(
            m[0] * point.x + m[1] * point.y + m[2],
            m[3] * point.x + m[4] * point.y + m[5],
        )
    }

    #[inline]
    fn translation(tx: f32, ty: f32) -> Self {
        Self::from_array([1.0, 0.0, tx, 0.0, 1.0, ty, 0.0, 0.0, 1.0])
    }

    #[inline]
    fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::from_array([cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0])
    }

    #[inline]
    fn scaling(sx: f32, sy: f32) -> Self {
        Self::from_array([sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0])
    }
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_identity_multiply() {
        let identity = Matrix3x3::identity();
        let m = Matrix3x3::translation(10.0, 20.0);

        let result = identity.multiply(&m);
        assert_eq!(result, m);
    }

    #[test]
    fn test_translation() {
        let m = Matrix3x3::translation(10.0, 20.0);
        let point = Vec2::new(5.0, 5.0);
        let result = m.transform_point(point);

        assert!((result.x - 15.0).abs() < 1e-6);
        assert!((result.y - 25.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotation_90_degrees() {
        let m = Matrix3x3::rotation(PI / 2.0);
        let point = Vec2::new(1.0, 0.0);
        let result = m.transform_point(point);

        assert!(result.x.abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_scaling() {
        let m = Matrix3x3::scaling(2.0, 3.0);
        let point = Vec2::new(5.0, 5.0);
        let result = m.transform_point(point);

        assert!((result.x - 10.0).abs() < 1e-6);
        assert!((result.y - 15.0).abs() < 1e-6);
    }

    #[test]
    fn test_combined_transform() {
        // 先缩放，再旋转，再平移
        let scale = Matrix3x3::scaling(2.0, 2.0);
        let rotate = Matrix3x3::rotation(PI / 2.0);
        let translate = Matrix3x3::translation(10.0, 10.0);

        // 变换顺序：translate * rotate * scale
        let combined = translate.multiply(&rotate).multiply(&scale);

        let point = Vec2::new(1.0, 0.0);
        let result = combined.transform_point(point);

        // (1,0) -> 缩放 (2,0) -> 旋转90° (0,2) -> 平移 (10, 12)
        assert!((result.x - 10.0).abs() < 1e-6);
        assert!((result.y - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse() {
        let m = Matrix3x3::translation(10.0, 20.0);
        let inv = m.inverse().unwrap();
        let result = m.multiply(&inv);

        // 应该得到单位矩阵
        let identity = Matrix3x3::identity();
        for i in 0..9 {
            assert!((result.as_array()[i] - identity.as_array()[i]).abs() < 1e-6);
        }
    }
}
