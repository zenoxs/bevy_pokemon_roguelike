use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContexts};
use bitmap_font::fonts::BitmapFont;

use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::ui::sprite_text::{
    SpriteText, SpriteTextBundle, SpriteTextSection, SpriteTextStyle, Text2DSpriteBundle,
};
use crate::graphics::ui::{SpriteTextEguiUiExt, UISpriteText, UISpriteTextSection};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test)
            .add_systems(Update, ui.run_if(in_state(GameState::Playing)));
    }
}

fn ui(mut ctx: EguiContexts, font_assets: Res<FontAssets>) {
    let ctx = ctx.ctx_mut();
    egui::CentralPanel::default()
        // Because it covers the whole screen, make sure that it doesn't overlay the egui background frame
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            // Get the screen rect
            let screen_rect = ui.max_rect();
            // Calculate a margin of 15% of the screen size
            let outer_margin = screen_rect.size() * 0.15;
            let outer_margin = UiRect {
                left: Val::Px(outer_margin.x),
                right: Val::Px(outer_margin.x),
                // Make top and bottom margins smaller
                top: Val::Px(outer_margin.y / 2.0),
                bottom: Val::Px(outer_margin.y / 2.0),
            };

            // ui.label("world");
            egui::SidePanel::left("SidePanel").default_width(300.).show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    UISpriteText::from_sections([
                        UISpriteTextSection{
                            value: "Lorem ipsum".to_string(),
                            color: Color32::RED,
                            font: &font_assets.text
                        },
                        UISpriteTextSection{
                            value: " dolor sit amet,".to_string(),
                            color: Color32::WHITE,
                            font: &font_assets.text
                        },
                        UISpriteTextSection{
                            value: " consectetur adipiscing elit.".to_string(),
                            color: Color32::BLUE,
                            font: &font_assets.text
                        }
                    ]).show(ui);
                    ui.sprite_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", &font_assets.text);
                    // ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.");
                    // ui.label("World!");
                });
            });
            // egui::Grid::new("some_unique_id").show(ui, |ui| {
            //     ui.sprite_text("Hello ", &font_assets.text);
            //     ui.sprite_colored_label("World!", Color32::BLUE, &font_assets.text);
            //     ui.end_row();
            // });

            // ui.vertical_centered(|ui| {
            //     ui.retro_label("Hello world!", &font_assets.text);
            // });

            // ui.vertical_centered(|ui| {
            //     ui.retro_label("Hello World UI", &font_assets.text);
            // });
        });
}

