use std::collections::HashMap;

use renderers;

pub type Id = i64;

pub struct Context<'a> {
    renderers: HashMap<Id, Box<renderers::Renderer + 'a>>,
    id: Id,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context{
            id: 0,
            renderers: HashMap::new()
        }
    }

    fn get_id(&mut self) -> Id {
        self.id += 1;
        self.id
    }

    pub fn add_renderer(
        &mut self, renderer: Box<renderers::Renderer + 'a>
    ) -> Id {
        let rid = self.get_id();
        self.renderers.insert(rid, renderer);
        rid
    }

    pub fn delete_renderer_with_id(&mut self, renderer_id: Id) {
        self.renderers.remove(&renderer_id);
    }

    pub fn get_renderer_with_id(&mut self, renderer_id: Id) -> &mut renderers::Renderer {
        // TODO: handle the error case here.
        &mut **(self.renderers.get_mut(&renderer_id).unwrap())
    }
}
