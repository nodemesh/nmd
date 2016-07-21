extern crate nalgebra as na;

pub mod webgl;

use self::na::*;
use std::collections::HashMap;
use std::sync::{RwLockWriteGuard};

pub struct Camera {
    pub name: String,
    pub transform: Matrix4<f32>,
    pub projection: Matrix4<f32>
}

pub struct Viewer {
    pub cameras: HashMap<String, Camera>,
    pub transform: Matrix4<f32>,
}

pub trait Renderer {
    fn viewer(&mut self) -> RwLockWriteGuard<Viewer>;
    fn update_viewer_transform(&mut self, transform: Matrix4<f32>);
    fn add_camera(&mut self, camera: &Camera);
    fn delete_camera_with_name(&mut self, name: &str);
    fn update_camera_transform(
        &mut self, camera_name: &str, transform: Matrix4<f32>
    );
    fn update_camera_projection(
        &mut self, camera_name: &str, projection: Matrix4<f32>
    );
}
