use crate::bot::Bot;
#[derive(Clone)]
pub struct Wavespawn {
    //Information loaded from configs
    pub squads: Vec<Bot>,
    pub tags: Vec<String>,
    pub weight: f64,
    pub rarity: f64,
    pub spawn_tank: bool,
    pub is_support: bool,
}

