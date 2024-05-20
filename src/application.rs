use crate::renderer::*;

pub struct Application {
    renderer: Option<Renderer>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            renderer: None,
        }
    }
}
