

use bevy::prelude::*;

use crate::{map::Position, player::Player, vector2_int::Vector2Int, GameState};

pub struct GameControlPlugin;

impl Plugin for GameControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameControlEvent>().add_systems(
            Update,
            player_input_controls.run_if(in_state(GameState::Playing)),
        );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int { x: 0, y: 1 }),
    (KeyCode::S, Vector2Int { x: 0, y: -1 }),
    (KeyCode::A, Vector2Int { x: -1, y: 0 }),
    (KeyCode::D, Vector2Int { x: 1, y: 0 }),
];

#[derive(Event)]
pub struct GameControlEvent(pub GameControl);

pub enum GameControl {
    Target(Vector2Int),
    Other,
}

fn player_input_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&Position, With<Player>>,
    mut ev_game_control: EventWriter<GameControlEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else {
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !keyboard_input.just_pressed(key) {
            continue;
        }
        ev_game_control.send(GameControlEvent(GameControl::Target(position.0 + dir)));
    }
}
