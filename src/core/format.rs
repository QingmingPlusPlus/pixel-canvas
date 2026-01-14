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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format_values() {
        // 验证每种格式对应正确的通道数
        assert_eq!(ImageFormat::Grayscale as u32, 1);
        assert_eq!(ImageFormat::Rgb as u32, 3);
        assert_eq!(ImageFormat::Rgba as u32, 4);
    }

    #[test]
    fn test_image_format_clone() {
        let format = ImageFormat::Rgba;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_image_format_copy() {
        let format = ImageFormat::Rgb;
        let copied = format;
        assert_eq!(format, copied);
    }

    #[test]
    fn test_image_format_debug() {
        // 测试 Debug trait 实现
        let debug_str = format!("{:?}", ImageFormat::Rgba);
        assert!(debug_str.contains("Rgba"));
    }
}
