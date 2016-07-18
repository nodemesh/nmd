extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use std::rc::Rc;

// use self::ws::{connect};
// use context;
use renderers;

// struct Payload {
//     cameras: Vec<renderers::Camera>,
//     viewer_transform: Matrix4<f64>
// }

pub struct WebGLRenderer {
    renderer_context: Option<Rc<renderers::RendererContext>>,
    // so: chat-using-web-socket
    // broadcast_rx: mpsc::channel::<Message>
}

impl WebGLRenderer {
    pub fn new() -> WebGLRenderer {
        WebGLRenderer{
            renderer_context: None
        }
    }

    // fn serialize_all() {
    //     self.cameras;
    //     self.viewer;
    // }

    // fn send_payload() {
    //     self.renderer_context.cameras;
    //     self.renderer_context.matrix_transform;
    // }
}

impl renderers::Renderer for WebGLRenderer {

    fn init(
        &mut self,
        options: HashMap<String, String>,
        renderer_context: Rc<renderers::RendererContext>
    ) {
        self.renderer_context = Some(renderer_context);
        // self.renderer_context = Some(renderer_context);
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

    fn add_camera(&mut self, camera: &renderers::Camera) {
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
