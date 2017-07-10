use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Snake,
        color: [0.0, 0.50, 0.50, 1.0],
        properties: HashMap::new(),
        action: snake_action,
    }
}

fn snake_action(neighbors: &Neighbors) -> Vec<Action> {

    let empty_neighbors = neighbors.of_type(CreatureType::Empty);
    if empty_neighbors.is_empty() {
        return vec![Action::Idle];
    }

    let new_index = random_index(0, empty_neighbors.len());
    let new_pos = empty_neighbors[new_index].1;

    vec![Action::Set(new_pos, get(CreatureType::Snake))]
}