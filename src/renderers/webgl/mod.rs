extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use std::rc::Rc;
use std::cell::RefCell;
use context;
use server;

// use self::ws::{connect};
// use context;
use renderers;

// struct Payload {
//     cameras: Vec<renderers::Camera>,
//     viewer_transform: Matrix4<f64>
// }

pub struct WebGLRenderer {
    graphs: Rc<RefCell<server::Graphs>>,
    viewer: renderers::Viewer,
    options: HashMap<String, String>
    // so: chat-using-web-socket
    // broadcast_rx: mpsc::channel::<Message>
}

impl WebGLRenderer {

    pub fn new(
        graphs: Rc<RefCell<server::Graphs>>,
        viewer: renderers::Viewer,
        options: HashMap<String, String>
    ) -> WebGLRenderer {
        WebGLRenderer{
            graphs: graphs,
            viewer: viewer,
            options: options
        }
    }
}

impl renderers::Renderer for WebGLRenderer {

    fn viewer(&mut self) -> &mut renderers::Viewer {
        &mut self.viewer
    }

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
