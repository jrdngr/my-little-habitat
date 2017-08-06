use ::lib::organisms;
use ::lib::type_aliases::*;

pub struct SelectionBox {
    pub organism_id: String,
    pub color: Color,
}

pub fn get_buttons() -> Vec<SelectionBox> {
    let mut buttons: Vec<SelectionBox> = Vec::new();
    
    buttons.push(
        SelectionBox {
            organism_id: String::from(organisms::PLANT),
            color: organisms::plant::new_plant().color,
        }
    );

    buttons    
} 