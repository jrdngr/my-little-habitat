extern crate rand;

use rand::Rng;

use ::lib::utils::*;
use ::lib::grid::Grid;
use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Plant,
        color: [0.0, 0.5, 0.0, 1.0],
        properties: HashMap::new(),
        action: plant_action,
    }
}

fn plant_action(neighbors: Neighbors) -> Vec<Action> {

    let empty_neighbors = neighbors.of_type(CreatureType::Empty);
    if empty_neighbors.len() <= 0 {
        return vec![Action::Idle];
    }

    let new_index = rand::thread_rng().gen_range(0, empty_neighbors.len());
    let new_pos = empty_neighbors[new_index].1;

    vec![Action::Set(new_pos, get(CreatureType::Plant))]
}