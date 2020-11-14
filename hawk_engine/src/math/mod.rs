pub mod primitive;
pub mod vector;
pub mod matrix;

use primitive::{Rect2D};
use vector::{Vec2, Vec3, Vec4};

// Aliased types for Vector structs
pub type Vec2f32 = Vec2<f32>;
pub type Vec2i32 = Vec2<i32>;

pub type Vec3f32 = Vec3<f32>;
pub type Vec3i32 = Vec3<i32>;

pub type Vec4f32 = Vec4<f32>;
pub type Vec4i32 = Vec4<i32>;

// Aliased types for "primitives"
pub type Rect2Df32 = Rect2D<f32>;
pub type Rect2Di32 = Rect2D<i32>;
//
// pub type Cube3Df32 = Cube3D<f32>;
// pub type Cube3Di32 = Cube3D<i32>;
