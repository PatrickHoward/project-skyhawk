pub mod matrix;
pub mod primitive;
pub mod rotation;
pub mod vector;

use primitive::Rect2D;

use vector::{Vec2T, Vec3T, Vec4T};

// Aliased types for Vector structs
pub type Vec2 = Vec2T<f32>;
pub type Vec2i32 = Vec2T<i32>;

pub type Vec3 = Vec3T<f32>;
pub type Vec3i32 = Vec3T<i32>;

pub type Vec4 = Vec4T<f32>;
pub type Vec4i32 = Vec4T<i32>;

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
