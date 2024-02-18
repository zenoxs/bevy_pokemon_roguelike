use std::{collections::HashMap, sync::Arc};

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetApp, Assets, Handle},
    ecs::system::Res,
};
use bevy_egui::{
    egui::{self, mutex::Mutex},
    EguiContexts,
};
use fonts::{BitmapFont, BitmapFontData, BitmapFontLoader};

pub mod bfn;
pub mod fonts;

pub struct BitmapFontPlugin;

impl Plugin for BitmapFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BitmapFont>()
            .init_asset_loader::<BitmapFontLoader>()
            .add_systems(Update, font_texture_update);
    }
}

/// Bitmap font texture cache. Used internally, but may be useful for advanced users.
pub type BitmapFontCache = Arc<Mutex<HashMap<Handle<BitmapFont>, BitmapFontCacheItem>>>;

/// Record in the bitmap font texture cache. Used internally, but may be useful for advanced users.
#[derive(Clone)]
pub struct BitmapFontCacheItem {
    pub texture_id: egui::TextureId,
    pub font_data: Arc<BitmapFontData>,
}

/// Loop through all [`BitmapFont`] assets and map their texture ids and uvs to their handle
pub(crate) fn font_texture_update(fonts: Res<Assets<BitmapFont>>, mut egui_ctx: EguiContexts) {
    for (handle_id, font) in fonts.iter() {
        let texture_id = egui_ctx.add_image(font.data.texture.clone_weak());
        let handle = Handle::Weak(handle_id);

        let ctx = egui_ctx.ctx_mut();
        ctx.memory_mut(|ctx| {
            let mut bitmap_font_texture_datas = ctx
                .data
                .get_temp_mut_or_default::<BitmapFontCache>(egui::Id::NULL)
                .lock();

            let texture_data =
                bitmap_font_texture_datas
                    .entry(handle)
                    .or_insert_with(|| BitmapFontCacheItem {
                        texture_id,
                        font_data: font.data.clone(),
                    });
            if !Arc::ptr_eq(&texture_data.font_data, &font.data) {
                texture_data.font_data = font.data.clone();
            }
            texture_data.texture_id = texture_id;
        });
    }
}