use renderers;

pub struct DisplayRenderer;

impl DisplayRenderer {
    pub fn new() -> DisplayRenderer {
        DisplayRenderer{}
    }
}

impl renderers::Renderer for DisplayRenderer {
}
