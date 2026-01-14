//! 精灵系统
//!
//! 提供精灵 trait 和具体实现

use crate::format::ImageFormat;
use crate::math::{Matrix3x3, MatrixOperations, Transform2D, Vec2};

/// 精灵 trait - 面向接口编程
///
/// 所有可渲染的 2D 对象都应实现此 trait
pub trait Sprite {
    /// 获取精灵宽度
    fn width(&self) -> u32;

    /// 获取精灵高度
    fn height(&self) -> u32;

    /// 获取 z-order 渲染层级
    fn z_order(&self) -> i32;

    /// 设置 z-order
    fn set_z_order(&mut self, z: i32);

    /// 获取变换引用
    fn transform(&self) -> &Transform2D;

    /// 获取变换可变引用
    fn transform_mut(&mut self) -> &mut Transform2D;

    /// 获取变换矩阵（带尺寸）
    fn get_transform_matrix(&mut self) -> Matrix3x3 {
        let w = self.width() as f32;
        let h = self.height() as f32;
        self.transform_mut().matrix_with_size(w, h)
    }

    /// 渲染到目标 buffer
    ///
    /// # Arguments
    /// * `target` - 目标 buffer（RGBA 格式）
    /// * `target_width` - 目标宽度
    /// * `target_height` - 目标高度
    fn render_to(&mut self, target: &mut [u8], target_width: u32, target_height: u32);

    /// 获取唯一标识符（用于精灵管理）
    fn id(&self) -> u64;
}

/// 图像精灵 - 持有图像数据的精灵
#[derive(Debug)]
pub struct ImageSprite {
    /// 唯一标识符
    id: u64,
    /// 图像数据（RGBA 格式）
    buffer: Vec<u8>,
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
    /// 图像格式
    format: ImageFormat,
    /// 变换信息
    transform: Transform2D,
    /// 渲染层级
    z_order: i32,
}

/// ID 生成器
static mut NEXT_SPRITE_ID: u64 = 0;

fn generate_sprite_id() -> u64 {
    unsafe {
        let id = NEXT_SPRITE_ID;
        NEXT_SPRITE_ID += 1;
        id
    }
}

impl ImageSprite {
    /// 创建新的图像精灵
    ///
    /// # Arguments
    /// * `width` - 图像宽度
    /// * `height` - 图像高度
    /// * `format` - 图像格式
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        let size = (width * height * (format as u32)) as usize;
        Self {
            id: generate_sprite_id(),
            buffer: vec![0; size],
            width,
            height,
            format,
            transform: Transform2D::new(),
            z_order: 0,
        }
    }

    /// 从现有数据创建图像精灵
    ///
    /// # Arguments
    /// * `buffer` - 图像数据
    /// * `width` - 图像宽度
    /// * `height` - 图像高度
    /// * `format` - 图像格式
    pub fn from_buffer(buffer: Vec<u8>, width: u32, height: u32, format: ImageFormat) -> Self {
        Self {
            id: generate_sprite_id(),
            buffer,
            width,
            height,
            format,
            transform: Transform2D::new(),
            z_order: 0,
        }
    }

    /// 获取 buffer 引用
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// 获取 buffer 可变引用
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    /// 获取图像格式
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    // ===== 变换操作便捷方法 =====

    /// 设置位置
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_position(x, y);
        self
    }

    /// 设置旋转（弧度）
    pub fn set_rotation(&mut self, angle: f32) -> &mut Self {
        self.transform.set_rotation(angle);
        self
    }

    /// 设置旋转（角度）
    pub fn set_rotation_degrees(&mut self, degrees: f32) -> &mut Self {
        self.transform.set_rotation_degrees(degrees);
        self
    }

    /// 设置缩放
    pub fn set_scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        self.transform.set_scale(sx, sy);
        self
    }

    /// 设置均匀缩放
    pub fn set_uniform_scale(&mut self, s: f32) -> &mut Self {
        self.transform.set_uniform_scale(s);
        self
    }

    /// 设置锚点
    pub fn set_anchor(&mut self, ax: f32, ay: f32) -> &mut Self {
        self.transform.set_anchor(ax, ay);
        self
    }

    /// 平移
    pub fn translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        self.transform.translate(dx, dy);
        self
    }

    /// 旋转
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.transform.rotate(angle);
        self
    }

    /// 旋转（角度）
    pub fn rotate_degrees(&mut self, degrees: f32) -> &mut Self {
        self.transform.rotate_degrees(degrees);
        self
    }

    /// 获取指定位置的像素（转换为 RGBA）
    fn get_pixel_rgba(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * self.width + x) as usize;
        match self.format {
            ImageFormat::Rgba => {
                let base = idx * 4;
                [
                    self.buffer[base],
                    self.buffer[base + 1],
                    self.buffer[base + 2],
                    self.buffer[base + 3],
                ]
            }
            ImageFormat::Rgb => {
                let base = idx * 3;
                [
                    self.buffer[base],
                    self.buffer[base + 1],
                    self.buffer[base + 2],
                    255,
                ]
            }
            ImageFormat::Grayscale => {
                let gray = self.buffer[idx];
                [gray, gray, gray, 255]
            }
        }
    }
}

