use wasm_bindgen::prelude::*;

/// SharedBuffer - 一个可以在 Rust 和 JS 之间共享的内存区域
#[wasm_bindgen]
pub struct SharedBuffer {
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl SharedBuffer {
    /// 创建一个新的 SharedBuffer，分配指定大小的内存
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> SharedBuffer {
        SharedBuffer {
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

    /// 刷新 buffer，填充随机值
    pub fn refresh(&mut self) {
        getrandom::getrandom(&mut self.buffer).expect("Failed to generate random bytes");
    }
}
