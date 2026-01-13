use wasm_bindgen::prelude::*;

use crate::buffer::SharedBuffer;
use crate::format::ImageFormat;

/// SharedBuffer 的图像效果扩展
#[wasm_bindgen]
impl SharedBuffer {
    /// 测试方法：生成渐变
    /// start_color / end_color: 0xRRGGBBAA packed u32
    pub fn test_gradient(&mut self, start_color: u32, end_color: u32) {
        let start_r = ((start_color >> 24) & 0xFF) as f32;
        let start_g = ((start_color >> 16) & 0xFF) as f32;
        let start_b = ((start_color >> 8) & 0xFF) as f32;
        let start_a = (start_color & 0xFF) as f32;

        let end_r = ((end_color >> 24) & 0xFF) as f32;
        let end_g = ((end_color >> 16) & 0xFF) as f32;
        let end_b = ((end_color >> 8) & 0xFF) as f32;
        let end_a = (end_color & 0xFF) as f32;

        let w = self.width as f32;
        let h = self.height as f32;
        let max_dist = w + h;

        match self.format {
            ImageFormat::Rgba => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let factor = (x as f32 + y as f32) / max_dist;

                        let r = (start_r + (end_r - start_r) * factor) as u8;
                        let g = (start_g + (end_g - start_g) * factor) as u8;
                        let b = (start_b + (end_b - start_b) * factor) as u8;
                        let a = (start_a + (end_a - start_a) * factor) as u8;

                        let idx = ((y * self.width + x) * 4) as usize;
                        self.buffer[idx] = r;
                        self.buffer[idx + 1] = g;
                        self.buffer[idx + 2] = b;
                        self.buffer[idx + 3] = a;
                    }
                }
            }
            ImageFormat::Rgb => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let factor = (x as f32 + y as f32) / max_dist;
                        let r = (start_r + (end_r - start_r) * factor) as u8;
                        let g = (start_g + (end_g - start_g) * factor) as u8;
                        let b = (start_b + (end_b - start_b) * factor) as u8;

                        let idx = ((y * self.width + x) * 3) as usize;
                        self.buffer[idx] = r;
                        self.buffer[idx + 1] = g;
                        self.buffer[idx + 2] = b;
                    }
                }
            }
            ImageFormat::Grayscale => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let factor = (x as f32 + y as f32) / max_dist;
                        let r = start_r + (end_r - start_r) * factor;
                        let g = start_g + (end_g - start_g) * factor;
                        let b = start_b + (end_b - start_b) * factor;

                        // 简单的灰度转换: 0.299R + 0.587G + 0.114B
                        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

                        let idx = ((y * self.width + x) * 1) as usize;
                        self.buffer[idx] = gray;
                    }
                }
            }
        }
    }
}
