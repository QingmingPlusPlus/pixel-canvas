use wasm_bindgen::prelude::*;

/// 图像格式枚举
/// 定义支持的图像通道格式
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    /// 灰度图像，单通道
    Grayscale = 1,
    /// RGB 图像，三通道
    Rgb = 3,
    /// RGBA 图像，四通道（带 Alpha）
    Rgba = 4,
}
