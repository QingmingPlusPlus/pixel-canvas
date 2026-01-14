//! WASM 演示模块
//!
//! 为 JavaScript 提供简化的 Scene/Sprite 操作接口

use wasm_bindgen::prelude::*;

use crate::format::ImageFormat;
use crate::scene::Scene;
use crate::scene::sprite::ImageSprite;
use crate::scene::sprite::Sprite;

/// WASM 可导出的场景包装器
#[wasm_bindgen]
pub struct WasmScene {
    scene: Scene,
    /// 精灵及其 ID
    sprites: Vec<ImageSprite>,
}

#[wasm_bindgen]
impl WasmScene {
    /// 创建新场景
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> WasmScene {
        WasmScene {
            scene: Scene::new(width, height),
            sprites: Vec::new(),
        }
    }

    /// 设置背景颜色
    pub fn set_background(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.scene.set_background_color(r, g, b, a);
    }

    /// 设置背景颜色（十六进制 0xRRGGBBAA）
    pub fn set_background_hex(&mut self, color: u32) {
        self.scene.set_background_color_hex(color);
    }

    /// 添加一个带颜色填充的精灵
    pub fn add_sprite(&mut self, width: u32, height: u32, color: u32, z_order: i32) -> u32 {
        let mut sprite = ImageSprite::new(width, height, ImageFormat::Rgba);
        sprite.set_z_order(z_order);

        // 填充颜色
        let r = ((color >> 24) & 0xFF) as u8;
        let g = ((color >> 16) & 0xFF) as u8;
        let b = ((color >> 8) & 0xFF) as u8;
        let a = (color & 0xFF) as u8;

        let buffer = sprite.buffer_mut();
        for i in (0..buffer.len()).step_by(4) {
            buffer[i] = r;
            buffer[i + 1] = g;
            buffer[i + 2] = b;
            buffer[i + 3] = a;
        }

        let index = self.sprites.len() as u32;
        self.sprites.push(sprite);
        index
    }

    /// 设置精灵位置
    pub fn set_sprite_position(&mut self, index: u32, x: f32, y: f32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_position(x, y);
        }
    }

    /// 设置精灵旋转（角度）
    pub fn set_sprite_rotation(&mut self, index: u32, degrees: f32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_rotation_degrees(degrees);
        }
    }

    /// 设置精灵缩放
    pub fn set_sprite_scale(&mut self, index: u32, sx: f32, sy: f32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_scale(sx, sy);
        }
    }

    /// 设置精灵均匀缩放
    pub fn set_sprite_uniform_scale(&mut self, index: u32, s: f32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_uniform_scale(s);
        }
    }

    /// 设置精灵锚点
    pub fn set_sprite_anchor(&mut self, index: u32, ax: f32, ay: f32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_anchor(ax, ay);
        }
    }

    /// 设置精灵 z-order
    pub fn set_sprite_z_order(&mut self, index: u32, z: i32) {
        if let Some(sprite) = self.sprites.get_mut(index as usize) {
            sprite.set_z_order(z);
        }
    }

    /// 渲染场景
    pub fn render(&mut self) {
        // 先渲染场景背景
        self.scene.render();

        let width = self.scene.width();
        let height = self.scene.height();

        // 获取 buffer
        let scene_ptr = self.scene.ptr() as *mut u8;
        let scene_len = self.scene.len();
        let scene_buffer = unsafe { std::slice::from_raw_parts_mut(scene_ptr, scene_len) };

        // 按 z-order 排序精灵
        self.sprites.sort_by_key(|s| s.z_order());

        // 渲染每个精灵
        for sprite in self.sprites.iter_mut() {
            sprite.render_to(scene_buffer, width, height);
        }
    }

    /// 获取 buffer 指针
    pub fn ptr(&self) -> *const u8 {
        self.scene.ptr()
    }

    /// 获取 buffer 长度
    pub fn len(&self) -> usize {
        self.scene.len()
    }

    /// 检查场景是否为空
    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// 获取场景宽度
    pub fn width(&self) -> u32 {
        self.scene.width()
    }

    /// 获取场景高度
    pub fn height(&self) -> u32 {
        self.scene.height()
    }

    /// 获取精灵数量
    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    /// 清空所有精灵
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }
}
