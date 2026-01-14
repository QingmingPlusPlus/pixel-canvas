//! 核心模块
//!
//! 包含 SharedBuffer 和 ImageFormat

mod buffer;
pub(crate) mod format;

// 导出核心类型
pub use buffer::SharedBuffer;
pub use format::ImageFormat;
