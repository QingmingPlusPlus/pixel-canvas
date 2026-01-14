//! 2D 变换封装
//!
//! 提供便捷的变换操作接口，内部管理变换矩阵

use super::{Matrix3x3, MatrixOperations, Vec2};

/// 2D 变换组件
///
/// 封装位置、旋转、缩放，提供便捷的变换操作接口
#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    /// 位置
    pub position: Vec2,
    /// 旋转角度（弧度）
    pub rotation: f32,
    /// 缩放
    pub scale: Vec2,
    /// 锚点（0-1 范围，相对于精灵尺寸）
    pub anchor: Vec2,
    /// 缓存的变换矩阵
    matrix_cache: Option<Matrix3x3>,
}

impl Transform2D {
    /// 创建新的变换，使用默认值
    pub fn new() -> Self {
        Self {
            position: Vec2::zero(),
            rotation: 0.0,
            scale: Vec2::one(),
            anchor: Vec2::new(0.5, 0.5), // 默认中心锚点
            matrix_cache: None,
        }
    }

    /// 设置位置
    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.position = Vec2::new(x, y);
        self.invalidate_cache();
        self
    }

    /// 设置旋转（弧度）
    #[inline]
    pub fn set_rotation(&mut self, angle: f32) -> &mut Self {
        self.rotation = angle;
        self.invalidate_cache();
        self
    }

    /// 设置旋转（角度）
    #[inline]
    pub fn set_rotation_degrees(&mut self, degrees: f32) -> &mut Self {
        self.rotation = degrees.to_radians();
        self.invalidate_cache();
        self
    }

    /// 设置缩放
    #[inline]
    pub fn set_scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        self.scale = Vec2::new(sx, sy);
        self.invalidate_cache();
        self
    }

    /// 设置均匀缩放
    #[inline]
    pub fn set_uniform_scale(&mut self, s: f32) -> &mut Self {
        self.scale = Vec2::new(s, s);
        self.invalidate_cache();
        self
    }

    /// 设置锚点（0-1 范围）
    #[inline]
    pub fn set_anchor(&mut self, ax: f32, ay: f32) -> &mut Self {
        self.anchor = Vec2::new(ax, ay);
        self.invalidate_cache();
        self
    }

    /// 平移
    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        self.position.x += dx;
        self.position.y += dy;
        self.invalidate_cache();
        self
    }

    /// 旋转（弧度）
    #[inline]
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotation += angle;
        self.invalidate_cache();
        self
    }

    /// 旋转（角度）
    #[inline]
    pub fn rotate_degrees(&mut self, degrees: f32) -> &mut Self {
        self.rotation += degrees.to_radians();
        self.invalidate_cache();
        self
    }

    /// 缩放（乘法）
    #[inline]
    pub fn scale_by(&mut self, sx: f32, sy: f32) -> &mut Self {
        self.scale.x *= sx;
        self.scale.y *= sy;
        self.invalidate_cache();
        self
    }

    /// 获取变换矩阵
    ///
    /// 变换顺序：缩放 -> 旋转 -> 平移
    /// 注意：锚点需要在渲染时与精灵尺寸结合使用
    pub fn matrix(&mut self) -> Matrix3x3 {
        if let Some(cached) = self.matrix_cache {
            return cached;
        }

        // 构建变换矩阵：T * R * S
        let translate = Matrix3x3::translation(self.position.x, self.position.y);
        let rotate = Matrix3x3::rotation(self.rotation);
        let scale = Matrix3x3::scaling(self.scale.x, self.scale.y);

        let matrix = translate.multiply(&rotate).multiply(&scale);
        self.matrix_cache = Some(matrix);
        matrix
    }

    /// 获取带尺寸的完整变换矩阵（考虑锚点）
    pub fn matrix_with_size(&mut self, width: f32, height: f32) -> Matrix3x3 {
        // 锚点偏移
        let anchor_offset = Matrix3x3::translation(-self.anchor.x * width, -self.anchor.y * height);

        // 完整变换：T * R * S * AnchorOffset
        let base_matrix = self.matrix();
        base_matrix.multiply(&anchor_offset)
    }

    /// 变换一个点
    #[inline]
    pub fn transform_point(&mut self, point: Vec2) -> Vec2 {
        self.matrix().transform_point(point)
    }

    /// 清除缓存
    #[inline]
    fn invalidate_cache(&mut self) {
        self.matrix_cache = None;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_default_transform() {
        let mut transform = Transform2D::new();
        let point = Vec2::new(1.0, 1.0);
        let result = transform.transform_point(point);

        // 默认变换不应改变点
        assert!((result.x - 1.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_chained_operations() {
        let mut transform = Transform2D::new();
        transform
            .set_position(10.0, 20.0)
            .set_rotation(PI / 2.0)
            .set_scale(2.0, 2.0);

        assert!((transform.position.x - 10.0).abs() < 1e-6);
        assert!((transform.rotation - PI / 2.0).abs() < 1e-6);
        assert!((transform.scale.x - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_translation() {
        let mut transform = Transform2D::new();
        transform.set_position(10.0, 20.0);

        let point = Vec2::new(5.0, 5.0);
        let result = transform.transform_point(point);

        assert!((result.x - 15.0).abs() < 1e-6);
        assert!((result.y - 25.0).abs() < 1e-6);
    }
}