fn spawn_test(font_assets: Res<FontAssets>, mut commands: Commands) {
    let text_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        ..default()
    };
    let text_red_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::RED,
        ..default()
    };
    let text_blue_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::hex("7098e3").unwrap(),
        ..default()
    };

    commands
        .spawn(Text2DSpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
            text_anchor: bevy::sprite::Anchor::BottomLeft,
            text: SpriteText {
                sections: [
                    SpriteTextSection::new("Lorem ipsum dolor sit amet, ", text_red_style.clone()),
                    SpriteTextSection::new("consectetur ", text_blue_style.clone()),
                    SpriteTextSection::new("adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", text_style.clone())
                ].to_vec(),
                ..default()
            },
            // text: SpriteText::from_section("Lorem ipsum dolor sit amet, ", text_style.clone()),
            text_2d_bounds: Text2dBounds {size: Vec2::new(200., 300.)},
            ..default()
        })
        .insert(Name::new("TextSprite Test"));

    // let box_size = Vec2::new(150.0, 80.0);
    // let box_position = Vec2::new(300., 35.0);
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(box_size.x, box_size.y)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(box_position.extend(20.)),
    //         ..default()
    //     })
    //     .insert(Name::new("Boxed SpriteText"))
    //     // .with_children(|builder| {
    //     //     builder.spawn(Text2dBundle {
    //     //         text: Text {
    //     //             sections: vec![TextSection::new(
    //     //                 "this text wraps in the box\n(Unicode linebreaks)",
    //     //                 TextStyle::default(),
    //     //             )],
    //     //             alignment: TextAlignment::Left,
    //     //             linebreak_behavior: BreakLineOn::WordBoundary,
    //     //         },
    //     //         text_2d_bounds: Text2dBounds {
    //     //             // Wrap text in the rectangle
    //     //             size: box_size,
    //     //         },
    //     //         // ensure the text is drawn on top of the box
    //     //         transform: Transform::from_translation(Vec3::Z),
    //     //         ..default()
    //     //     });
    //     // });
    //     .with_children(|builder| {
    //         builder.spawn(Text2DSpriteBundle {
    //             // text_anchor: bevy::sprite::Anchor::TopLeft,
    //             text: SpriteText {
    //                 sections: vec![
    //                     SpriteTextSection::new(
    //                         "this text wraps in the box (Unicode linebreaks) \nthis text wraps in the box (Unicode linebreaks)",
    //                         text_style.clone(),
    //                     ),
    //                     SpriteTextSection::new(
    //                         " Another text section",
    //                         text_style.clone(),
    //                     ),
    //                 ],
    //                 alignment: TextAlignment::Center,
    //                 linebreak_behavior: BreakLineOn::WordBoundary,
    //                 ..default()
    //             },
    //             text_2d_bounds: Text2dBounds {
    //                 // Wrap text in the rectangle
    //                 size: box_size,
    //             },
    //             // ensure the text is drawn on top of the box
    //             transform: Transform::from_translation(Vec3::Z),
    //             ..default()
    //         });
    //     });

    // Text with one section
    // ImageBundle
    // UI test
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             justify_content: JustifyContent::SpaceBetween,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Name::new("Node Text"),
    //             // Create a TextBundle that has a Text with a single section.
    //             SpriteTextBundle {
    //                 text: SpriteText::from_section("hello hello hello bevy!", text_style.clone()),
    //                 style: Style {
    //                     margin: UiRect::all(Val::VMin(3.)),
    //                     align_self: AlignSelf::FlexStart,
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //         ));

    //         // parent.spawn(ImageBundle {
    //         //     image: UiImage::new(asset_server.load("test.png")),
    //         //     background_color: Color::WHITE.into(),
    //         //     ..default()
    //         // });

    //         // parent.spawn((
    //         //     NodeBundle {
    //         //         style: Style {
    //         //             width: Val::Px(16.0),
    //         //             height: Val::Px(17.0),
    //         //             left: Val::Px(100.),
    //         //             margin: UiRect::top(Val::VMin(5.)),
    //         //             ..default()
    //         //         },
    //         //         // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
    //         //         background_color: Color::WHITE.into(),
    //         //         ..default()
    //         //     },
    //         //     UiImage::new(asset_server.load("test.png")),
    //         // ));

    //         // Set the alignment of the Text
    //         // .with_text_alignment(TextAlignment::Center)
    //         // // Set the style of the TextBundle itself.
    //         // .with_style(Style {
    //         //     position_type: PositionType::Absolute,
    //         //     bottom: Val::Px(5.0),
    //         //     right: Val::Px(5.0),
    //         //     ..default()

    //         // parent.spawn((
    //         //     ImageBundle {
    //         //         image: UiImage::new(asset_server.load("logo.png")),
    //         //         ..default()
    //         //     },
    //         //     // NodeBundle {
    //         //     //     style: Style {
    //         //     //         // width: Val::Px(500.0),
    //         //     //         // height: Val::Px(125.0),
    //         //     //         margin: UiRect::top(Val::VMin(5.)),
    //         //     //         ..default()
    //         //     //     },
    //         //     //     // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
    //         //     //     background_color: Color::WHITE.into(),
    //         //     //     ..default()
    //         //     // },
    //         //     // UiImage::new(asset_server.load("logo.png")),
    //         // ));
    //     });
}
