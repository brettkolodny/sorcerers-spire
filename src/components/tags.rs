use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct GameplayTag;

impl Component for GameplayTag {
    type Storage = NullStorage<Self>;
}
