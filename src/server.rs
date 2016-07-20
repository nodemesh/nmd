extern crate zmq;
extern crate libnm;
extern crate nalgebra as na;

use protobuf::*;
use self::libnm::protocol::messages;
use self::na::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use context;
use renderers;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// TODO: Matrix4 might already have a constructor method that
// initializes the matrix values from a vector
fn unserialize_matrix4(v: &[f32]) -> Matrix4<f32> {
    Matrix4::new(
        v[0],
        v[1],
        v[2],
        v[3],
        v[4],
        v[5],
        v[6],
        v[7],
        v[8],
        v[9],
        v[10],
        v[11],
        v[12],
        v[13],
        v[14],
        v[15]
    )
}

pub struct Graphs;

pub struct Server<'a> {
    addr: &'a str,
    ctx: context::Context<'a>,
    graphs: Rc<RefCell<Graphs>>
}

impl<'a> Server<'a> {

    pub fn new() -> Server<'a> {
        Server{
            addr: option_env!("NMD_ADDR").unwrap_or("*:5555"),
            ctx: context::Context::new(),
            graphs: Rc::new(RefCell::new(Graphs{}))
        }
    }

    pub fn listen(&mut self) {
        let mut context = zmq::Context::new();
        let mut responder = context.socket(zmq::REP).unwrap();

        let addr = format!("tcp://{}", self.addr);
        responder.bind(&addr).ok().expect("Could not bind to the address.");

        let mut msg = zmq::Message::new().unwrap();

        loop {
            responder.recv(&mut msg, 0).unwrap();
            let requests = parse_from_bytes::<messages::Requests>(&msg).unwrap();
            let mut responses = Vec::new();

            for (i, request) in requests.get_requests().iter().enumerate() {
                let response = self.get_response(request);

                if let Some(mut res) = response {
                    res.set_index(i as i64);
                    responses.push(res);
                }
            }

            let mut res_message = messages::Responses::new();
            res_message.set_responses(RepeatedField::from_vec(responses));
            let bytes = res_message.write_to_bytes().unwrap();
            responder.send(&bytes, 0).unwrap();
        }
    }

    fn get_response(
        &mut self, request: &messages::Request
    ) -> Option<messages::Response> {
        match request.get_request_type() {
            messages::Request_RequestType::GET_VERSION => {
                Server::get_version()
            },
            messages::Request_RequestType::CREATE_RENDERER => {
                self.create_renderer(request.get_create_renderer_request())
            },
            messages::Request_RequestType::DELETE_RENDERER => {
                self.delete_renderer(request.get_delete_renderer_request())
            },
            messages::Request_RequestType::UPDATE_VIEWER_TRANSFORM => {
                self.update_viewer_transform(
                    request.get_update_viewer_transform_request()
                )
            },
            messages::Request_RequestType::CREATE_CAMERA => {
                self.create_camera(request.get_create_camera_request())
            },
            messages::Request_RequestType::DELETE_CAMERA => {
                self.delete_camera(request.get_delete_camera_request())
            },
            messages::Request_RequestType::UPDATE_CAMERA_TRANSFORM => {
                self.update_camera_transform(
                    request.get_update_camera_transform_request()
                )
            },
            messages::Request_RequestType::UDPATE_CAMERA_PROJECTION => {
                self.update_camera_projection(
                    request.get_update_camera_projection_request()
                )
            },
            _ => None
        }
    }

    fn get_version() -> Option<messages::Response> {
        let mut res = messages::GetVersionResponse::new();
        res.set_version(VERSION.unwrap_or("unknown").to_string());

        let mut msg = messages::Response::new();
        msg.set_response_type(messages::Response_ResponseType::GET_VERSION);
        msg.set_get_version_response(res);
        return Some(msg);
    }

    fn create_renderer(
        &mut self, request: &messages::CreateRendererRequest
    ) -> Option<messages::Response> {
        let mut cameras = HashMap::new();
        for camera in request.get_cameras() {
            let camera_name = camera.get_name().to_string();
            cameras.insert(camera_name.clone(), renderers::Camera{
                name: camera_name,
                transform: unserialize_matrix4(camera.get_transform()),
                projection: unserialize_matrix4(camera.get_projection())
            });
        }

        let viewer = renderers::Viewer{
            cameras: cameras,
            transform: unserialize_matrix4(
                request.get_viewer_transform()
            ),
        };

        // Initialize the renderer with options.
        let mut renderer_options = HashMap::new();
        for option in request.get_options() {
            renderer_options.insert(option.get_key().to_string(), option.get_value().to_string());
        }

        let renderer: Box<renderers::Renderer + 'a> = Box::new(
            renderers::webgl::WebGLRenderer::new(
                self.graphs.clone(),
                viewer,
                renderer_options
            )
        );


        // renderer.init(renderer_options);

        // let renderer: Box<renderers::Renderer> = match request.get_renderer_type() {
        //     messages::CreateRendererRequest_RendererType::DISPLAY => {
        //         unimplemented!()
        //         // Box::new(renderers::display::DisplayRenderer::new(
        //         //     self.ctx,
        //         //     renderer_options
        //         // ))
        //     },
        //     messages::CreateRendererRequest_RendererType::WEBGL => {
        //         Box::new(renderers::webgl::WebGLRenderer{
        //             graphs: &self.ctx.graphs
        //         })
        //     }
        // };

        // Add the renderer to context.
        let renderer_id = self.ctx.add_renderer(renderer);
        let mut msg = messages::ItemCreatedResponse::new();
        msg.set_item_id(renderer_id);

        let mut res = messages::Response::new();
        res.set_response_type(messages::Response_ResponseType::ITEM_CREATED);
        res.set_item_created_response(msg);
        Some(res)
    }

    fn delete_renderer(
        &mut self, request: &messages::DeleteRendererRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        self.ctx.delete_renderer_with_id(renderer_id);
        None
    }

    fn update_viewer_transform(
        &mut self, request: &messages::UpdateViewerTransformRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let renderer = self.ctx.get_renderer_with_id(renderer_id);
        let transform = unserialize_matrix4(request.get_transform());
        renderer.viewer().transform = transform;
        renderer.set_viewer_transform(transform);
        None
    }

    fn create_camera(
        &mut self, request: &messages::CreateCameraRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let req_camera = request.get_camera();
        let renderer = self.ctx.get_renderer_with_id(renderer_id);
        let camera_name = req_camera.get_name();
        let camera = renderers::Camera{
            name: camera_name.to_string(),
            transform: unserialize_matrix4(req_camera.get_transform()),
            projection: unserialize_matrix4(req_camera.get_projection())
        };
        renderer.viewer().cameras.insert(camera_name.to_string(), camera);
        // TODO: renderer.add_camera(camera);
        None
    }

    fn delete_camera(
        &mut self, request: &messages::DeleteCameraRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let renderer = self.ctx.get_renderer_with_id(renderer_id);
        let camera_name = request.get_camera_name();
        renderer.viewer().cameras.remove(camera_name);
        renderer.delete_camera_with_name(request.get_camera_name());
        None
    }

    fn update_camera_transform(
        &mut self, request: &messages::UpdateCameraTransformRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let mut renderer = self.ctx.get_renderer_with_id(renderer_id);
        let camera_name = request.get_camera_name();
        let transform = unserialize_matrix4(request.get_transform());
        renderer.viewer().cameras.get_mut(camera_name).unwrap().transform = transform;
        renderer.set_camera_transform(
            camera_name,
            transform
        );
        None
    }

    fn update_camera_projection(
        &mut self, request: &messages::UpdateCameraProjectionRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let renderer = self.ctx.get_renderer_with_id(renderer_id);
        let camera_name = request.get_camera_name();
        let projection = unserialize_matrix4(request.get_transform());
        renderer.viewer().cameras.get_mut(camera_name).unwrap().projection = projection;
        renderer.set_camera_projection(
            camera_name,
            projection
        );
        None
    }
}
