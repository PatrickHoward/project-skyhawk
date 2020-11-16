extern crate glm;

use glm::{Matrix4, Vector4, Vector3};

use crate::math::vector::{Vec4, Vec3};

pub struct Mat4<T>
where T: glm::BaseFloat
{
    internal: glm::Matrix4<T>,
}

impl<T> Mat4<T>
where
    T: glm::BaseFloat
{
    pub fn new(a: &[Vec4<T>; 4]) -> Self {
        let c1 = glm::Vector4::new(a[0].x, a[0].y, a[0].z, a[0].w);
        let c2 = glm::Vector4::new(a[1].x, a[1].y, a[1].z, a[1].w);
        let c3 = glm::Vector4::new(a[2].x, a[2].y, a[2].z, a[2].w);
        let c4 = glm::Vector4::new(a[3].x, a[3].y, a[3].z, a[3].w);

        let internal = glm::Matrix4::<T>::new(c1, c2, c3, c4);

        Mat4 { internal }
    }

    pub fn from_value(v: T) -> Self {
        let c1 = glm::Vector4::new(v, v, v, v);
        let c2 = glm::Vector4::new(v, v, v, v);
        let c3 = glm::Vector4::new(v, v, v, v);
        let c4 = glm::Vector4::new(v, v, v, v);

        let internal = glm::Matrix4::<T>::new(c1, c2, c3, c4);

        Mat4 { internal }
    }

    pub fn translate(&mut self, trans_vec: Vec3<T>) {
        let trans_vec = Vector3::<T>::new(trans_vec.x, trans_vec.y, trans_vec.z);
        self.internal = glm::ext::translate(&self.internal, trans_vec);
    }
}