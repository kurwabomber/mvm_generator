pub struct Bot{
    /* General Map Info */
    pub name: String,
    pub class: String,
    pub weapons: Vec<String>,
    pub difficulty: i64,
    pub behavior: String,
    pub weapon_restriction: i64,
    pub bot_attributes: Vec<String>,
    pub health: String,
    pub scale: f64,
    pub max_vision_range: i64,
    pub class_icon: String,
    pub auto_jump_min: i64,
    pub auto_jump_max: i64,
    pub weight: f64,
    pub rarity: i64,
    pub attributes: Vec<[String;2]>,
    pub is_boss: bool,
    pub is_giant: bool,
}
impl Default for Bot{
    fn default() -> Self {
        Bot { 
            name: "Scout".to_string(),
            class: "scout".to_string(),
            weapons: vec![],
            difficulty: 1,
            behavior: "".to_string(),
            weapon_restriction: -1,
            bot_attributes: vec![],
            health: "125".to_string(),
            scale: 1.0,
            max_vision_range: 1200,
            class_icon: "scout".to_string(),
            auto_jump_min: 0,
            auto_jump_max: 0,
            weight: 1.0,
            rarity: 1,
            attributes: vec![],
            is_boss: false,
            is_giant: false,
        }
    }
}