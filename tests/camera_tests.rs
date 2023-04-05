use camera_models::*;
use camera_models::cameras::*;
use nalgebra::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn camera_intrinsics() {
    let default = cameras::PinholeCamera::default();
    let default2 = cameras::PinholeCamera::default();

    assert_eq!(default, default2);
}

#[test]
fn camera_baseline() {
    let nfov1 = cameras::PinholeCamera::default();
    let nfov2 = cameras::PinholeCamera::default();

    assert_eq!(nfov2, nfov2);
}


#[test]
fn perspective_Project() {
    let nfov1 = cameras::PinholeCamera::default();
    let nfov2 = cameras::PinholeCamera::default();

    let world_coord = Point3::new(30.0, 40.0, 50.0);
    let image_coord = nfov1.project(world_coord);

    println!("{:?}", image_coord);

    assert_eq!(nfov2, nfov2);
}
