use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

const ENERGY_PER_TICK: &'static str = "energy_per_tick";
const ENERGY_TO_SPLIT: &'static str = "energy_to_split";

pub fn new() -> Creature {
    let mut properties: HashMap<String, Property> = HashMap::new();
    properties.insert(String::from(ENERGY_PER_TICK), Property::Integer(1));
    properties.insert(String::from(ENERGY_TO_SPLIT), Property::Integer(100));
    
    Creature {
        creature_type: CreatureType::Plant,
        color: [0.0, 0.5, 0.0, 1.0],
        energy: 0,
        properties: properties,
        action: plant_action,
    }
}

fn plant_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {

    let energy_per_tick: i64 = match myself.properties.get(ENERGY_PER_TICK) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };
    let energy_to_split = match myself.properties.get(ENERGY_PER_TICK){
        Some(&Property::Integer(n)) => n,
        _ => 100,
    };

    if myself.energy < energy_to_split as u64 {
        if random_percentage(2) {
            myself.energy += energy_per_tick as u64;
        }
        return vec![Action::Idle, Action::Queue(neighbors.pos())];
    } else {
        let empty_neighbors = neighbors.of_type(CreatureType::Empty);
        if empty_neighbors.is_empty() {
            return vec![Action::Idle];
        }
        let new_index = random_index(0, empty_neighbors.len());
        let new_pos = empty_neighbors[new_index].1;

        vec![Action::Set(new_pos, get(CreatureType::Plant)), Action::Queue(neighbors.pos())]
    }

}