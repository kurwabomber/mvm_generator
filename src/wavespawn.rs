use crate::bot::Bot;
#[derive(Clone)]
pub struct Wavespawn {
    //Information loaded from configs
    pub squads: Vec<Bot>,
    pub tags: Vec<String>,
    pub weight: i64,
    pub rarity: i64,
    pub spawn_tank: bool,
}

