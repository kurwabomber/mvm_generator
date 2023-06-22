use crate::bot::Bot;

pub struct Wavespawn {
    //Information loaded from configs
    pub squads: Vec<Bot>,
    pub tags: Vec<String>,
    pub weight: i64,
    pub rarity: i64,
}

