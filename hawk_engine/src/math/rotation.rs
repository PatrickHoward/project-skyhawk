extern crate nalgebra_glm as glm;

use crate::math::Vec3f32;

pub enum Axis {
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ,
    ARBITRARY(Vec3f32),
}

pub fn to_rad(deg: f32) -> f32 {
    glm::radians(&glm::vec1(deg)).data[0]
}
