//! # bevy_sprite_material
//!
//! Materials for sprites in Bevy
//!
//! [![workflow](https://github.com/ManevilleF/bevy_sprite_material/actions/workflows/rust.yaml/badge.svg)](https://github.com/ManevilleF/bevy_sprite_material/actions/workflows/rust.yaml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_sprite_material.svg)](https://crates.io/crates/bevy_sprite_material)
//! [![Docs.rs](https://docs.rs/bevy_sprite_material/badge.svg)](https://docs.rs/bevy_sprite_material)
//! [![dependency status](https://deps.rs/crate/bevy_sprite_material/0.2.0/status.svg)](https://deps.rs/crate/bevy_sprite_material)
//!
//! This [bevy] plugin changes the `bevy_sprite` implementation using a material.
//!
//! > You might be interested in [bevy_ui_material](https://github.com/ManevilleF/bevy_ui_material) which is a similar plugin for `bevy_ui` instead of `bevy_sprite`.
//!
//! The new `Sprite` component is identical to `bevy_sprite` but without the `color` field.
//!
//! The new `SpriteBundle` component bundle replaces the `texture` field (`Handle<Image>`) by a `material` field (`Handle<ColorMaterial>`)
//!
//! ## Objective
//!
//! The goal of this plugin is to allow seamless edition of sprites `texture` **and** `color` which was removed with [bevy] 0.6.
//!
//! This is very useful if you have many sprites and you have, for example, various themes and don't want to *query* every sprite to change its color.
//!
//! If you have a dedicated artist, you probably don't use the sprite `color` tinting field anyway, so the base implementation is perfect for you.
//! This is specifically if you want to "massively update" the sprite `color` and maybe the `texture` as well.
//!
//! ## Disclaimer
//!
//! This plugin is very straightforward, and simply plugs itself in the `bevy_sprite` render pipeline (in the *extraction* stage).
//! This system might be slower than the base implementation, because of the extra `Handle` involved.
//!
//! Also, there might be compatibility issues, so feel free to open issues or merge requests.
//!
//! > This plugin should work fine if you use both the plugin and the base sprite implementation
//!
//! [bevy]: https://github.com/bevyengine/bevy
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style,
    rustdoc::broken_intra_links
)]

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_render::{ExtractSchedule, RenderApp, RenderSet};
pub use bevy_sprite::ColorMaterial;
use bevy_sprite::ExtractedSprites;

pub use {
    bundle::SpriteMaterialBundle, bundle::SpriteMaterialSheetBundle, sprite::SpriteMaterial,
    texture_atlas::TextureAtlasSpriteMaterial,
};

mod bundle;
mod extract;
mod sprite;
mod texture_atlas;

/// Plugin to use sprites with materials.
///
/// It requires both [`bevy_sprite::SpritePlugin`] and [`bevy_sprite::ColorMaterialPlugin`]
#[derive(Default)]
pub struct MaterialSpritePlugin;

impl Plugin for MaterialSpritePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                extract::extract_sprites.in_set(RenderSet::ExtractCommands),
            );
        }
    }
}
