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

    pub fn execute(&mut self, function: fn(), option: RunOption) -> &mut Self {
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
        self
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
    
    pub fn set_renderer(&mut self, renderer: Renderer) -> &mut Self {
        self.renderer = Some(renderer);
        self
    }
}
