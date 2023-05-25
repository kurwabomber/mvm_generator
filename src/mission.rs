pub struct Mission{
    pub bot_path_length: f64,
    pub spawn_bot_areas: Vec<&'static str>,
    pub spawn_giants_areas: Vec<&'static str>,
    pub spawn_boss_areas: Vec<&'static str>,
    pub spawn_tank_areas: Vec<&'static str>,
    pub max_tank_speed: f64,
    pub engineers_enabled: bool,
}
impl Default for Mission{
    fn default() -> Self {
        Mission { 
            bot_path_length: 0.5,
            spawn_bot_areas: vec!["spawnbot"],
            spawn_giants_areas: vec!["spawnbot"],
            spawn_boss_areas: vec!["spawnbot"],
            spawn_tank_areas: vec!["boss_path_a1"],
            max_tank_speed: 500.0,
            engineers_enabled: false,
        }
    }
}