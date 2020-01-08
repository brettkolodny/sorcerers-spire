use amethyst::ecs::prelude::*;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::{math::Vector3, transform::Transform};
use amethyst::window::ScreenDimensions;
use amethyst::renderer::{formats::texture::ImageFormat, sprite::SpriteSheetHandle, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::{tags::GameplayTag, attributes::{Speed, Size}};

#[derive(Default)]
pub struct Player;

impl Player {
    pub fn init(world: &mut World, dimensions: &ScreenDimensions) {
        let mut transform = Transform::default(); 
        transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 0.0);

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/knight/knight_idle.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "sprites/knight/knight_idle.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };

        let sprite_render = SpriteRender {
           sprite_sheet: sheet_handle, 
           sprite_number: 2,
        };
        
       world
           .create_entity()
           .with(Player)
           .with(sprite_render)
           .with(Speed::new(5.0))
           .with(GameplayTag)
           .with(transform)
           .build();
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
