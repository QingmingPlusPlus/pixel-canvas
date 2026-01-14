use wasm_bindgen::prelude::*;

use super::format::ImageFormat;

/// SharedBuffer - 一个可以在 Rust 和 JS 之间共享的内存区域
#[wasm_bindgen]
pub struct SharedBuffer {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: ImageFormat,
    pub(crate) buffer: Vec<u8>,
}

#[wasm_bindgen]
impl SharedBuffer {
    /// 创建一个新的 SharedBuffer，根据 宽、高、格式 分配内存
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, format: ImageFormat) -> SharedBuffer {
        let size = (width * height * (format as u32)) as usize;
        SharedBuffer {
            width,
            height,
            format,
            buffer: vec![0; size],
        }
    }

    /// 返回 buffer 的指针，供 JS 端访问
    pub fn ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    /// 返回 buffer 的长度
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// 返回图像宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 返回图像高度
    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_buffer_new_rgba() {
        let buffer = SharedBuffer::new(100, 100, ImageFormat::Rgba);
        assert_eq!(buffer.width(), 100);
        assert_eq!(buffer.height(), 100);
        assert_eq!(buffer.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_shared_buffer_new_rgb() {
        let buffer = SharedBuffer::new(50, 50, ImageFormat::Rgb);
        assert_eq!(buffer.width(), 50);
        assert_eq!(buffer.height(), 50);
        assert_eq!(buffer.len(), 50 * 50 * 3);
    }

    #[test]
    fn test_shared_buffer_new_grayscale() {
        let buffer = SharedBuffer::new(32, 32, ImageFormat::Grayscale);
        assert_eq!(buffer.width(), 32);
        assert_eq!(buffer.height(), 32);
        assert_eq!(buffer.len(), 32 * 32 * 1);
    }

    #[test]
    fn test_shared_buffer_ptr_not_null() {
        let buffer = SharedBuffer::new(10, 10, ImageFormat::Rgba);
        assert!(!buffer.ptr().is_null());
    }

    #[test]
    fn test_shared_buffer_initial_values_zero() {
        let buffer = SharedBuffer::new(10, 10, ImageFormat::Rgba);
        // 验证 buffer 初始化为全零
        for byte in buffer.buffer.iter() {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_shared_buffer_different_sizes() {
        // 测试不同尺寸的 buffer
        let sizes = [(1, 1), (100, 200), (1920, 1080)];
        for (w, h) in sizes {
            let buffer = SharedBuffer::new(w, h, ImageFormat::Rgba);
            assert_eq!(buffer.width(), w);
            assert_eq!(buffer.height(), h);
            assert_eq!(buffer.len(), (w * h * 4) as usize);
        }
    }
}
