pub mod plant;

pub use plant::Plant;

pub struct Energy(pub u32);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Position {
    pub x : f32,
    pub y: f32,
}
