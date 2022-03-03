use super::Camera;
use std::fmt; // super refers to the parent scope
              // or
              //use crate::cameras::Camera;

pub trait PinholeCamera {
    fn get_extrinsic(&self) -> f32;
}

impl PinholeCamera for Camera {
    fn get_extrinsic(&self) -> f32 {
        return self.intrinsic_mtx[(0, 2)];
    }
}

impl fmt::Display for dyn PinholeCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_extrinsic())
    }
}
