use hecs::{Entity, World};

use crate::{
    component::{Actor, ActorKind, Cardinal, Position, Viewshed},
    game::{Action, RunState},
    input,
    resource::Resources,
    util::{WorldPoint, WorldVector},
};

/// The Actor System implements energy-based turn actions using Actor components
pub fn process_actors(world: &mut World, resources: &mut Resources) -> RunState {
    tracing::trace!("processing actors");
    let mut actions: Vec<Action> = vec![];

    // Collect mut references to all the actors
    let mut actors: Vec<(Entity, &mut Actor)> =
        world.query_mut::<&mut Actor>().into_iter().collect();

    // Sort actors by
    actors.sort_by(|(_, a), (_, b)| a.energy().cmp(&b.energy()));

    tracing::trace!("found {:?} actors", actors.len());

    // Find the minimum turn number. We're only going to process the minimum turn number each iteration.
    // Eventually, the minimum turn number will catch up and all entities will get another turn.
    resources.turn_number = match actors.is_empty() {
        true => 0,
        false => {
            let (_, actor) = actors
                .iter()
                .min_by(|(_, a), (_, b)| a.turns().cmp(&b.turns()))
                .expect("actor minimum");
            actor.turns()
        }
    };

    tracing::trace!("turn number {:?}", resources.turn_number);

    let mut needs_player_input = false;

    for (entity, actor) in actors {
        tracing::debug!("Processing actor for turn {:?}", resources.turn_number);

        // Filter to only actors in the current turn
        if actor.turns() <= resources.turn_number {
            // Increment the actor's turn counter (even if no action is taken/possible)
            match actor.energy() {
                0.. => {
                    let action: Option<Action> = match actor.kind() {
                        // Handle Actors controlled by the player
                        ActorKind::Player(inbox) => match inbox {
                            // If the player-controlled Actor entity has an action, use it
                            Some(next_action) => {
                                let next_action = match next_action {
                                    input::PlayerAction::MoveWest => {
                                        Action::Moves(entity, Cardinal::W)
                                    }
                                    input::PlayerAction::MoveEast => {
                                        Action::Moves(entity, Cardinal::E)
                                    }
                                    input::PlayerAction::MoveNorth => {
                                        Action::Moves(entity, Cardinal::N)
                                    }
                                    input::PlayerAction::MoveSouth => {
                                        Action::Moves(entity, Cardinal::S)
                                    }
                                    input::PlayerAction::PassTurn => Action::Noop,
                                };
                                // Reset the player actor
                                actor.set_kind(ActorKind::Player(None));

                                Some(next_action)
                            }
                            // If the player-controlled Actor does not have an action,
                            // skip over this entity without incrementing its turn counter
                            None => {
                                needs_player_input = true;
                                None
                            }
                        },
                        ActorKind::Computer(inbox) => match inbox {
                            Some(next_action) => {
                                let next_action = *next_action;
                                actor.set_kind(ActorKind::Computer(None));

                                Some(next_action)
                            }
                            None => {
                                // Wait for the AI system to set something for this entity
                                None
                            }
                        },
                    };

                    match action {
                        Some(action) => {
                            // TODO: the cost of an action should vary based on equipment, ability, and status
                            actor.use_energy(action.cost());
                            actor.take_turn();
                            actions.push(action)
                        }
                        // Actor has energy, but is unable to process an action without input (from systems or player input)
                        None => continue,
                    }
                }
                _ => {
                    actor.recover_energy();
                    actor.take_turn();
                }
            }
        }
    }

    ActionProcessor::new(world, resources).process_actions(&actions);
    resources.turn_history.add_turn(actions);

    if needs_player_input {
        RunState::GameAwaitingInput
    } else {
        RunState::GameSystems
    }
}

struct ActionProcessor<'a> {
    world: &'a mut World,
    resources: &'a mut Resources,
}

impl<'a> ActionProcessor<'a> {
    fn new(world: &'a mut World, resources: &'a mut Resources) -> Self {
        Self { world, resources }
    }

    fn process_actions(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::Moves(entity, direction) => self.move_entity(entity, direction),
                Action::Teleports(entity, point) => self.teleport_entity(entity, point),
                Action::Activates(_) => todo!(),
                Action::Noop => {}
            }
        }
    }

    fn move_entity(&mut self, entity: &Entity, direction: &Cardinal) {
        let (position, viewshed) = self
            .world
            .query_one_mut::<(&mut Position, &mut Viewshed)>(*entity)
            .expect("move entity exists");

        let source_point = position.point();

        // TODO: clean up "off by one" point clamping for player movement
        let map = self.resources.map.as_ref().expect("map");
        let map_rect = map.get_rect();
        let dest_point = (source_point + direction.to_vector())
            .clamp(map_rect.min(), map_rect.max() - WorldVector::new(1, 1));

        // TODO: move this logic somewhere (Map method?)
        if !map.is_blocked(&dest_point) && map[&dest_point].handler().is_passable() {
            viewshed.set_dirty();
            position.set_point(dest_point);
        }

        // TODO: create camera system to sync the camera with player movement
    }

    fn teleport_entity(&mut self, entity: &Entity, point: &WorldPoint) {
        let (position, viewshed) = self
            .world
            .query_one_mut::<(&mut Position, &mut Viewshed)>(*entity)
            .expect("move entity exists");

        let map = self.resources.map.as_ref().expect("map");
        if &position.point() != point && !map.is_blocked(point) {
            viewshed.set_dirty();
            position.set_point(*point)
        }
    }
}
