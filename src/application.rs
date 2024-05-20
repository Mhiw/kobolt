use crate::renderer::*;

pub struct Application {
    renderer: Renderer,
}

impl Default for Application {
    fn default() -> Application {
        Application {
            renderer: Renderer::new(),
        }
    }
}

impl Application {
    pub fn new() -> Application {
        Application {
            renderer: Renderer::new(),
        }
    }
}
