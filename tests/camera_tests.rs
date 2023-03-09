use camera_models::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn camera_intrinsics() {
    let default = cameras::Camera::default();
    let default2 = cameras::Camera::default();

    assert_eq!(default, default2);
}

#[test]
fn camera_baseline() {
    let nfov1 = cameras::Camera::default();
    let nfov2 = cameras::Camera::default();

    assert_eq!(nfov2, nfov2);
}
