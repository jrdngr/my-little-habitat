use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Ivy,
        color: [0.8, 1.0, 0.4, 1.0],
        energy: 0,
        properties: HashMap::new(),
        action: ivy_action,
    }
}

fn ivy_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {

    let empty_neighbors = neighbors.of_type(CreatureType::Empty);
    if empty_neighbors.is_empty() {
        return vec![Action::Idle];
    }

    let new_index = random_index(0, empty_neighbors.len());
    let new_pos = empty_neighbors[new_index].1;

    vec![Action::Set(new_pos, get(CreatureType::Ivy))]
}