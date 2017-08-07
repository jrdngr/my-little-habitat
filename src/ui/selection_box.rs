use ::lib::organisms;
use ::lib::organisms::OrganismType;
use ::lib::type_aliases::*;

pub struct SelectionBox {
    pub organism_type: OrganismType,
    pub color: Color,
}

pub fn get_buttons() -> Vec<SelectionBox> {
    let mut buttons: Vec<SelectionBox> = Vec::new();
    
    buttons.push(
        SelectionBox {
            organism_type: OrganismType::Plant,
            color: organisms::plant::new_plant().color,
        }
    );

    buttons    
} 