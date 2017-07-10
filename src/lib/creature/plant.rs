use ::lib::utils::*;
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

fn plant_action(neighbors: &Neighbors) -> Vec<Action> {

    let empty_neighbors = neighbors.of_type(CreatureType::Empty);
    if empty_neighbors.is_empty() {
        return vec![Action::Idle];
    }
    if random_percentage(10) {
        return vec![Action::Idle, Action::Queue(neighbors.pos())];
    }

    let new_index = random_index(0, empty_neighbors.len());
    let new_pos = empty_neighbors[new_index].1;

    vec![Action::Set(new_pos, get(CreatureType::Plant)), Action::Queue(neighbors.pos())]
}