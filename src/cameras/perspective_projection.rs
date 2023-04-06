use nalgebra::{ Point2, Point3 };
use super::PinholeCamera;

pub trait PerspectiveProjection {
    fn project(&self, world_point: Point3<f32>) -> Point2<f32>;
    fn unproject(&self, uv: Point2<f32>) -> Point3<f32>;
}

impl PerspectiveProjection for PinholeCamera {

    /// Double check the math 
    fn project(&self, world_point: Point3<f32>) -> Point2<f32>{
        let rslt = &self.intrinsic_mtx * world_point;
        Point2::new(rslt.x /rslt.z, rslt.y/rslt.z)
    }

    /// Double chedk the math 
    fn unproject(&self, uv: Point2<f32>) -> Point3<f32>{
        let kinv = &self.intrinsic_mtx.try_inverse().unwrap();
        let homogeneous = uv.to_homogeneous();
        let coords = kinv * homogeneous;
        Point3::from(coords)
    }
}