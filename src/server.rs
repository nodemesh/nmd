extern crate zmq;
extern crate libnm;
extern crate nalgebra as na;

use protobuf::*;
use self::libnm::protocol::messages;
use self::na::*;
use std::collections::HashMap;

use context;
use renderers;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// TODO: Matrix4 might already have a constructor method that
// initializes the matrix values from a vector
fn unserialize_matrix4<T>(v: &[T]) -> Matrix4<T> {
    Matrix4{
        m11: v[0],
        m21: v[1],
        m31: v[2],
        m41: v[3],
        m12: v[4],
        m22: v[5],
        m32: v[6],
        m42: v[7],
        m13: v[8],
        m23: v[9],
        m33: v[10],
        m43: v[11],
        m14: v[12],
        m24: v[13],
        m34: v[14],
        m44: v[15]
    }
}

pub struct Server<'a> {
    addr: &'a str,
    ctx: &'a mut context::Context<'a>
}

impl<'a> Server<'a> {

    pub fn new(ctx: &'a mut context::Context<'a>) -> Server<'a> {
        Server{
            ctx: ctx,
            addr: option_env!("NMD_ADDR").unwrap_or("*:5555"),
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
        let renderer: Box<renderers::Renderer> = match request.get_renderer_type() {
            messages::CreateRendererRequest_RendererType::DISPLAY => {
                unimplemented!()
                // Box::new(renderers::display::DisplayRenderer::new(
                //     self.ctx,
                //     renderer_options
                // ))
            },
            messages::CreateRendererRequest_RendererType::WEBGL => {
                Box::new(renderers::webgl::WebGLRenderer::new())
            }
        };

        // Set cameras.
        let mut cameras = HashMap::new();
        for camera in request.get_cameras() {
            let camera_name = camera.get_name().to_string();
            cameras.insert(camera_name, renderers::Camera{
                name: camera_name,
                transform: unserialize_matrix4(camera.get_transform()),
                projection: unserialize_matrix4(camera.get_projection())
            });
        }

        // Set up the renderer context.
        let renderer_context = renderers::RendererContext{
            cameras: cameras,
            viewer_transform: unserialize_matrix4(
                request.get_viewer_transform()
            ),
            renderer: renderer
        };

        // Initialize the renderer with options.
        let mut renderer_options = HashMap::new();
        for option in request.take_options().iter() {
            renderer_options.insert(option.get_key(), option.get_value());
        }

        renderer.init(renderer_options, &renderer_context);

        // Add the renderer to context.
        let renderer_id = self.ctx.add_renderer(renderer_context);
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
        self.ctx.delete_renderer_context_with_id(renderer_id);
        None
    }

    fn update_viewer_transform(
        &mut self, request: &messages::UpdateViewerTransformRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let rctx = self.ctx.get_renderer_context_with_id(renderer_id);
        rctx.set_viewer_transform(
            unserialize_matrix4(request.get_transform())
        );
        None
    }

    fn create_camera(
        &mut self, request: &messages::CreateCameraRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let req_camera = request.get_camera();
        let rctx = self.ctx.get_renderer_context_with_id(renderer_id);
        let camera = renderers::Camera{
            name: req_camera.get_name().to_string(),
            transform: unserialize_matrix4(req_camera.get_transform()),
            projection: unserialize_matrix4(req_camera.get_projection())
        };
        rctx.add_camera(camera);
        None
    }

    fn delete_camera(
        &mut self, request: &messages::DeleteCameraRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let rctx = self.ctx.get_renderer_context_with_id(renderer_id);
        rctx.delete_camera_with_name(request.get_camera_name());
        None
    }

    fn update_camera_transform(
        &mut self, request: &messages::UpdateCameraTransformRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let rctx = self.ctx.get_renderer_context_with_id(renderer_id);
        rctx.set_camera_transform(
            request.get_camera_name(),
            unserialize_matrix4(request.get_transform())
        );
        None
    }

    fn update_camera_projection(
        &mut self, request: &messages::UpdateCameraProjectionRequest
    ) -> Option<messages::Response> {
        let renderer_id = request.get_renderer_id();
        let rctx = self.ctx.get_renderer_context_with_id(renderer_id);
        rctx.set_camera_projection(
            request.get_camera_name(),
            unserialize_matrix4(request.get_transform())
        );
        None
    }
}
