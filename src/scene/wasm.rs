//! WebAssembly bindings for Scene and Sprite
//!
//! 提供 JavaScript 可调用的场景和精灵接口

use wasm_bindgen::prelude::*;

use crate::scene::{ImageSprite, Scene};

/// WASM Scene 包装器
#[wasm_bindgen]
pub struct WasmScene {
    scene: Scene,
    /// 保存精灵的 ID 列表（用于访问）
    sprite_ids: Vec<u64>,
}

#[wasm_bindgen]
impl WasmScene {
    /// 创建新场景
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> WasmScene {
        WasmScene {
            scene: Scene::new(width, height),
            sprite_ids: Vec::new(),
        }
    }

    /// 获取场景宽度
    pub fn width(&self) -> u32 {
        self.scene.width()
    }

    /// 获取场景高度
    pub fn height(&self) -> u32 {
        self.scene.height()
    }

    /// 获取 buffer 指针（供 JS 端访问）
    pub fn ptr(&self) -> *const u8 {
        self.scene.ptr()
    }

    /// 获取 buffer 长度
    pub fn len(&self) -> usize {
        self.scene.len()
    }

    /// 设置背景颜色
    pub fn set_background_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.scene.set_background_color(r, g, b, a);
    }

    /// 添加长方形精灵
    ///
    /// 返回精灵索引（用于后续操作）
    pub fn add_rectangle(&mut self, width: u32, height: u32, r: u8, g: u8, b: u8, a: u8) -> usize {
        let sprite = ImageSprite::create_rectangle(width, height, r, g, b, a);
        let id = self.scene.add(sprite);
        self.sprite_ids.push(id);
        self.sprite_ids.len() - 1
    }

    /// 设置精灵位置
    pub fn set_sprite_position(&mut self, index: usize, x: f32, y: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().set_position(x, y);
            }
        }
    }

    /// 设置精灵旋转（角度）
    pub fn set_sprite_rotation(&mut self, index: usize, degrees: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().set_rotation_degrees(degrees);
            }
        }
    }

    /// 设置精灵缩放
    pub fn set_sprite_scale(&mut self, index: usize, sx: f32, sy: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().set_scale(sx, sy);
            }
        }
    }

    /// 设置精灵均匀缩放
    pub fn set_sprite_uniform_scale(&mut self, index: usize, scale: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().set_uniform_scale(scale);
            }
        }
    }

    /// 设置精灵锚点
    pub fn set_sprite_anchor(&mut self, index: usize, ax: f32, ay: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().set_anchor(ax, ay);
            }
        }
    }

    /// 平移精灵
    pub fn translate_sprite(&mut self, index: usize, dx: f32, dy: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().translate(dx, dy);
            }
        }
    }

    /// 旋转精灵（增量，角度）
    pub fn rotate_sprite(&mut self, index: usize, degrees: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().rotate_degrees(degrees);
            }
        }
    }

    /// 缩放精灵（乘法）
    pub fn scale_sprite_by(&mut self, index: usize, sx: f32, sy: f32) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                sprite.transform_mut().scale_by(sx, sy);
            }
        }
    }

    /// 重置精灵变换
    pub fn reset_sprite_transform(&mut self, index: usize) {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                let transform = sprite.transform_mut();
                transform.set_position(0.0, 0.0);
                transform.set_rotation(0.0);
                transform.set_scale(1.0, 1.0);
            }
        }
    }

    /// 获取精灵位置 X
    pub fn get_sprite_position_x(&mut self, index: usize) -> f32 {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                return sprite.transform().position.x;
            }
        }
        0.0
    }

    /// 获取精灵位置 Y
    pub fn get_sprite_position_y(&mut self, index: usize) -> f32 {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                return sprite.transform().position.y;
            }
        }
        0.0
    }

    /// 获取精灵旋转角度（弧度）
    pub fn get_sprite_rotation(&mut self, index: usize) -> f32 {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                return sprite.transform().rotation;
            }
        }
        0.0
    }

    /// 获取精灵缩放 X
    pub fn get_sprite_scale_x(&mut self, index: usize) -> f32 {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                return sprite.transform().scale.x;
            }
        }
        1.0
    }

    /// 获取精灵缩放 Y
    pub fn get_sprite_scale_y(&mut self, index: usize) -> f32 {
        if index < self.sprite_ids.len() {
            let id = self.sprite_ids[index];
            if let Some(sprite) = self.scene.get_sprite_mut(id) {
                return sprite.transform().scale.y;
            }
        }
        1.0
    }

    /// 渲染场景
    pub fn render(&mut self) {
        self.scene.render();
    }

    /// 清空场景
    pub fn clear(&mut self) {
        self.scene.clear();
        self.sprite_ids.clear();
    }

    /// 获取精灵数量
    pub fn sprite_count(&self) -> usize {
        self.scene.sprite_count()
    }
}
