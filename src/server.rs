extern crate zmq;
extern crate libnm;

use protobuf::*;
use self::libnm::protocol::messages;

use context;
use renderers;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Server<'a> {
    addr: &'a str,
    ctx: &'a mut context::Context
}

impl<'a> Server<'a> {

    pub fn new(ctx: &'a mut context::Context) -> Server<'a> {
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

    fn get_response(&mut self, request: &messages::Request) -> Option<messages::Response> {
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
                Box::new(renderers::display::DisplayRenderer::new())
            },
            messages::CreateRendererRequest_RendererType::WEBGL => {
                Box::new(renderers::webgl::WebGLRenderer::new())
            }
        };

        // TODO: set renderer info
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
        self.ctx.delete_renderer(renderer_id);
        None
    }
}
