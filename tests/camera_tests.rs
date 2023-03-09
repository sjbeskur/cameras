use camera_models::*;

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
