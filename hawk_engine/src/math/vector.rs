use std::{
    clone::Clone,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vec2T<T>
where
    T: Clone,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2T<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar.clone(),
            y: self.y * scalar,
        }
    }
}

impl<T> Add for Vec2T<T>
where
    T: Clone + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2T<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vec2T<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2T<f32> {
    pub fn unit() -> Self {
        Vec2T::<f32>::new(1.0f32, 1.0f32)
    }
    pub fn zero() -> Self {
        Vec2T::<f32>::new(0.0f32, 0.0f32)
    }
}

impl From<(f32, f32)> for Vec2T<f32> {
    fn from(data: (f32, f32)) -> Self {
        Vec2T::<f32>::new(data.0, data.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vec3T<T>
where
    T: std::clone::Clone,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3T<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Vec3T<T> {
        Vec3T { x, y, z }
    }

    pub fn from_value(v: T) -> Vec3T<T> {
        Vec3T {
            x: v.clone(),
            y: v.clone(),
            z: v,
        }
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3T {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z * rhs.x.clone() - self.x.clone() * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar.clone(),
            y: self.y * scalar.clone(),
            z: self.z * scalar,
        }
    }

    pub fn into_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T> Add for Vec3T<T>
where
    T: Clone + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3T<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Neg for Vec3T<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3T<f32> {
    pub fn unit() -> Self {
        Vec3T::<f32>::new(1.0f32, 1.0f32, 1.0f32)
    }
    pub fn zero() -> Self {
        Vec3T::<f32>::new(0.0f32, 0.0f32, 0.0f32)
    }

    // TODO: Make a generic normalize
    pub fn normalize(self) -> Result<Self, ()> {
        let mag = (self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone())
        .sqrt();

        if mag.is_nan() {
            return Err(());
        }

        Ok(Self {
            x: self.x / mag.clone(),
            y: self.y / mag.clone(),
            z: self.z / mag,
        })
    }
}

impl From<(f32, f32, f32)> for Vec3T<f32> {
    fn from(data: (f32, f32, f32)) -> Self {
        Vec3T::<f32>::new(data.0, data.1, data.2)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vec4T<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4T<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T>,
{
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

    pub fn mul_scalar(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar.clone(),
            y: self.y * scalar.clone(),
            z: self.z * scalar.clone(),
            w: self.w * scalar,
        }
    }

    pub fn into_tuple(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T> Add for Vec4T<T>
where
    T: Clone + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vec4T<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Neg for Vec4T<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Vec4T<f32> {
    pub fn unit() -> Self {
        Vec4T::<f32>::from_value(1.0f32)
    }
    pub fn zero() -> Self {
        Vec4T::<f32>::from_value(0.0f32)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4T<f32> {
    fn from(data: (f32, f32, f32, f32)) -> Self {
        Vec4T::<f32>::new(data.0, data.1, data.2, data.3)
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::math::vector::*;

    #[test]
    fn cross_product() {
        let vec_a = Vec3T::<f32>::new(3.0f32, 2.0f32, 1.0f32);
        let vec_b = Vec3T::<f32>::new(1.0f32, 2.0f32, 3.0f32);

        assert_eq!(
            vec_a.cross(vec_b),
            Vec3T::<f32>::new(4.0f32, -8.0f32, 4.0f32)
        );
    }

    #[test]
    fn dot_product_for_Vec3() {
        let vec_a = Vec3T::<f32>::new(3.0f32, 2.0f32, 1.0f32);
        let vec_b = Vec3T::<f32>::new(1.0f32, 2.0f32, 3.0f32);

        assert_eq!(vec_a.dot(vec_b), 10.0f32);
    }

    #[test]
    fn normalize_Vec3() {
        let vec_a = Vec3T::<f32>::new(3.0f32, 2.0f32, 1.0f32);
        let norm_vec_a = Vec3T::<f32>::new(0.8017837f32, 0.5345225f32, 0.26726124f32);

        assert_eq!(vec_a.normalize().unwrap(), norm_vec_a);
    }
}
