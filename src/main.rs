use camera_models::cameras;
use camera_models::cameras::{PinholeCamera, Projection};

use nalgebra::Matrix3x4;

fn main() {
    // Parses and executes the target subcommand (e.g filter, centriod)
    let cam = cameras::Camera::default();

    let mut mtx = Matrix3x4::new(
        1.0, 0.0, 0.0, 0.069266, 
        0.0, 1.0, 0.0, -0.129384, 
        1.0, 0.0, 1.0, 0.026577,
    );

    let cam0 = cameras::Camera::new(mtx);

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
