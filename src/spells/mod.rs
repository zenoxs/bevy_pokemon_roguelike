use std::ops::RangeInclusive;

use crate::graphics::anim_data::AnimKey;

#[derive(Debug, Clone)]
pub struct ProjectileSpell {
    pub visual_effect: &'static str,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Projectile(ProjectileSpell),
}

#[derive(Debug, Clone)]
pub struct SpellHit {
    pub visual_effect: &'static str,
    pub damage: i32,
}

#[derive(Debug, Clone)]
pub struct SpellCast {
    pub visual_effect: &'static str,
    pub animation: AnimKey,
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: &'static str,
    pub range: RangeInclusive<i32>,
    pub spell_type: SpellType,
    pub hit: SpellHit,
    pub cast: SpellCast,
}
