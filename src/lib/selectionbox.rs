use super::utils::*;
use super::creature::{CreatureType, get};

pub struct SelectionBox {
    pub creature_type: CreatureType,
    pub color: Color,
}

pub fn get_buttons() -> Vec<SelectionBox> {
    let mut buttons: Vec<SelectionBox> = Vec::new();
    
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Empty,
            color: get(CreatureType::Empty).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Wall,
            color: get(CreatureType::Wall).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Plant,
            color: get(CreatureType::Plant).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::DeadPlant,
            color: get(CreatureType::DeadPlant).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Cow,
            color: get(CreatureType::Cow).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Vine,
            color: get(CreatureType::Vine).color,
        }
    );
    buttons.push(
        SelectionBox {
            creature_type: CreatureType::Parasite,
            color: get(CreatureType::Parasite).color,
        }
    );

    buttons    
} 