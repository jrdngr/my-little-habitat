use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Cow,
        color: [0.0, 0.0, 1.0, 1.0],
        properties: HashMap::new(),
        action: cow_action,
    }
}

fn cow_action(neighbors: &Neighbors) -> Vec<Action> {

    let plant_neighbors = neighbors.of_type(CreatureType::Plant);
    if plant_neighbors.is_empty() {
        if random_percentage(25) {
            let empty_neighbors = neighbors.of_type(CreatureType::Empty);
            if !empty_neighbors.is_empty() {
                let new_index = random_index(0, empty_neighbors.len());
                let new_pos = empty_neighbors[new_index].1;
                return vec![Action::Set(new_pos, get(CreatureType::Cow)), Action::Clear(neighbors.pos()), Action::QueueNeighbors];
            } else {
                return vec![Action::Clear(neighbors.pos()), Action::QueueNeighbors];
            }

        } else {
            return vec![Action::Clear(neighbors.pos()), Action::QueueNeighbors];
        }
    } else {
        let new_index = random_index(0, plant_neighbors.len());
        let new_pos = plant_neighbors[new_index].1;
        let mut result = vec![Action::Set(new_pos, get(CreatureType::Cow))]; 
        if random_percentage(35) {
            result.push(Action::Clear(neighbors.pos()));
            result.push(Action::QueueNeighbors);
        }
        else {
            result.push(Action::Queue(neighbors.pos()));
        }
        result
    }
}