extern crate nalgebra_glm as glm; // Todo: Probably shouldn't live here anymore now that multiple modules rely on this

use crate::math::{Vec4f32, Vec3f32, rotation::Axis};
use self::glm::{Mat4};

// TODO: Make Mat4f32 a generic type eventually
///Wrapped type to interact with nalgebra_glm and hawk_engine's Vec classes
#[derive(Clone, Copy)]
pub struct Mat4f32 {
   pub internal: glm::Mat4,
}

impl Mat4f32 {
    pub fn new(a: &[Vec4f32; 4]) -> Self {
        let internal = glm::mat4(
            a[0].x, a[0].y, a[0].z, a[0].w,
            a[1].x, a[1].y, a[1].z, a[1].w,
            a[2].x, a[2].y, a[2].z, a[2].w,
            a[3].x, a[3].y, a[3].z, a[3].w,
        );

        Mat4f32 { internal }
    }

    pub fn from_value(v: f32) -> Self {
        let internal = glm::mat4(
            v.clone(), v.clone(), v.clone(), v.clone(),
            v.clone(), v.clone(), v.clone(), v.clone(),
            v.clone(), v.clone(), v.clone(), v.clone(),
            v.clone(), v.clone(), v.clone(), v
        );

        Mat4f32 { internal }
    }

    pub fn identity() -> Self {
        let internal = glm::mat4(
            1.0f32, 0.0f32, 0.0f32, 0.0f32,
            0.0f32,1.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 1.0f32
        );

        Mat4f32 { internal }
    }

    pub fn identity_from_value(v: f32) -> Self {
        let internal = glm::mat4(
            v.clone(), 0.0f32, 0.0f32, 0.0f32,
            0.0f32,v.clone(), 0.0f32, 0.0f32,
            0.0f32, 0.0f32, v.clone(), 0.0f32,
            0.0f32, 0.0f32, 0.0f32, v
        );

        Mat4f32 { internal }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let internal: Mat4 = glm::perspective(aspect,glm::radians(&glm::vec1(fov)).data[0], near, far);
        Mat4f32 { internal }
    }

    pub fn look_at(pos: Vec3f32, tar: Vec3f32, up: Vec3f32) -> Self {
        let internal: Mat4 = glm::look_at(
            &glm::vec3(pos.x, pos.y, pos.z),
            &glm::vec3(tar.x, tar.y, tar.z),
            &glm::vec3(up.x, up.y, up.z),
        );

        Mat4f32 { internal }
    }

    pub fn translate(&mut self, trans_vec: Vec3f32) {
        let translation = glm::vec3::<f32>(trans_vec.x, trans_vec.y, trans_vec.z);
        self.internal = glm::translate::<f32>(&self.internal, &translation);
    }

    pub fn rotate(&mut self, deg: f32, axis: Axis) {
        let unit_vec = match axis {
            Axis::X => glm::vec3(1.0f32, 0.0f32, 0.0f32),
            Axis::Y => glm::vec3(0.0f32, 1.0f32, 0.0f32),
            Axis::Z => glm::vec3(0.0f32, 0.0f32, 1.0f32),
            Axis::XY => glm::vec3(1.0f32, 1.0f32, 0.0f32),
            Axis::XZ => glm::vec3(1.0f32, 0.0f32, 1.0f32),
            Axis::YZ => glm::vec3(0.0f32, 1.0f32, 1.0f32),
            Axis::XYZ => glm::vec3(1.0f32, 1.0f32, 1.0f32),
            Axis::ARBITRARY(v) => glm::vec3(v.x, v.y, v.z),
        };

        let rad = glm::radians(&glm::vec1(deg)).data[0];

        self.internal = glm::rotate(&self.internal, rad, &unit_vec);
    }

    pub fn scale(&mut self, scale_vec: Vec3f32) {
        let translation = glm::vec3::<f32>(scale_vec.x, scale_vec.y, scale_vec.z);
        self.internal = glm::scale(&self.internal, &translation);
    }
}