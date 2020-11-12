use std::{ops::{Add, Mul}, clone::Clone};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn into_tuple(self) -> (T, T) { (self.x, self.y) }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x +
        self.y * rhs.y
    }

    pub fn cross(self, _rhs: Self) -> Self {
        unimplemented!();
    }

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self { x: self.x * scalar.clone(), y: self.y * scalar}
    }

}

impl<T> Add for Vec2<T>
where T: Clone + Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y}
    }
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

impl<T> Vec3<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn from_value(v: T) -> Vec3<T> {
        Vec3 {
            x: v.clone(),
            y: v.clone(),
            z: v,
        }
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, _rhs: Self) -> Self {
        unimplemented!();
    }

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self { x: self.x * scalar.clone(), y: self.y * scalar.clone(), z: self.z * scalar}
    }

    pub fn into_tuple(self) -> (T, T, T) {(self.x, self.y, self.z)}
}

impl<T> Add for Vec3<T>
    where T: Clone + Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
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
where T: Clone + Mul<Output = T> + Add<Output = T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_value(v: T) -> Self {
        Self {
            x: v.clone(),
            y: v.clone(),
            z: v.clone(),
            w: v,
        }
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(self, _rhs: Self) -> Self {
        unimplemented!();
    }

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self { x: self.x * scalar.clone(), y: self.y * scalar.clone(), z: self.z * scalar.clone(), w: self.w * scalar}
    }

    pub fn into_tuple(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T> Add for Vec4<T>
    where T: Clone + Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Vec4<f32> {
    pub fn unit() -> Self { Vec4::<f32>::from_value(1.0f32) }
    pub fn zero() -> Self { Vec4::<f32>::from_value(0.0f32) }
}

impl From<(f32, f32, f32, f32)> for Vec4<f32> {
    fn from(data: (f32, f32, f32, f32)) -> Self {
        Vec4::<f32>::new(data.0, data.1, data.2, data.3)
    }
}
