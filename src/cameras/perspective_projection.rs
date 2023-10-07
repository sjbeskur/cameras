use nalgebra::{ Point2, Point3, SMatrix };
use super::PinholeCamera;

pub trait PerspectiveProjection {
    fn project(&self, world_point: &SMatrix<f32, 3,1>) -> SMatrix<f32, 2,1>;
    fn project_point(&self, world_point: &Point3<f32>) -> Point2<f32>;
    fn unproject(&self, uv: &Point2<f32>) -> Point3<f32>;
}
pub trait LinearizeProjection {
    fn project_linearize(&self, world_point: &Point3<f32>, jacobian: &mut SMatrix<f32, 2,3>) -> Point2<f32>;
    //fn unproject(&self, uv: &Point2<f32>) -> Point3<f32>;
}



impl PerspectiveProjection for PinholeCamera {

    /// Double check the math 
    fn project(&self, world_point: &SMatrix<f32, 3,1>) -> SMatrix<f32,2,1>{
        let rslt = &self.k * world_point;
        SMatrix::<f32, 2,1>::new(rslt.x /rslt.z, rslt.y/rslt.z)
    }

    fn project_point(&self, world_point: &Point3<f32>) -> Point2<f32>{
        let rslt = &self.k * world_point;
        Point2::new(rslt.x /rslt.z, rslt.y/rslt.z)
    }

    /// Double check the math 
    fn unproject(&self, uv: &Point2<f32>) -> Point3<f32>{
        //let kinv = &self.k.try_inverse().unwrap();  // panics if self is not a square matrix 
        let homogeneous = uv.to_homogeneous();
        let coords = &self.kinv * homogeneous;
        Point3::from(coords)
    }
}

type Jacobian2x3 = SMatrix<f32, 2, 3>;
type Matrix2x1 = SMatrix<f32, 2,1>;
impl LinearizeProjection for PinholeCamera {

    fn project_linearize(&self, world_point: &Point3<f32>, jacobian: &mut SMatrix<f32, 2,3>) -> Point2<f32>{
        let rk = self.k * world_point;
        let s: f32 = 1.0 / rk.z;
        let uv = Matrix2x1::new(rk.x * s, rk.y * s);
        let juv_rk = Jacobian2x3::new(s,   0.0, -uv.x * s,
                                               0.0,  s , -uv.y * s);
        let tmp = juv_rk * self.k;
        jacobian.clone_from(&tmp);
        uv.into()
    }
}

#[test]
fn test_project_unproject(){
    use nalgebra::Matrix3;

    let k = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0,
    );
    println!("k = :{}", k);

    let inv = k.try_inverse().unwrap();
    println!("inverse k = :{}", inv);

    let cam = PinholeCamera::new(k);
    println!("camera: {}", cam);

    let world_point  = Point3::new(5.0,5.0,5.0);
    let projected_point = cam.project(&world_point);

    println!("projected point: {}", projected_point);

    let w2 = cam.unproject(&projected_point);
    assert_eq!(w2, world_point);

}