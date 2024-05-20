pub trait RenderTarget {
    fn fill(&mut self, buf: Buffer);
}

pub struct Buffer(Vec<Vec<Pixel>>);

impl Buffer {
    pub fn get(self) -> Vec<Vec<Pixel>> {
        self.0
    }
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Renderer {
    dimensions: (usize, usize),
    target: Option<Box<dyn RenderTarget>>,
    buffer: Option<Buffer>,
}

impl Renderer {
    pub fn new(dimensions: (usize, usize)) -> Renderer {
        Renderer {
            dimensions,
            target: None,
            buffer: None,
        }
    }
}
