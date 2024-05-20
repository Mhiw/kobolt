use crate::renderer::Renderer;

pub struct Application {
    renderer: Option<Renderer>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            renderer: None,
        }
    }
    
    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = Some(renderer);
    }
}
