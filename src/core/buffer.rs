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
