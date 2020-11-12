use crate::math::vector::{Vec2};

pub struct Rect2D<T>
where
    T: std::clone::Clone,
{
    position: Vec2<T>,
    size: Vec2<T>,
}

impl<T> Rect2D<T>
where
    T: std::clone::Clone,
{
    pub fn new(position: Vec2<T>, size: Vec2<T>) -> Self {
        Rect2D { position, size }
    }

    pub fn pos(&self) -> Vec2<T> {
        self.position.clone()
    }

    pub fn size(&self) -> Vec2<T> {
        self.size.clone()
    }
}

// pub struct Cube3D<T>
// where
//     T: std::clone::Clone,
// {
//     position: Vec3<T>,
//     size: Vec3<T>,
// }

// pub struct Triangle2D<T>
// where
//     T: std::clone::Clone,
// {
//
// }
//
// pub struct Pyramid3D<T>
// where
//     T: std::clone::Clone,
// {
//
// }