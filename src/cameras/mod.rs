//! Core Computer Vision library
//!
//! Provides a collection of CV utils and functions
//! Utilizes aspects of [`opencv`]
//! 
//! [`opencv`]: https://opencv.org/
//! 

mod camera;
mod perspective_projection;

pub use camera::*;
pub use perspective_projection::*;
