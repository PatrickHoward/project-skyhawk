use crate::math::Vec3f32;

pub enum Axis {
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ,
    ARBITRARY(Vec3f32)
}