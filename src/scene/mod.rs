//! 场景管理模块
//!
//! 提供类似 Three.js 的场景管理结构，支持精灵图的渲染和变换。

mod scene;
pub mod sprite;

pub use scene::Scene;
pub use sprite::{ImageSprite, Sprite};
