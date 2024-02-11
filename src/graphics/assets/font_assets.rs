use std::time::Instant;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::get_path_from_handle;
use crate::GameState;

use super::AssetsLoading;

const FONTS_PATH: &str = "fonts";

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FontAssetsFolder(default()))
            .init_resource::<FontSheetAsset>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_font_assets);
    }
}

#[derive(Debug)]
pub struct GlyphData {
    pub index: usize,
    pub color_less: bool,
    pub size: UVec2,
}

#[derive(Resource, Debug, Default)]
pub struct FontSheetAsset(pub FontSheet);

#[derive(Debug, Default)]
pub struct FontSheet {
    pub texture_atlas: Handle<TextureAtlas>,
    pub characters: HashMap<u32, GlyphData>,
}

#[derive(Default, Resource)]
struct FontAssetsFolder(Handle<LoadedFolder>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut font_assets_folder: ResMut<FontAssetsFolder>,
) {
    info!("visual effect assets loading...");

    // Visual Effects
    let fonts_folder = asset_server.load_folder(FONTS_PATH);
    loading.0.push(fonts_folder.clone().untyped());
    font_assets_folder.0 = fonts_folder;
}

fn process_font_assets(
    font_assets_folder: Res<FontAssetsFolder>,
    mut font_sheet_assets: ResMut<FontSheetAsset>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    info!("font assets loading...");

    let folder: &LoadedFolder = match loaded_folder_assets.get(&font_assets_folder.0) {
        Some(folder) => folder,
        None => {
            error!("Couldn't load the visual effects folder");
            return;
        }
    };

    // Build a vector containing all the individual font image with their ID
    let mut texture_font_atlas_builder = TextureAtlasBuilder::default();
    let mut glyph_images: Vec<_> = folder
        .handles
        .par_iter() // Use parallel iterator here
        .filter_map(|handle| {
            let path = get_path_from_handle(handle)?;
            let file_stem = path.file_stem().and_then(|n| n.to_str())?;
            let glyph_id = u32::from_str_radix(file_stem, 16).ok()?;
            let glyph_handle = handle.to_owned().typed::<Image>();

            Some((glyph_id, glyph_handle))
        })
        .collect();

    let glyph_images: Vec<_> = glyph_images
        .drain(..)
        .filter_map(|(glyph_id, glyph_handle)| {
            let glyph_texture = textures.get(glyph_handle.id())?;
            texture_font_atlas_builder.add_texture(glyph_handle.id(), glyph_texture);

            Some((glyph_id, glyph_texture.size(), glyph_handle))
        })
        .collect();

    // Build the atlas
    let texture_atlas = texture_font_atlas_builder
        .finish(&mut textures)
        .expect("Unable to construct the font atlas");

    let mut characters: HashMap<u32, GlyphData> = HashMap::new();
    for (id, glyph_size, glyph_handle) in glyph_images.iter() {
        let index = texture_atlas.get_texture_index(glyph_handle.id()).unwrap();
        characters.insert(
            id.to_owned(),
            GlyphData {
                index,
                color_less: false, // TODO: read the data from the FontData.xml
                size: glyph_size.to_owned(),
            },
        );
    }

    info!("{} glyph added to the font assets", characters.len());

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    font_sheet_assets.0 = FontSheet {
        texture_atlas: texture_atlas_handle,
        characters,
    };

    // Clean up unused resources
    commands.remove_resource::<FontAssetsFolder>();
}
