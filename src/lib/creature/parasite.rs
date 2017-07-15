use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Parasite,
        color: [1.0, 0.0, 0.0, 1.0],
        energy: 0,
        sleep: 0,
        properties: HashMap::new(),
        action: parasite_action,
    }
}

fn parasite_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {

    let plant_neighbors = neighbors.of_type(CreatureType::Plant);
    if plant_neighbors.is_empty() {
        return vec![Action::Clear(neighbors.pos())];
    }

    let parasite_neighbors = neighbors.of_type(CreatureType::Parasite);
    if parasite_neighbors.len() > 4 {
        return vec![Action::Clear(neighbors.pos())];
    }

    if parasite_neighbors.len() < 3 {
        let new_index = random_index(0, plant_neighbors.len());
        let new_pos = plant_neighbors[new_index].1;
        return vec![Action::Set(new_pos, get(CreatureType::Parasite)), Action::Queue(neighbors.pos())];
    }

    vec![Action::Idle, Action::Queue(neighbors.pos())]
}