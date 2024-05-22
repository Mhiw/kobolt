use crate::renderer::Renderer;

pub struct Application {
    renderer: Option<Renderer>,
    running: bool,
}

pub enum RunOption {
    Update,
    Init,
}

impl Application {
    pub fn new() -> Application {
        Application {
            renderer: None,
            running: false,
        }
    }

    pub fn execute(&self, function: fn(), option: RunOption) {
        match option {
            RunOption::Update => {
                while self.running {
                    function();
                }
            },
            RunOption::Init => {
                function();
            },
        };
    }

    pub fn quit(&mut self) {

    }
    
    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = Some(renderer);
    }
}
