pub struct Mission {
    /* General Map Info */
    pub bot_path_length: f64,
    pub spawn_bot_areas: Vec<String>,
    pub spawn_giants_areas: Vec<String>,
    pub spawn_boss_areas: Vec<String>,
    pub spawn_tank_areas: Vec<String>,
    pub max_tank_speed: f64,
    pub engineers_enabled: bool,
    /* Mission Specific Info */
    pub wave_amount: i64,
    pub money_per_wave: String,       //Evaluated
    pub bot_giant_chance: f64,        //0.0 -> 1.0 chance
    pub bot_boss_waves: i64,          //Every x waves has a boss.
    pub bot_superboss_waves: i64,     //Every x waves has a superboss.
    pub bot_damage_outgoing: String,  //Evaluated
    pub bot_damage_incoming: String,  //Evaluated
    pub bot_speed_multiplier: String, //Evaluated
}
impl Default for Mission {
    fn default() -> Self {
        Mission {
            bot_path_length: 0.5,
            spawn_bot_areas: vec!["spawnbot".to_string()],
            spawn_giants_areas: vec!["spawnbot".to_string()],
            spawn_boss_areas: vec!["spawnbot".to_string()],
            spawn_tank_areas: vec!["boss_path_a1".to_string()],
            max_tank_speed: 500.0,
            engineers_enabled: false,
            wave_amount: 6,
            money_per_wave: "100*wave".to_string(),
            bot_giant_chance: 0.1,
            bot_boss_waves: 3,
            bot_superboss_waves: 5,
            bot_damage_outgoing: "1.0".to_string(),
            bot_damage_incoming: "1.0".to_string(),
            bot_speed_multiplier: "1.0".to_string(),
        }
    }
}
