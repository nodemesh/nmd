extern crate nalgebra as na;

// TODO: implement display renderer
// pub mod display;
pub mod webgl;

use self::na::*;
use std::collections::HashMap;

pub trait Renderer<'a> {
    fn init(
        &mut self,
        options: HashMap<&'a str, &'a str>,
        renderer_context: &'a RendererContext
    );
    fn set_viewer_transform(&mut self, transform: Matrix4<f32>);
    fn add_camera(&mut self, camera: Camera);
    fn delete_camera_with_name(&mut self, name: &str);
    fn set_camera_transform(&mut self, camera_name: &str, transform: Matrix4<f32>);
    fn set_camera_projection(&mut self, camera_name: &str, projection: Matrix4<f32>);
}

pub struct Camera {
    name: String,
    transform: Matrix4<f32>,
    projection: Matrix4<f32>
}

pub struct RendererContext<'a> {
    cameras: HashMap<String, Camera>,
    viewer_transform: Matrix4<f32>,
    renderer: Box<Renderer<'a>>
}

impl<'a> RendererContext<'a> {
    pub fn set_viewer_transform(&self, transform: Matrix4<f32>) {
        self.viewer_transform = transform;
        self.renderer.set_viewer_transform(transform);
    }

    pub fn add_camera(&self, camera: Camera) {
        self.cameras.insert(camera.name, camera);
        self.renderer.add_camera(camera);
    }

    pub fn delete_camera_with_name(&self, name: &str) {
        self.cameras.remove(name);
        self.renderer.delete_camera_with_name(name);
    }

    pub fn set_camera_transform(
        &mut self, camera_name: &str, transform: Matrix4<f32>
    ) {
        let camera = self.cameras.get(camera_name).unwrap();
        camera.transform = transform;
        self.renderer.set_camera_transform(camera_name, transform);
    }

    pub fn set_camera_projection(
        &mut self, camera_name: &str, projection: Matrix4<f32>
    ) {
        let camera = self.cameras.get(camera_name).unwrap();
        camera.projection = projection;
        self.renderer.set_camera_projection(camera_name, projection);
    }
}
