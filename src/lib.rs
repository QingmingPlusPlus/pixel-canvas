//! pixel-canvas - 像素画布 WebAssembly 库
//!
//! 提供高性能的图像缓冲区，用于 Rust 和 JavaScript 之间共享内存。

pub(crate) mod core;
pub mod math;
pub mod scene;

// 导出核心类型
pub use core::ImageFormat;
pub use core::SharedBuffer;
pub use core::WasmScene;
pub use math::{Matrix3x3, MatrixOperations, Transform2D, Vec2};
pub use scene::{ImageSprite, Scene, Sprite};
