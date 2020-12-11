pub mod primitive;
pub mod vector;
pub mod matrix;
pub mod rotation;

use primitive::{Rect2D};

use vector::{Vec2, Vec3, Vec4};

// Aliased types for Vector structs
pub type Vec2f32 = Vec2<f32>;
pub type Vec2i32 = Vec2<i32>;

pub type Vec3f32 = Vec3<f32>;
pub type Vec3i32 = Vec3<i32>;

pub type Vec4f32 = Vec4<f32>;
pub type Vec4i32 = Vec4<i32>;

// Aliased types for Matrix structs
// pub type Mat4f32 = Mat4f32;

// Aliased types for "primitives"
pub type Rect2Df32 = Rect2D<f32>;
pub type Rect2Di32 = Rect2D<i32>;
//
// pub type Cube3Df32 = Cube3D<f32>;
// pub type Cube3Di32 = Cube3D<i32>;

// TODO: Is this really something I should do to enforce this as a numeric primative?
// trait Numeric {}
//
// impl Numeric for f32 {}
// impl Numeric for f64 {}
// impl Numeric for i8 {}
// impl Numeric for i32 {}
// impl Numeric for i64 {}
// impl Numeric for i128 {}
// impl Numeric for u8 {}
// impl Numeric for u32 {}
// impl Numeric for u64 {}
// impl Numeric for u128 {}
