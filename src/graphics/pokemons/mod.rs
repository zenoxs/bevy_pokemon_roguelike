pub mod offsets;
mod pokemon_animator;
mod shadow;

use bevy::prelude::*;
use char_animation::{anim_key::AnimKey, CharAnimation};

use crate::{
    map::Position, pieces::FacingOrientation, pokemons::Pokemon, GamePlayingSet, GameState,
};

use self::{
    offsets::{
        debug_offsets, update_body_offset, update_head_offset, update_offsets, PokemonBodyOffset,
        PokemonHeadOffset,
    },
    pokemon_animator::get_pokemon_animator,
    shadow::{spawn_shadow_renderer, update_shadow_offsets, PokemonShadow},
};

use super::{
    action_animations::ActionAnimationSet, assets::pokemon_chara_assets::PokemonCharaAssets,
    POKEMON_Z,
};

pub struct PokemonPlugin;

// TODO: Create plugin for sub systems
impl Plugin for PokemonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PokemonAnimationState>()
            .add_event::<AnimatorUpdatedEvent>()
            .add_systems(
                Update,
                (spawn_pokemon_renderer, spawn_shadow_renderer)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_animator,
                    // update_shadow_animator,
                    // update_offsets_animator,
                    update_head_offset,
                    update_body_offset,
                    // update_pokemon_shadow_renderer,
                )
                    .chain()
                    .in_set(ActionAnimationSet::Animator),
            )
            .add_systems(
                Update,
                (update_offsets, update_shadow_offsets).after(GamePlayingSet::LateLogics),
            );
        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, (debug_offsets).run_if(in_state(GameState::Playing)));
        }
    }
}

#[derive(Event, Debug)]
pub struct AnimatorUpdatedEvent(pub Entity);

#[derive(Component, Default, Reflect)]
pub struct PokemonAnimationState(pub AnimKey);

#[allow(clippy::type_complexity)]
fn update_animator(
    mut query: Query<
        (
            Entity,
            &FacingOrientation,
            &PokemonAnimationState,
            &Handle<CharAnimation>,
            &mut TextureAtlas,
            &mut Handle<Image>,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    char_animation_assets: Res<Assets<CharAnimation>>,
    mut ev_animator_updated: EventWriter<AnimatorUpdatedEvent>,
    mut commands: Commands,
) {
    for (
        entity,
        facing_orientation,
        animation_state,
        char_animation_handle,
        mut texture_atlas,
        mut texture,
    ) in query.iter_mut()
    {
        let Some(animator) = get_pokemon_animator(
            &char_animation_assets,
            char_animation_handle,
            &animation_state.0,
            &facing_orientation.0,
        ) else {
            continue;
        };
        texture_atlas.layout = animator.atlas_layout.clone();
        *texture = animator.texture.clone();
        commands.entity(entity).insert(animator);
        ev_animator_updated.send(AnimatorUpdatedEvent(entity));
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    char_animation_assets: Res<Assets<CharAnimation>>,
    pokemon_char_assets: Res<PokemonCharaAssets>,
    query: Query<(Entity, &Position, &Pokemon, &FacingOrientation), Added<Pokemon>>,
) {
    let default_state = AnimKey::Idle;
    for (entity, position, pokemon, orientation) in query.iter() {
        let pokemon_animation_handle = pokemon_char_assets.0.get(&pokemon.id).unwrap();
        let pokemon_char_animation = char_animation_assets.get(pokemon_animation_handle).unwrap();
        let animation_data = pokemon_char_animation.anim.get(&default_state).unwrap();

        let char_animation_offsets = &animation_data.offsets.get(&orientation.0).unwrap()[0];

        let v = super::get_world_position(&position.0, POKEMON_Z);

        let atlas = TextureAtlas {
            index: 0,
            layout: animation_data.atlas_layout.clone(),
        };

        commands
            .entity(entity)
            .insert((
                PokemonAnimationState(default_state),
                pokemon_animation_handle.clone(),
                char_animation_offsets.clone(),
                SpriteSheetBundle {
                    atlas,
                    texture: animation_data.texture.clone(),
                    transform: Transform::from_translation(v),
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Shadow
                parent.spawn((Name::new("Shadow"), PokemonShadow::default()));
            })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("HeadOffset"),
                    PokemonHeadOffset,
                    SpatialBundle::default(),
                ));
            })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("BodyOffset"),
                    PokemonBodyOffset,
                    SpatialBundle::default(),
                ));
            });
    }
}
