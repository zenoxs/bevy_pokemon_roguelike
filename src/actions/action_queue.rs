use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{ActionQueueProcessedEvent, RunningAction},
    graphics::action_animation::ActionAnimationFinishedEvent,
    pieces::Health,
};

use super::{Action, ProcessingActionEvent};

#[derive(Debug, Clone)]
pub struct QueuedAction {
    pub entity: Entity,
    pub performable_actions: Vec<Box<dyn Action>>,
}

#[derive(Resource, Default, Clone)]
pub struct ActionQueue(pub VecDeque<QueuedAction>);

pub fn process_action_queue(world: &mut World, mut tracking_queue_animation: Local<u32>) {
    let mut ev_animation_finished = world.resource_mut::<Events<ActionAnimationFinishedEvent>>();
    let mut animation_finished_reader = ev_animation_finished.get_reader();

    let mut processed = false;
    for _ in animation_finished_reader.read(&ev_animation_finished) {
        if *tracking_queue_animation > 0 {
            *tracking_queue_animation = tracking_queue_animation.saturating_sub(1);

            if *tracking_queue_animation == 0 {
                processed = true;
            }
        }
        info!("tracking queue {:?}", tracking_queue_animation);
    }
    ev_animation_finished.clear();

    if *tracking_queue_animation > 0 {
        return;
    }

    'queue_action_loop: loop {
        // Get the first action of the queue
        let queued_action = {
            let mut action_queue = world.resource_mut::<ActionQueue>();
            if action_queue.0.is_empty() {
                // If the ActionQueue is empty, return
                break;
            }
            action_queue.0.pop_front()
        };

        let Some(queued_action) = queued_action else {
            // If there is no more action in the queue
            break;
        };

        world.send_event(ProcessingActionEvent);

        if let Ok(health) = world.query::<&Health>().get(world, queued_action.entity) {
            if health.is_dead() {
                info!("{:?} is dead ", queued_action.entity);
                continue;
            }
        }

        for (action_index, action) in queued_action.performable_actions.iter().enumerate() {
            match action.execute(world) {
                Ok(result_actions) => {
                    // Action well executed (insert the `RunningAction`)
                    info!("action executed {:?}", action);
                    *tracking_queue_animation += 1;
                    world
                        .entity_mut(queued_action.entity)
                        .insert(RunningAction(action.clone()));

                    if !result_actions.is_empty() {
                        let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
                        action_queue.0.push_front(QueuedAction {
                            entity: queued_action.entity,
                            performable_actions: result_actions,
                        });
                    }

                    if action.is_parallel_execution() {
                        // go to the next queue action
                        break;
                    }

                    // Avoid processing multiple action at the same time
                    break 'queue_action_loop;
                }
                Err(_) => {
                    warn!("Action not valid");
                    if action_index == queued_action.performable_actions.len() - 1 {
                        // Last performable action is also invalid
                        warn!("No more performable action");
                        break;
                    }
                }
            };
        }

        apply_deferred(world);
    }

    if processed {
        if world.resource::<ActionQueue>().0.is_empty() {
            info!("Action queue processed");
            world.send_event(ActionQueueProcessedEvent);
        } else {
            info!(
                "Action queue couldn't processed, actions remaining {:?}",
                world.resource::<ActionQueue>().0.len()
            );
        }
    }
}