//! 场景管理
//!
//! 类似 Three.js 的场景结构，管理所有精灵并渲染到 buffer

use super::sprite::Sprite;

/// 场景 - 管理所有可渲染对象
pub struct Scene {
    /// 渲染目标 buffer（RGBA 格式）
    buffer: Vec<u8>,
    /// 场景宽度
    width: u32,
    /// 场景高度
    height: u32,
    /// 背景颜色 (RGBA)
    background_color: [u8; 4],
    /// 精灵列表
    sprites: Vec<Box<dyn Sprite>>,
    /// 是否需要重新排序
    needs_sort: bool,
}

impl Scene {
    /// 创建新场景
    ///
    /// # Arguments
    /// * `width` - 场景宽度
    /// * `height` - 场景高度
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize; // RGBA
        Self {
            buffer: vec![0; size],
            width,
            height,
            background_color: [0, 0, 0, 255], // 默认黑色背景
            sprites: Vec::new(),
            needs_sort: false,
        }
    }

    /// 获取场景宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取场景高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 获取 buffer 指针（供 JS 端访问）
    pub fn ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    /// 获取 buffer 长度
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// 检查场景是否为空
    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// 获取 buffer 引用
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// 设置背景颜色
    pub fn set_background_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.background_color = [r, g, b, a];
        self
    }

    /// 设置背景颜色（十六进制）
    pub fn set_background_color_hex(&mut self, color: u32) -> &mut Self {
        self.background_color = [
            ((color >> 24) & 0xFF) as u8,
            ((color >> 16) & 0xFF) as u8,
            ((color >> 8) & 0xFF) as u8,
            (color & 0xFF) as u8,
        ];
        self
    }

    /// 添加精灵到场景
    pub fn add<S: Sprite + 'static>(&mut self, sprite: S) -> u64 {
        let id = sprite.id();
        self.sprites.push(Box::new(sprite));
        self.needs_sort = true;
        id
    }

    /// 移除精灵
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(pos) = self.sprites.iter().position(|s| s.id() == id) {
            self.sprites.remove(pos);
            true
        } else {
            false
        }
    }

    /// 获取精灵数量
    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    /// 获取精灵可变引用（通过 ID）
    pub fn get_sprite_mut(&mut self, id: u64) -> Option<&mut Box<dyn Sprite>> {
        self.sprites.iter_mut().find(|s| s.id() == id)
    }

    /// 清空所有精灵
    pub fn clear(&mut self) {
        self.sprites.clear();
        self.needs_sort = false;
    }

    /// 调整场景尺寸
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.buffer.resize((width * height * 4) as usize, 0);
    }

    /// 按 z-order 排序精灵（稳定排序保持添加顺序）
    fn sort_sprites(&mut self) {
        if self.needs_sort {
            self.sprites.sort_by_key(|s| s.z_order());
            self.needs_sort = false;
        }
    }

    /// 清空 buffer（填充背景色）
    fn clear_buffer(&mut self) {
        let [r, g, b, a] = self.background_color;
        for i in (0..self.buffer.len()).step_by(4) {
            self.buffer[i] = r;
            self.buffer[i + 1] = g;
            self.buffer[i + 2] = b;
            self.buffer[i + 3] = a;
        }
    }

    /// 渲染场景
    ///
    /// 按 z-order 从小到大顺序渲染所有精灵
    pub fn render(&mut self) {
        // 排序精灵
        self.sort_sprites();

        // 清空 buffer
        self.clear_buffer();

        // 按顺序渲染精灵
        let width = self.width;
        let height = self.height;

        // 使用 unsafe 来绕过借用检查，因为我们需要同时访问 sprites 和 buffer
        // 这里是安全的，因为 sprite.render_to 只会修改 buffer，不会修改 sprites
        let buffer_ptr = self.buffer.as_mut_ptr();
        let buffer_len = self.buffer.len();

        for sprite in self.sprites.iter_mut() {
            let buffer_slice = unsafe { std::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
            sprite.render_to(buffer_slice, width, height);
        }
    }

    /// 标记需要重新排序（当精灵 z-order 改变时调用）
    pub fn mark_needs_sort(&mut self) {
        self.needs_sort = true;
    }
}

/// 为 Scene 实现 Debug trait
impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("sprite_count", &self.sprites.len())
            .field("background_color", &self.background_color)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::ImageFormat;
    use crate::scene::sprite::ImageSprite;

    #[test]
    fn test_create_scene() {
        let scene = Scene::new(800, 600);
        assert_eq!(scene.width(), 800);
        assert_eq!(scene.height(), 600);
        assert_eq!(scene.buffer().len(), 800 * 600 * 4);
    }

    #[test]
    fn test_add_remove_sprite() {
        let mut scene = Scene::new(100, 100);
        let sprite = ImageSprite::new(50, 50, ImageFormat::Rgba);
        let id = scene.add(sprite);

        assert_eq!(scene.sprite_count(), 1);

        let removed = scene.remove(id);
        assert!(removed);
        assert_eq!(scene.sprite_count(), 0);
    }

    #[test]
    fn test_z_order_sorting() {
        let mut scene = Scene::new(100, 100);

        let mut sprite1 = ImageSprite::new(10, 10, ImageFormat::Rgba);
        sprite1.set_z_order(10);

        let mut sprite2 = ImageSprite::new(10, 10, ImageFormat::Rgba);
        sprite2.set_z_order(5);

        let mut sprite3 = ImageSprite::new(10, 10, ImageFormat::Rgba);
        sprite3.set_z_order(15);

        scene.add(sprite1);
        scene.add(sprite2);
        scene.add(sprite3);

        // 渲染触发排序
        scene.render();

        // 验证排序结果：z_order 5, 10, 15
        let z_orders: Vec<i32> = scene.sprites.iter().map(|s| s.z_order()).collect();
        assert_eq!(z_orders, vec![5, 10, 15]);
    }

    #[test]
    fn test_background_color() {
        let mut scene = Scene::new(2, 2);
        scene.set_background_color(255, 0, 0, 255);
        scene.render();

        // 验证背景色
        assert_eq!(scene.buffer()[0], 255); // R
        assert_eq!(scene.buffer()[1], 0); // G
        assert_eq!(scene.buffer()[2], 0); // B
        assert_eq!(scene.buffer()[3], 255); // A
    }
}
