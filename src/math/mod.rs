//! 数学库 - 2D 变换矩阵计算
//!
//! 提供高效的 2D 变换操作，包括旋转、缩放、平移。
//! 设计为可扩展的接口，便于后续升级为 SIMD 或其他优化实现。

mod matrix;
mod transform;
mod vec2;

pub use matrix::{Matrix3x3, MatrixOperations};
pub use transform::Transform2D;
pub use vec2::Vec2;
