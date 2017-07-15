use ::lib::utils::*;
use ::lib::grid::Action;
use ::lib::creature::*;

const ENERGY_LOST_PER_TICK: &'static str = "energy_lost_per_tick";

pub fn new() -> Creature {
    let mut properties: HashMap<String, Property> = HashMap::new();
    properties.insert(String::from(ENERGY_LOST_PER_TICK), Property::Integer(1));

    Creature {
        creature_type: CreatureType::DeadPlant,
        color: [0.0, 0.45, 0.0, 1.0],
        energy: 2000,
        sleep: 0,
        properties: properties,
        action: dead_plant_action,
    }
}

fn dead_plant_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {

    let energy_lost_per_tick = get_variables(&myself.properties);

    if myself.energy <= 0 {
        return vec![Action::Set(neighbors.pos(), get(CreatureType::Plant))];
    } else {
        myself.energy -= random_int(0, 20) * energy_lost_per_tick;
        return vec![Action::Queue(neighbors.pos())];
    }
}

fn get_variables(properties: &HashMap<String, Property>) -> i64 {
    let energy_lost_per_tick: i64 = match properties.get(ENERGY_LOST_PER_TICK) {
        Some(&Property::Integer(n)) => n,
        _ => 1,
    };

    energy_lost_per_tick
}