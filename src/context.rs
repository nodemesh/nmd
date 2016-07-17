use std::collections::HashMap;

use renderers;

pub type Id = i64;

pub struct Context<'a> {
    renderer_contexts: HashMap<Id, renderers::RendererContext<'a>>,
    id: Id
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context{
            id: 0,
            renderer_contexts: HashMap::new()
        }
    }

    fn get_id(&mut self) -> Id {
        self.id += 1;
        self.id
    }

    pub fn add_renderer(
        &mut self, rctx: renderers::RendererContext<'a>
    ) -> Id {
        let rid = self.get_id();
        self.renderer_contexts.insert(rid, rctx);
        rid
    }

    pub fn delete_renderer_context_with_id(&mut self, renderer_id: Id) {
        self.renderer_contexts.remove(&renderer_id);
    }

    pub fn get_renderer_context_with_id(&self, renderer_id: Id) -> &renderers::RendererContext<'a> {
        // TODO: handle the error case here.
        self.renderer_contexts.get(&renderer_id).unwrap()
    }
}
