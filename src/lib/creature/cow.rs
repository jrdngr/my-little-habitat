use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

const ENERGY_TO_SPLIT: &'static str = "energy_to_split";
const ENERGY_PER_PLANT: &'static str = "energy_per_plant";
const ENERGY_TO_MOVE: &'static str = "energy_to_move";
const WAIT_AFTER_MOVE: &'static str = "wait_after_move";
const WAIT_AFTER_SPLIT: &'static str = "wait_after_split";

pub fn new() -> Creature {
    let mut properties: HashMap<String, Property> = HashMap::new();
    properties.insert(String::from(ENERGY_TO_SPLIT), Property::Integer(100));
    properties.insert(String::from(ENERGY_PER_PLANT), Property::Integer(10));
    properties.insert(String::from(ENERGY_TO_MOVE), Property::Integer(5));
    
    properties.insert(String::from(WAIT_AFTER_MOVE), Property::Integer(2));
    properties.insert(String::from(WAIT_AFTER_SPLIT), Property::Integer(5));

    Creature {
        creature_type: CreatureType::Cow,
        color: [0.0, 0.0, 1.0, 1.0],
        energy: 50,
        sleep: 0,
        properties: properties,
        action: cow_action,
    }
}

fn cow_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {

    let (energy_to_split, energy_per_plant, energy_to_move, wait_after_move, wait_after_split) = get_variables(&myself.properties);

    if myself.energy <= 0 {
        return vec![Action::Clear(neighbors.pos()), Action::QueueNeighbors];
    }

    if myself.sleep > 0 || random_percentage(75) {
        myself.sleep -= 1;
        myself.energy -= 1;
        return vec![Action::Queue(neighbors.pos())];
    }
    
    let plant_neighbors = neighbors.of_type(CreatureType::Plant);
    if plant_neighbors.is_empty() {
        let empty_neighbors = neighbors.of_types(&[CreatureType::Empty, CreatureType::DeadPlant]);
        if empty_neighbors.is_empty() {
            myself.energy -= 1;
            return vec![Action::Queue(neighbors.pos())];
        } else {
            myself.energy -= energy_to_move;
            myself.sleep = wait_after_move;
            let new_index = random_index(0, empty_neighbors.len());
            let new_pos = empty_neighbors[new_index].1;
            return vec![Action::Set(new_pos, myself.clone()), Action::Clear(neighbors.pos()), Action::QueueNeighbors];
        }
    } else {
        let new_index = random_index(0, plant_neighbors.len());
        let new_pos = plant_neighbors[new_index].1;
        if myself.energy >= energy_to_split {
            myself.energy = 50;
            myself.sleep = wait_after_split;
            return vec![Action::Set(new_pos, myself.clone()), Action::Queue(neighbors.pos())];
        } else {
            myself.energy += energy_per_plant;
            myself.sleep = wait_after_move;
            return vec![Action::Set(new_pos, myself.clone()), Action::Set(neighbors.pos(), get(CreatureType::DeadPlant)), Action::QueueNeighbors];
        }
    }
}

fn get_variables(properties: &HashMap<String, Property>) -> (i64, i64, i64, i64, i64) {
    let energy_to_split: i64 = match properties.get(ENERGY_TO_SPLIT) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };
    let energy_per_plant: i64 = match properties.get(ENERGY_PER_PLANT) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };
    let energy_to_move: i64 = match properties.get(ENERGY_TO_MOVE) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };
    let wait_after_move: i64 = match properties.get(WAIT_AFTER_MOVE) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };
    let wait_after_split: i64 = match properties.get(WAIT_AFTER_SPLIT) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };

    (energy_to_split, energy_per_plant, energy_to_move, wait_after_move, wait_after_split)
}