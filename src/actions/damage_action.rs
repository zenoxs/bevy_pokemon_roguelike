use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::PieceDeathEvent,
    stats::{Health, Stats},
};

use super::{death_action::DeathAction, orient_entity, Action};

#[derive(Debug, Clone)]
pub struct DamageAction {
    pub attacker: Entity,
    pub target: Entity,
    pub value: i32,
}

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        let Some(mut health) = world.get_mut::<Health>(self.target) else {
            return Err(());
        };

        health.value = health.value.saturating_sub(self.value);

        let mut next_actions = vec![];
        if health.is_dead() {
            world.send_event(PieceDeathEvent {
                entity: self.target,
            });
            next_actions.push(Box::new(DeathAction {
                target: self.target,
                attacker: self.attacker,
            }) as Box<dyn Action>);
        }

        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;

        orient_entity(world, self.target, attacker_position.0);

        Ok(next_actions)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, world: &mut World) -> bool {
        return world.get::<Health>(self.target).is_some();
    }
}
