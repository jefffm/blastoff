use hecs::{Entity, World};

use crate::{
    component::{Actor, ActorKind, Cardinal, Player, Position, Renderable, Viewshed},
    game::Action,
    input,
    map::Map,
    resource::Resources,
    scene::GameState,
    util::{WorldPoint, WorldVector},
};

use super::TurnsHistory;

/// The Actor System implements energy-based turn actions using Actor components
pub fn process_actors(
    world: &mut World,
    resources: &mut Resources,
    map: &Map,
    turn_history: &mut TurnsHistory,
) -> GameState {
    let mut actions: Vec<Action> = vec![];

    // Collect mut references to all the actors
    let mut actors: Vec<(Entity, &mut Actor)> =
        world.query_mut::<&mut Actor>().into_iter().collect();

    // Sort actors by their priority (ascending). This is a function of energy remaining + how many times they've been skipped
    actors.sort_by(|(_, a), (_, b)| a.priority().cmp(&b.priority()));
    // ...but we actually want descending order
    actors.reverse();

    tracing::trace!("found {:?} actors", actors.len());

    let mut needs_player_input = false;

    // Iterate over actors in priority-descending order
    'outer: for (entity, actor) in actors {
        tracing::trace!("Processing actor for turn {:?}", turn_history.steps);

        // Filter to only actors in the current turn
        // Increment the actor's turn counter (even if no action is taken/possible)
        if actor.energy() > 0 {
            let action: Option<Action> = match actor.kind() {
                // Handle Actors controlled by the player
                ActorKind::Player(inbox) => match inbox {
                    // If the player-controlled Actor entity has an action, use it
                    Some(next_action) => {
                        let next_action = match next_action {
                            input::PlayerAction::MoveWest => Action::Moves(entity, Cardinal::W),
                            input::PlayerAction::MoveEast => Action::Moves(entity, Cardinal::E),
                            input::PlayerAction::MoveNorth => Action::Moves(entity, Cardinal::N),
                            input::PlayerAction::MoveSouth => Action::Moves(entity, Cardinal::S),
                            input::PlayerAction::MoveNorthWest => {
                                Action::Moves(entity, Cardinal::NW)
                            }
                            input::PlayerAction::MoveNorthEast => {
                                Action::Moves(entity, Cardinal::NE)
                            }
                            input::PlayerAction::MoveSouthWest => {
                                Action::Moves(entity, Cardinal::SW)
                            }
                            input::PlayerAction::MoveSouthEast => {
                                Action::Moves(entity, Cardinal::SE)
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
                        tracing::trace!("Need player input for turn {:?}", turn_history.steps);
                        needs_player_input = true;
                        actor.increase_priority();
                        break 'outer;
                    }
                },
                ActorKind::Computer(inbox) => match inbox {
                    Some(next_action) => {
                        tracing::trace!("AI has action {:?}", next_action);
                        let next_action = *next_action;
                        actor.set_kind(ActorKind::Computer(None));

                        Some(next_action)
                    }
                    None => {
                        tracing::trace!("Need AI input for turn {:?}", turn_history.steps);
                        // Wait for the AI system to set something for this entity
                        actor.increase_priority();
                        break 'outer;
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
                None => break,
            }
        }
        actor.recover_energy();
    }

    ActionProcessor::new(world, resources, map).process_actions(&actions);
    turn_history.add_turn(actions);

    if needs_player_input {
        GameState::NeedPlayerInput
    } else {
        GameState::Ticking
    }
}

struct ActionProcessor<'a> {
    world: &'a mut World,
    resources: &'a mut Resources,
    map: &'a Map,
}

impl<'a> ActionProcessor<'a> {
    fn new(world: &'a mut World, resources: &'a mut Resources, map: &'a Map) -> Self {
        Self {
            world,
            resources,
            map,
        }
    }

    fn process_actions(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::Moves(entity, direction) => self.move_entity(entity, direction.to_vector()),
                Action::MovesBy(entity, vector) => self.move_entity(entity, vector),
                Action::Teleports(entity, point) => self.teleport_entity(entity, point),
                Action::Activates(_) => todo!(),
                Action::Noop => {}
            }
        }
    }

    fn move_entity(&mut self, entity: &Entity, vector: &WorldVector) {
        let source_point = {
            let mut query = self.world.query_one::<&Position>(*entity).unwrap();
            let position = query.get().unwrap();

            position.point()
        };

        let is_player = {
            let mut query = self.world.query_one::<&Player>(*entity).unwrap();
            query.get().is_some()
        };

        // TODO: clean up "off by one" point clamping for player movement
        let map_rect = self.map.get_rect();
        let dest_point =
            (source_point + *vector).clamp(map_rect.min(), map_rect.max() - WorldVector::new(1, 1));

        // Check if we're bumping into another actor. If they're hostile, melee attack instead. If they're friendly, swap spots with them(?)
        let mut position_swap = false;
        let query = self
            .world
            .query_mut::<(&Actor, &mut Position, &mut Renderable)>();
        for (other_entity, (_actor, position, renderable)) in query.into_iter() {
            if dest_point == position.point() {
                // if this isn't a player, prevent the move from happening
                if !is_player {
                    tracing::trace!(
                        "NPC would collide with {:?}. Skipping move for {:?} from {:?} to {:?}",
                        &other_entity,
                        &entity,
                        &source_point,
                        &dest_point
                    );
                    return;
                }
                renderable.set_move(position.point(), source_point);
                position.set_point(source_point);
                position_swap = true;
                break;
            }
        }

        if position_swap || !self.map.is_blocked(&dest_point) {
            // No entities in the way. Anything else?
            let (position, viewshed, renderable) = self
                .world
                .query_one_mut::<(&mut Position, &mut Viewshed, &mut Renderable)>(*entity)
                .unwrap();
            viewshed.set_dirty();
            renderable.set_move(position.point(), source_point);
            position.set_point(dest_point);
        }
    }

    fn teleport_entity(&mut self, entity: &Entity, point: &WorldPoint) {
        let (position, viewshed) = self
            .world
            .query_one_mut::<(&mut Position, &mut Viewshed)>(*entity)
            .expect("move entity exists");

        if &position.point() != point && !self.map.is_blocked(point) {
            viewshed.set_dirty();
            position.set_point(*point)
        }
    }
}
