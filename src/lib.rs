#[allow(dead_code)]
pub mod cameras;

#[cfg(test)]
mod tests {
    use super::*;

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
}
