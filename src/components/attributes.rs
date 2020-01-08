use amethyst::ecs::prelude::{Component, DenseVecStorage, VecStorage};

pub struct Speed {
    pub speed: f32,
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Speed { speed }
    }
}


impl Default for Speed {
    fn default() -> Self {
        Speed { speed: 1000.0 }
    }
}

impl Component for Speed {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }
}

impl Component for Size {
    type Storage = DenseVecStorage<Self>;
}
