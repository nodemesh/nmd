extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use std::rc::Rc;
use std::cell::RefCell;
use server;
use std::thread;
use std::thread::{JoinHandle};
use renderers;
use std::sync::{Arc, RwLock};
use self::ws::{listen, Sender as WSSender, Handler, Message, CloseCode, Handshake};
use std::sync::mpsc;
// use serde_json::builder::{ArrayBuilder, ObjectBuilder};

#[derive(Clone)]
enum WSMessage {
    INITIAL_PAYLOAD = 0
}

struct WebsocketServer {
    out: WSSender,
    graphs: Arc<RwLock<server::Graphs>>
    // rx: mspc::Receiver<WSMessage>,
}

impl Handler for WebsocketServer {
    // fn on_open(&mut self, _: Handshake) -> Result<(), Error> {
    //     thread::spawn(move || {
    //         loop {
    //             let message = self.rx.recv().unwrap();
    //             self.out.send(message);
    //         }
    //     })
    //     self.out.send()
    // }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let graphs = self.graphs.as_ref().read().unwrap();
        graphs.n;
        let data = msg.into_data();
        self.out.send("foo")
    }
}

pub struct WebGLRenderer {
    viewer: renderers::Viewer,
    options: HashMap<String, String>,
    handle: Option<JoinHandle<()>>
    // rx: Receiver<WSMessage>
}

impl WebGLRenderer {

    // fn get_payload_message(&self) -> Message {
        // MessageTypes::INITIAL_PAYLOAD;
        // self.viewer;
        // let value = ObjectBuilder::new()
        //     .insert("cameras", ArrayBuilder::new()
        //             .)
        //     pub cameras: HashMap<String, Camera>,
        // pub transform: Matrix4<f32>,
    // }

    pub fn new(
        graphs: Arc<RwLock<server::Graphs>>,
        viewer: renderers::Viewer,
        options: HashMap<String, String>
    ) -> WebGLRenderer {
        let mut renderer = WebGLRenderer{
            viewer: viewer,
            options: options,
            handle: None,
        };

        let addr = "127.0.0.1:12345";
        renderer.handle = Some(thread::spawn(move || {
            listen(addr, |out| {
                // add a listener tx to a shared list and rx to the server
                WebsocketServer{
                    out: out,
                    graphs: graphs.clone()
                    //rx: foo.clone()
                }
            }).unwrap();
        }));

        renderer
    }
}

impl renderers::Renderer for WebGLRenderer {

    fn viewer(&mut self) -> &mut renderers::Viewer {
        &mut self.viewer
    }

    fn set_viewer_transform(&mut self, transform: Matrix4<f32>) {
        // self.bus.unwrap().broadcast(WSMessage::INITIAL_PAYLOAD)
        // self.rx.send(Message{
        //     type: SET_VIEWER_TRANSFORM,
        //     transform: transform
        // });
    }

    fn add_camera(&mut self, camera: &renderers::Camera) {
        // self.rx.send(Message{
        //     type: ADD_CAMERA,
        //     camera: camera
        // });
    }

    fn delete_camera_with_name(&mut self, name: &str) {
        // self.rx.send(Message{
        //     type: DELETE_CAMERA_WITH_NAME,
        //     name: name
        // });
    }

    fn set_camera_transform(&mut self, camera_name: &str, transform: Matrix4<f32>) {
        // self.rx.send(Message{
        //     type: SET_CAMERA_TRANSFORM,
        //     name: camera_name,
        //     transform: transform
        // });
    }

    fn set_camera_projection(&mut self, camera_name: &str, projection: Matrix4<f32>) {
        // self.rx.send(Message{
        //     type: SET_CAMERA_PROJECTION,
        //     name: camera_name,
        //     projection: projection
        // });
    }
}
