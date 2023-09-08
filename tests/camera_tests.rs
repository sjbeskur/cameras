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

    let k = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0,
    );
    let default = cameras::PinholeCamera::new(k);
    let default2 = cameras::PinholeCamera::new(k);

    assert_eq!(default, default2);
}

#[test]
fn camera_baseline() {
    let k = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0,
    );

    let nfov1 = cameras::PinholeCamera::new(k);
    let nfov2 = cameras::PinholeCamera::new(k);

    assert_eq!(nfov2, nfov2);
}


#[test]
fn perspective_project() {
    let k = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0,
    );

    let nfov1 = cameras::PinholeCamera::new(k);
    let world_coord = Point3::new(30.0, 40.0, 50.0);
    let image_coord = nfov1.project(&world_coord);

    println!("{:?}", image_coord);
}

#[test]
fn perspective_unproject() {
    let k = Matrix3::new(
        31818.1818, 0.0, 1295.5,
        0.0, 31818.1818, 971.5,
        0.0, 0.0, 1.0,
    );

    let nfov1 = cameras::PinholeCamera::new(k);
    let uv = Point2::new(20386.408, 26426.045);
    let world = nfov1.unproject(&uv);

    println!("{:?}", world);
}
