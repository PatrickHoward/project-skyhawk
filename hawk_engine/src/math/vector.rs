use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2<T>
where
    T: std::clone::Clone,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: std::clone::Clone,
{
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn as_tuple(&self) -> (T, T) { (self.x.clone(), self.y.clone()) }
}

impl Vec2<f32> {
    pub fn unit() -> Self {
        Vec2::<f32>::new(1.0f32, 1.0f32)
    }

    pub fn zero() -> Self {
        Vec2::<f32>::new(0.0f32, 0.0f32)
    }
}

impl From<(f32, f32)> for Vec2<f32> {
    fn from(data: (f32, f32)) -> Self {
        Vec2::<f32>::new(data.0, data.1)
    }
}

impl Mul for Vec2<f32> {
    type Output = Vec2<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;

        Vec2::<f32>::new(x, y)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec3<T>
where
    T: std::clone::Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

// Todo: Make conversion traits for mapping colors to Vec3f32
// pub trait AsVec3<T> {
//     fn as_vec(self) -> Vec3<T>;
// }

impl<T> Vec3<T>
where
    T: std::clone::Clone,
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn from_value(v: T) -> Vec3<T> {
        let value = v;

        Vec3 {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
        }
    }

    pub fn as_tuple(&self) -> (T, T, T) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl Vec3<f32> {
    pub fn unit() -> Self {
        Vec3::<f32>::new(1.0f32, 1.0f32, 1.0f32)
    }

    pub fn zero() -> Self { Vec3::<f32>::new(0.0f32, 0.0f32, 0.0f32) }
}

impl From<(f32, f32, f32)> for Vec3<f32> {
    fn from(data: (f32, f32, f32)) -> Self {
        Vec3::<f32>::new(data.0, data.1, data.2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
where T: std::clone::Clone {
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x, y, z, w }
    }

    pub fn from_value(v: T) -> Vec4<T> {
        let value = v;

        Vec4 {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
            w: value.clone(),
        }
    }
}

impl Vec4<f32> {
    pub fn unit() -> Self {
        Vec4::<f32>::new(1.0f32, 1.0f32, 1.0f32, 0.0f32)
    }

    pub fn zero() -> Self {
        Vec4::<f32>::new(0.0f32, 0.0f32, 0.0f32, 0.0f32)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4<f32> {
    fn from(data: (f32, f32, f32, f32)) -> Self {
        Vec4::<f32>::new(data.0, data.1, data.2, data.3)
    }
}
