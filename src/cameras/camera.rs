use nalgebra::{Matrix3x4, Point2, Point3, Matrix3, Vector3};
use std::fmt;

pub trait PinholeModel { }

#[derive(Debug)]
pub struct PinholeCamera {
    pub(crate) intrinsic_mtx: Matrix3<f32>,
    //pub(crate) extrinsic_mtx: Matrix3x4<f32>,
}

impl PinholeCamera {
    pub fn new(intrinsic_mtx: Matrix3<f32>) -> Self {
        Self {
            intrinsic_mtx: intrinsic_mtx,
        }
    }

    pub fn get_type() -> String {
        "default".to_string()        
    }

    pub fn get_intrinsics(&self) -> Matrix3<f32> {
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

impl Default for PinholeCamera {
    fn default() -> Self {
        Self {
            intrinsic_mtx: Matrix3::new(
                31818.1818, 0.0, 1295.5,
                0.0, 31818.1818, 971.5,
                0.0, 0.0, 1.0,
            ),

            // extrinsic_mtx: Matrix4x4::new(
            //     1, 0, 0,  0.069266 ,
            //     0, 1, 0, -0.129384 ,
            //     0, 0, 1, -0.026577 ,
            //     0, 0, 0, 1
            // )
                     

        }
    }
}

impl fmt::Display for PinholeCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.intrinsic_mtx)
    }
}

impl PartialEq for PinholeCamera {
    fn eq(&self, other: &PinholeCamera) -> bool {
        self.intrinsic_mtx == other.intrinsic_mtx
    }
}

pub trait Perspective {
    fn project(&self, world_point: Point3<f32>) -> Point2<f32>;
    fn unproject(&self, uv: Point2<f32>) -> Point3<f32>;
}

impl Perspective for PinholeCamera {

    /// Fix math 
    fn project(&self, world_point: Point3<f32>) -> Point2<f32>{
        let rslt = &self.intrinsic_mtx * world_point;
        Point2::new(rslt[0]/rslt[2], rslt[1]/rslt[2])
    }

    /// Fix math 
    fn unproject(&self, uv: Point2<f32>) -> Point3<f32>{
        let kinv = &self.intrinsic_mtx.try_inverse().unwrap();
        let homogeneous = uv.to_homogeneous();
        let coords = kinv * homogeneous;
        Point3::from(coords)
    }
}
