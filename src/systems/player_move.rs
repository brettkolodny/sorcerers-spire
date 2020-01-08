use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;

use crate::components::{Player, Speed};

pub struct PlayerMove;

impl<'s> System<'s> for PlayerMove {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Speed>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, speeds, players, cameras, input): Self::SystemData) {
        let mut player_change_x = 0.0;
        let mut player_change_y = 0.0;

        for (_, transform, speed) in (&players, &mut transforms, &speeds).join() {
            let horizontal_movement = input.axis_value("horizontal");
            let vertical_movement = input.axis_value("vertical");

            let mut change_x: f32 = {
                if let Some(amount) = horizontal_movement {
                    player_change_x = speed.speed * amount as f32;
                    player_change_x
                } else {
                    0.0
                }
            };

            let mut change_y: f32 = {
                if let Some(amount) = vertical_movement {
                    player_change_y = speed.speed * amount as f32;
                    player_change_y
                } else {
                    0.0
                }
            };

            transform.set_translation_x(transform.translation().x + change_x); 
        }

        for (_, transform) in (&cameras, &mut transforms).join() {
            transform.set_translation_x(transform.translation().x + player_change_x); 
        }
    }
}
