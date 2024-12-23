use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use bevy::{
    ecs::component::Component,
    math::{IVec2, Vec2},
};
use serde::{Deserialize, Serialize};

use crate::{anim_key::AnimKey, orientation::Orientation};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct IVec2Serialized {
    x: i32,
    y: i32,
}

impl From<IVec2Serialized> for IVec2 {
    fn from(def: IVec2Serialized) -> Self {
        IVec2 { x: def.x, y: def.y }
    }
}

impl From<IVec2> for IVec2Serialized {
    fn from(vec: IVec2) -> Self {
        IVec2Serialized { x: vec.x, y: vec.y }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CharAnimationOffsets {
    pub body: Vec2,  // Green
    pub head: Vec2,  // Black
    pub right: Vec2, // Blue
    pub left: Vec2,  // Red
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CharAnimationFileEntry {
    pub texture: Vec<u8>,
    pub index: usize,
    pub frame_width: u32,
    pub frame_height: u32,
    pub durations: Vec<u32>,
    pub is_single_orientation: bool,
    pub rush_frame: Option<usize>,
    pub hit_frame: Option<usize>,
    pub return_frame: Option<usize>,

    // Offsets
    pub shadow_offsets: HashMap<Orientation, Vec<Vec2>>,
    pub offsets: HashMap<Orientation, Vec<CharAnimationOffsets>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CharAnimationFile {
    pub anim: HashMap<AnimKey, CharAnimationFileEntry>,
}

impl CharAnimationFile {
    pub fn save(&self, file: &mut File) -> Result<(), io::Error> {
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn load(buffer: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (font_sheet, _): (CharAnimationFile, usize) =
            bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
        Ok(font_sheet)
    }
}
