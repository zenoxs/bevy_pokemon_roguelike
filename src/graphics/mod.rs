use bevy::prelude::*;

use crate::{map::Position, vector2_int::Vector2Int};

use self::{
    action_animation::ActionAnimationPlugin, anim_data::AnimDataPlugin,
    animations::AnimationsPlugin, assets::AssetsPlugin, pokemon::PokemonPlugin, tiles::TilesPlugin,
};

pub mod action_animation;
pub mod anim_data;
pub mod animations;
pub mod assets;
mod pokemon;
mod tiles;

pub const TILE_Z: f32 = 0.;
pub const TILE_SIZE: f32 = 24.;

pub const PIECE_Z: f32 = 10.;
// pub const PIECE_SIZE: f32 = 32.;
pub const WALK_SPEED: f32 = 1.5;
pub const POSITION_TOLERANCE: f32 = 0.1;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>().add_plugins((
            ActionAnimationPlugin,
            TilesPlugin,
            PokemonPlugin,
            AssetsPlugin,
            AnimDataPlugin,
            AnimationsPlugin,
        ));
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

fn get_world_position(position: &Vector2Int, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.x as f32,
        TILE_SIZE * position.y as f32,
        z,
    )
}

fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}