impl Sprite for ImageSprite {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn z_order(&self) -> i32 {
        self.z_order
    }

    fn set_z_order(&mut self, z: i32) {
        self.z_order = z;
    }

    fn transform(&self) -> &Transform2D {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform2D {
        &mut self.transform
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn render_to(&mut self, target: &mut [u8], target_width: u32, target_height: u32) {
        let matrix = self.get_transform_matrix();
        let inv_matrix = match matrix.inverse() {
            Some(inv) => inv,
            None => return, // 矩阵不可逆，跳过渲染
        };

        let sprite_w = self.width as f32;
        let sprite_h = self.height as f32;

        // 遍历目标像素
        for ty in 0..target_height {
            for tx in 0..target_width {
                // 逆变换获取源坐标
                let target_point = Vec2::new(tx as f32, ty as f32);
                let source_point = inv_matrix.transform_point(target_point);

                let sx = source_point.x;
                let sy = source_point.y;

                // 边界检查
                if sx >= 0.0 && sx < sprite_w && sy >= 0.0 && sy < sprite_h {
                    let src_x = sx as u32;
                    let src_y = sy as u32;

                    let pixel = self.get_pixel_rgba(src_x, src_y);

                    // Alpha 混合
                    let alpha = pixel[3] as f32 / 255.0;
                    if alpha > 0.0 {
                        let target_idx = ((ty * target_width + tx) * 4) as usize;

                        if alpha >= 1.0 {
                            // 完全不透明，直接覆盖
                            target[target_idx] = pixel[0];
                            target[target_idx + 1] = pixel[1];
                            target[target_idx + 2] = pixel[2];
                            target[target_idx + 3] = 255;
                        } else {
                            // Alpha 混合
                            let inv_alpha = 1.0 - alpha;
                            target[target_idx] = (pixel[0] as f32 * alpha
                                + target[target_idx] as f32 * inv_alpha)
                                as u8;
                            target[target_idx + 1] = (pixel[1] as f32 * alpha
                                + target[target_idx + 1] as f32 * inv_alpha)
                                as u8;
                            target[target_idx + 2] = (pixel[2] as f32 * alpha
                                + target[target_idx + 2] as f32 * inv_alpha)
                                as u8;
                            target[target_idx + 3] = ((alpha
                                + target[target_idx + 3] as f32 / 255.0 * inv_alpha)
                                * 255.0) as u8;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sprite() {
        let sprite = ImageSprite::new(100, 100, ImageFormat::Rgba);
        assert_eq!(sprite.width(), 100);
        assert_eq!(sprite.height(), 100);
        assert_eq!(sprite.buffer().len(), 100 * 100 * 4);
    }

    #[test]
    fn test_sprite_transform() {
        let mut sprite = ImageSprite::new(50, 50, ImageFormat::Rgba);
        sprite.set_position(10.0, 20.0).set_rotation_degrees(45.0);

        let transform = sprite.transform();
        assert!((transform.position.x - 10.0).abs() < 1e-6);
        assert!((transform.position.y - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_z_order() {
        let mut sprite = ImageSprite::new(10, 10, ImageFormat::Rgba);
        sprite.set_z_order(5);
        assert_eq!(sprite.z_order(), 5);
    }
}
