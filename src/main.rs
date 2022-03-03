use camera_models::cameras::{Camera, PinholeCamera, Projection};

use nalgebra::{Matrix3x4, Matrix3 };

fn main() {
    let cam = Camera::default();


    let mut Kmtx = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0
    );

    let mut T1 = Matrix3x4::new(
        1.0, 0.0, 0.0, 0.069266, 
        0.0, 1.0, 0.0, -0.129384, 
        1.0, 0.0, 1.0, 0.026577,
    );

    let cam0 =Camera::new(Kmtx);

    let pinhole_cameras: Vec<&dyn PinholeCamera> = vec![&cam, &cam0];
    for p in pinhole_cameras {
        println!("{}", p);
    }

    cam0.project();
}



#[test]
fn matrix_ops(){
    let mut mtx = Matrix3x4::new(
        1.0, 0.0, 0.0, 0.069266, 
        0.0, 1.0, 0.0, -0.129384, 
        1.0, 0.0, 1.0, 0.026577,
    );

    let (r,c) = mtx.iamax_full();
    println!("{},{}",r,c);
    mtx.swap_rows(0,1);
    println!("{}",mtx);
    mtx.swap_rows(0,1);
    println!("{}",mtx);

    mtx *= 3.0;
    println!("{}",mtx);
    
    println!("{}",mtx);
    println!("{}",mtx.row(0) * 100.0);

}
