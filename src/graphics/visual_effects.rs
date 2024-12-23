use std::time::Duration;

use bevy::prelude::*;

use crate::{constants::GAME_SPEED, visual_effects::VisualEffect, GameState};

use super::{
    animations::{AnimationFinished, AnimationFrame, Animator},
    assets::visual_effect_assets::VisualEffectAssets,
    FRAME_DURATION_MILLIS,
};

pub struct VisualEffectsPlugin;

impl Plugin for VisualEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_visual_effect_renderer, auto_despawn_effect)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct AutoDespawnEffect;

fn spawn_visual_effect_renderer(
    mut commands: Commands,
    visual_effect_assets: Res<VisualEffectAssets>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    query: Query<(Entity, &VisualEffect, &Transform), Added<VisualEffect>>,
) {
    for (entity, effect, _transform) in query.iter() {
        let Some(effect_texture_info) = visual_effect_assets.0.get(effect.name).cloned() else {
            warn!("Visual effect texture not found for {}", effect.name);
            continue;
        };

        let Some(texture_atlas) = texture_atlases.get(&effect_texture_info.layout) else {
            continue;
        };

        let frames = texture_atlas
            .textures
            .iter()
            .enumerate()
            .map(|(atlas_index, _)| AnimationFrame {
                atlas_index,
                duration: Duration::from_millis(
                    ((FRAME_DURATION_MILLIS * 2) as f32 / GAME_SPEED).floor() as u64,
                ),
            })
            .collect::<Vec<_>>();

        let first_index = frames[0].atlas_index;

        commands.entity(entity).insert((
            Animator::new(
                effect_texture_info.layout.clone(),
                effect_texture_info.texture.clone(),
                frames,
                effect.is_loop,
                None,
                None,
                None,
            ),
            Sprite::default(),
            effect_texture_info.texture.clone(),
            TextureAtlas {
                index: first_index,
                layout: effect_texture_info.layout.clone(),
            },
        ));
    }
}

// TODO: maybe make it more generic to be used with other animator
fn auto_despawn_effect(
    query: Query<&Animator, (With<VisualEffect>, With<AutoDespawnEffect>)>,
    mut ev_animation_finished: EventReader<AnimationFinished>,
    mut commands: Commands,
) {
    for ev in ev_animation_finished.read() {
        let Ok(animator) = query.get(ev.0) else {
            continue;
        };

        if animator.is_finished() {
            commands.entity(ev.0).despawn_recursive();
        }
    }
}
