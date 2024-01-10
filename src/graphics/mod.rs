use bevy::prelude::*;

use strum::{Display, EnumIter};

use crate::{map::Position, vector2_int::Vector2Int};

use self::{
    anim_data::AnimDataPlugin, animations::AnimationsPlugin, assets::AssetsPlugin,
    pokemon::PiecesPlugin, tiles::TilesPlugin,
};

pub mod anim_data;
mod animations;
pub mod assets;
mod pokemon;
mod tiles;

pub const TILE_Z: f32 = 0.;
pub const TILE_SIZE: f32 = 24.;

pub const PIECE_Z: f32 = 10.;
// pub const PIECE_SIZE: f32 = 32.;
pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>().add_plugins((
            TilesPlugin,
            PiecesPlugin,
            AssetsPlugin,
            AnimDataPlugin,
            AnimationsPlugin,
        ));
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

#[derive(Debug, EnumIter, Display)]
pub enum Orientation {
    South,
    SouthEst,
    Est,
    NorthEst,
    North,
    NorthWest,
    West,
    SouthWest,
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.0.x as f32,
        TILE_SIZE * position.0.y as f32,
        z,
    )
}

fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}

fn get_orientation_from_vector(direction: Vector2Int) -> Orientation {
    match direction {
        Vector2Int { x: 0, y: -1 } => Orientation::South,
        Vector2Int { x: 1, y: -1 } => Orientation::SouthEst,
        Vector2Int { x: 1, y: 0 } => Orientation::Est,
        Vector2Int { x: 1, y: 1 } => Orientation::NorthEst,
        Vector2Int { x: 0, y: 1 } => Orientation::North,
        Vector2Int { x: -1, y: 1 } => Orientation::NorthWest,
        Vector2Int { x: -1, y: 0 } => Orientation::West,
        Vector2Int { x: -1, y: -1 } => Orientation::SouthWest,
        Vector2Int { x, y } => {
            warn!("unable to get orientation from {:?}", direction);
            Orientation::South
        }
    }
}
