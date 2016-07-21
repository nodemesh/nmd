extern crate serde;
extern crate serde_json;
extern crate nalgebra as na;
extern crate ws;

use std::collections::HashMap;
use self::na::Matrix4;
use server;
use std::thread;
use std::thread::{JoinHandle};
use renderers;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use self::ws::{WebSocket, Sender as WSSender, Handler, Message, Handshake};
use std::sync::mpsc;

enum ServerMessage<'a> {
    UpdateViewerTransformMessage { transform: Matrix4<f32> },
    AddCamera { camera: &'a renderers::Camera },
    DeleteCameraWithName { name: &'a str },
    UpdateCameraTransform { camera_name: &'a str, transform: Matrix4<f32> },
    UpdateCameraProjection { camera_name: &'a str, projection: Matrix4<f32> },
    SetInitialState
}

impl<'a> serde::Serialize for ServerMessage<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        serializer.serialize_map(ServerMessageMapVisitor { value: self })
    }
}

struct ServerMessageMapVisitor<'a> {
    value: &'a ServerMessage<'a>
}

impl<'a> serde::ser::MapVisitor for ServerMessageMapVisitor<'a> {
    fn visit<S: serde::Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
        match *self.value {
            ServerMessage::UpdateViewerTransformMessage{ transform } => {
                try!(serializer.serialize_map_elt("code", 0));
                // TODO: try!(serializer.serialize_map_elt("transform", transform));
            },
            ServerMessage::AddCamera{ camera } => {
                try!(serializer.serialize_map_elt("code", 1));
                // TODO: try!(serializer.serialize_map_elt("camera", camera));
            },
            ServerMessage::DeleteCameraWithName{ name } => {
                try!(serializer.serialize_map_elt("code", 2));
                try!(serializer.serialize_map_elt("name", name));
            },
            ServerMessage::UpdateCameraTransform{ camera_name, transform } => {
                try!(serializer.serialize_map_elt("code", 3));
                try!(serializer.serialize_map_elt("camera_name", camera_name));
                // TODO: try!(serializer.serialize_map_elt("transform", transform));
            },
            ServerMessage::UpdateCameraProjection{ camera_name, projection } => {
                try!(serializer.serialize_map_elt("code", 4));
                try!(serializer.serialize_map_elt("camera_name", camera_name));
                // TODO: try!(serializer.serialize_map_elt("projection", projection));
            },
            ServerMessage::SetInitialState => {
                try!(serializer.serialize_map_elt("code", 5));
            },
        };

        Ok(None)
    }
}

struct WebsocketServer {
    out: WSSender,
    graphs: Arc<RwLock<server::Graphs>>,
    viewer: Arc<RwLock<renderers::Viewer>>
}

impl WebsocketServer {
    fn payload_message(&self) -> ws::Message {
        // let graphs = self.graphs.as_ref().read().unwrap();
        // let viewer = self.viewer.as_ref().read().unwrap();
        let msg = ServerMessage::SetInitialState{};
        Message::Binary(serde_json::to_vec(&msg).unwrap())
    }
}

impl Handler for WebsocketServer {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        // The the client connects, send a snapshot of the
        // server state.
        self.out.send(self.payload_message())
    }

    // fn on_message(&mut self, msg: Message) -> ws::Result<()> {
    //     let graphs = self.graphs.as_ref().read().unwrap();
    //     graphs.n;
    //     let data = msg.into_data();
    //     self.out.send("response")
    // }
}

pub struct WebGLRenderer {
    viewer: Arc<RwLock<renderers::Viewer>>,
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

        let viewer = Arc::new(RwLock::new(viewer));
        let mut renderer = WebGLRenderer{
            viewer: viewer.clone(),
            options: options,
            join_handle: None,
            tx: tx
        };

        // TODO: get addr from options
        let addr = "127.0.0.1:12345";

        renderer.join_handle = Some(thread::spawn(move || {
            let ws = WebSocket::new(|out| {
                WebsocketServer{
                    out: out,
                    graphs: graphs.clone(),
                    viewer: viewer.clone()
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
        let _ = self.join_handle.take().unwrap().join();
    }
}

impl renderers::Renderer for WebGLRenderer {
    fn viewer(&mut self) -> RwLockWriteGuard<renderers::Viewer> {
        self.viewer.as_ref().write().unwrap()
    }

    fn update_viewer_transform(&mut self, transform: Matrix4<f32>) {
        let msg = ServerMessage::UpdateViewerTransformMessage {
            transform: transform
        };
        let _ = self.tx.send(Message::Binary(serde_json::to_vec(&msg).unwrap()));
    }

    fn add_camera(&mut self, camera: &renderers::Camera) {
        let msg = ServerMessage::AddCamera { camera: camera };
        let _ = self.tx.send(Message::Binary(serde_json::to_vec(&msg).unwrap()));
    }

    fn delete_camera_with_name(&mut self, name: &str) {
        let msg = ServerMessage::DeleteCameraWithName { name: name };
        let _ = self.tx.send(Message::Binary(serde_json::to_vec(&msg).unwrap()));
    }

    fn update_camera_transform(&mut self, camera_name: &str, transform: Matrix4<f32>) {
        let msg = ServerMessage::UpdateCameraTransform {
            camera_name: camera_name,
            transform: transform
        };
        let _ = self.tx.send(Message::Binary(serde_json::to_vec(&msg).unwrap()));
    }

    fn update_camera_projection(&mut self, camera_name: &str, projection: Matrix4<f32>) {
        let msg = ServerMessage::UpdateCameraProjection {
            camera_name: camera_name,
            projection: projection
        };
        let _ = self.tx.send(Message::Binary(serde_json::to_vec(&msg).unwrap()));
    }
}
