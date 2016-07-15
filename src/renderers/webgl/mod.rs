use renderers;

pub struct WebGLRenderer {

}

impl WebGLRenderer {
    pub fn new() -> WebGLRenderer {
        WebGLRenderer{}
    }
}

impl renderers::Renderer for WebGLRenderer {
}
