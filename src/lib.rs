//! pixel-canvas - 像素画布 WebAssembly 库
//!
//! 提供高性能的图像缓冲区，用于 Rust 和 JavaScript 之间共享内存。

mod buffer;
mod effects;
mod format;

// 导出核心类型
pub use buffer::SharedBuffer;
pub use format::ImageFormat;
