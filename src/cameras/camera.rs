use nalgebra::{Matrix3x4, Point2, Point3, Matrix3, Vector3};
use std::fmt;

pub trait PinholeModel { }

#[derive(Debug)]
pub struct PinholeCamera {
    pub(crate) k: Matrix3<f32>,
    pub(crate) kinv: Matrix3<f32>,

    //pub(crate) extrinsic_mtx: Matrix3x4<f32>,
}

impl PinholeCamera {
    pub fn new(intrinsics: Matrix3<f32>) -> Self {
        Self {
            k: intrinsics,
            kinv: intrinsics.try_inverse().unwrap(),
        }
    }

    pub fn get_type() -> String {
        "default".to_string()        
    }

    pub fn get_intrinsics(&self) -> Matrix3<f32> {
        self.k
    }

    pub fn skew(&self) -> f32 {
        self.k[(0, 1)]
    }

    pub fn focal_length(&self) -> Point2<f32> {
        Point2::new(self.k[(0, 0)], self.k[(1, 1)])
    }

    pub fn optical_center(&self) -> Point2<f32> {
        Point2::new(self.k[(0, 3)], self.k[(1, 3)])
    }

    pub fn principal_point(&self) -> Point2<f32> {
        self.optical_center()
    }
}




impl fmt::Display for PinholeCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.k)
    }
}

impl PartialEq for PinholeCamera {
    fn eq(&self, other: &PinholeCamera) -> bool {
        self.k == other.k
    }
}


