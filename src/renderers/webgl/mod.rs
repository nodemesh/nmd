extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use self::ws::{connect};
use context;
use renderers;

// struct Payload {
//     cameras: Vec<renderers::Camera>,
//     viewer_transform: Matrix4<f64>
// }

pub struct WebGLRenderer<'a> {
    renderer_context: Option<&'a renderers::RendererContext<'a>>,
    // so: chat-using-web-socket
    // broadcast_rx: mpsc::channel::<Message>
}

impl<'a> WebGLRenderer<'a> {
    pub fn new() -> WebGLRenderer<'a> {
        WebGLRenderer{
            renderer_context: None
        }
    }

    // fn serialize_all() {
    //     self.cameras;
    //     self.viewer;
    // }

    fn terminate() {
        // this is called before removing the backend
        // TODO: join the ws thread
    }

    // different functions that are called when an action happens
    // create camera
    // remove camera

    fn send_payload() {
        // self.renderer_context.cameras;
        // self.renderer_context.matrix_transform;
    }
}

impl<'a> renderers::Renderer<'a> for WebGLRenderer<'a> {

    fn init(
        &mut self,
        options: HashMap<&'a str, &'a str>,
        renderer_context: &'a renderers::RendererContext
    ) {
        self.renderer_context = Some(renderer_context);
        /*connect(self.options.addr, |out| {
            self.ctx;
            // send the viewer location
            // send location, projection for each camera
            // owned by this renderer

            // send the entire graph for now, later we will need to cull objects by their exclusion from the union of the camera frustums
        })*/
    }

    // The following methods are going to broadcast a message
    // to all connected client for now.

    fn set_viewer_transform(&mut self, transform: Matrix4<f32>) {
        // transform
    }

    fn add_camera(&mut self, camera: renderers::Camera) {
        // name, transform, projection
    }

    fn delete_camera_with_name(&mut self, name: &str) {
        // name
    }

    fn set_camera_transform(&mut self, camera_name: &str, transform: Matrix4<f32>) {
        // name, transform
    }

    fn set_camera_projection(&mut self, camera_name: &str, projection: Matrix4<f32>) {
        // name, projection
    }
}
