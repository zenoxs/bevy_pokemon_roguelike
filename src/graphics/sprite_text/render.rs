use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
use image::{DynamicImage, ImageBuffer, RgbaImage};

use crate::graphics::{assets::font_assets::FontSheet, sprite_text::utils::extract_sub_image};

use super::SpriteText;

pub(crate) fn render_texture(
    query: Query<(Entity, &SpriteText, &Text2dBounds), Changed<SpriteText>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    font_sheets: Res<Assets<FontSheet>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    for (entity, sprite_text, bounds) in query.iter() {
        for section in sprite_text.sections.iter() {
            let font_sheet = font_sheets
                .get(section.font.font_sheet.id())
                .expect("Unable to load the fontsheet for the font");

            let texture_atlas = texture_atlases
                .get(section.font.texture_atlas.id())
                .expect("Unable to load the texture atlas for the font");

            let texture_image = images.get(texture_atlas.texture.id()).unwrap();

            let mut total_width: f32 = 0.;
            let mut max_height: f32 = 0.;

            let glyphs: Vec<_> = section
                .value
                .chars()
                .map(|character| {
                    let glyph_id = character as u32;

                    if character == ' ' {
                        let space_image = ImageBuffer::new(5, 0);
                        total_width += space_image.width() as f32;
                        return (glyph_id, space_image);
                    }

                    // TODO: handle glyph not found
                    let glyph = font_sheet.glyphs.get(&glyph_id).unwrap();
                    let glyph_rect = texture_atlas.textures[glyph.index];

                    total_width += glyph_rect.width();
                    max_height = max_height.max(glyph_rect.height());

                    let glyph_image = extract_sub_image(texture_image, &glyph_rect)
                        .expect("Failed to extract sub-image");

                    (glyph_id, glyph_image)
                })
                .collect();

            let mut combined = RgbaImage::new(total_width as u32, max_height as u32);

            // Backgrounds
            // let red = Rgba([255, 0, 0, 255]);
            // for pixel in combined.pixels_mut() {
            //     *pixel = red;
            // }

            let mut x_offset: i64 = 0;
            for (_id, image) in glyphs {
                image::imageops::overlay(&mut combined, &image, x_offset, 0);
                x_offset += image.width() as i64;
            }

            let image = Image::from_dynamic(DynamicImage::ImageRgba8(combined), false);
            let image_handle = images.add(image);

            commands.entity(entity).insert((
                image_handle,
                Sprite {
                    custom_size: Some(Vec2::new(total_width, max_height)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
            ));
        }
    }
}
