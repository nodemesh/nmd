extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use server;
use std::thread;
use std::thread::{JoinHandle};
use renderers;
use std::sync::{Arc, RwLock};
use self::ws::{WebSocket, Sender as WSSender, Handler, Message};
use std::sync::mpsc;

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
        self.out.send("response")
    }
}

pub struct WebGLRenderer {
    viewer: renderers::Viewer,
    options: HashMap<String, String>,
    join_handle: Option<JoinHandle<()>>,
    tx: mpsc::Sender<Message>
}

impl WebGLRenderer {

    pub fn new(
        graphs: Arc<RwLock<server::Graphs>>,
        viewer: renderers::Viewer,
        options: HashMap<String, String>
    ) -> WebGLRenderer {
        let (tx, rx) = mpsc::channel();

        let mut renderer = WebGLRenderer{
            viewer: viewer,
            options: options,
            join_handle: None,
            tx: tx
        };

        let addr = "127.0.0.1:12345";
        renderer.join_handle = Some(thread::spawn(move || {
            let ws = WebSocket::new(|out| {
                WebsocketServer{
                    out: out,
                    graphs: graphs.clone()
                }
            }).unwrap();
            let broadcaster = ws.broadcaster();

            thread::spawn(move || {
                let msg = rx.recv().unwrap();
                let _ = broadcaster.send(msg);
            });

            ws.listen(addr).unwrap();
        }));

        renderer
    }
}

impl Drop for WebGLRenderer {

    fn drop(&mut self) {
        self.join_handle.take().unwrap().join();
    }
}

impl renderers::Renderer for WebGLRenderer {

    fn viewer(&mut self) -> &mut renderers::Viewer {
        &mut self.viewer
    }

    fn set_viewer_transform(&mut self, transform: Matrix4<f32>) {
        let msg = ws::Message::Text("hello world".to_string());
        let _ = self.tx.send(msg);
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
