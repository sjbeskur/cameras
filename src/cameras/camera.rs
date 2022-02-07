use nalgebra::{Matrix3x4, Point2};
use std::fmt;

#[derive(Debug)]
pub struct Camera {
    pub(crate) intrinsic_mtx: Matrix3x4<f32>,
}

impl Camera {
    pub fn new(intrinsic_mtx: Matrix3x4<f32>) -> Self {
        Self {
            intrinsic_mtx: intrinsic_mtx,
        }
    }

    pub fn get_type() -> String {
        "default".to_string()
    }

    pub fn get_intrinsics(&self) -> Matrix3x4<f32> {
        self.intrinsic_mtx
    }

    pub fn skew(&self) -> f32 {
        self.intrinsic_mtx[(0, 1)]
    }

    pub fn focal_length(&self) -> Point2<f32> {
        Point2::new(self.intrinsic_mtx[(0, 0)], self.intrinsic_mtx[(1, 1)])
    }

    pub fn optical_center(&self) -> Point2<f32> {
        Point2::new(self.intrinsic_mtx[(0, 3)], self.intrinsic_mtx[(1, 3)])
    }

    pub fn principal_point(&self) -> Point2<f32> {
        self.optical_center()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            intrinsic_mtx: Matrix3x4::new(
                1.0, 0.0, 0.0, 0.069266, 0.0, 1.0, 0.0, -0.129384, 1.0, 0.0, 1.0, 0.026577,
            ),
        }
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.intrinsic_mtx)
    }
}

impl PartialEq for Camera {
    fn eq(&self, other: &Camera) -> bool {
        self.intrinsic_mtx == other.intrinsic_mtx
    }
}

pub trait Projection {
    fn project(&self);
    fn unproject(&self);
}

impl Projection for Camera {
    fn project(&self) {
        println!("Project NOT YET implemented :D")
    }

    fn unproject(&self) {
        println!("Unproject NOT YET implemented :D")
    }
}
