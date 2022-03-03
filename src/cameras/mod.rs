//! Core Computer Vision library
//!
//! Provides a collection of CV utils and functions
//! Utilizes aspects of [`opencv`]
//! 
//! [`opencv`]: https://opencv.org/
//! 

mod camera;
mod pinhole_camera;

pub use camera::*;
pub use pinhole_camera::*;
