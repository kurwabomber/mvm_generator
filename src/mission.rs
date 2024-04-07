use std::{time::Instant, fs};

pub struct Mission {
    /* General Map Info */
    pub bot_path_length: f64,
    pub spawn_bot_areas: Vec<String>,
    pub spawn_giants_areas: Vec<String>,
    pub spawn_boss_areas: Vec<String>,
    pub spawn_tank_areas: Vec<String>,
    pub max_tank_speed: f64,
    pub engineers_enabled: bool,
    pub gatebots_enabled: bool,
    /* Mission Specific Info */
    pub wave_amount: i64,
    pub starting_money: i64,
    pub money_per_wave: String,       //Evaluated
    pub bot_giant_chance: f64,        //0.0 -> 1.0 chance
    pub bot_boss_waves: i64,          //Every x waves has a boss.
    pub bot_superboss_waves: i64,     //Every x waves has a superboss.
    pub global_attributes: Vec<[String; 2]>,
    pub wavespawn_tags: Vec<String>,
    pub wavespawn_amount: i64,
    pub mission_name: String,
    pub tank_health_formula: String,
    pub rarity_formula: String,
}
impl Mission{
    pub fn parse_map_config(&mut self, selected_map: &String) {
        let now = Instant::now();
        let map_config = fs::read_to_string("./config/maps.json").unwrap();
        let map: serde_json::Value = serde_json::from_str(&map_config).unwrap();
        let map_info = &map[selected_map];

        self.bot_path_length = match map_info["bot_path_length"].as_f64() {
            None => 1200.0,
            Some(value) => value,
        };
        self.spawn_bot_areas = match map_info["spawnbots"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        self.spawn_giants_areas = match map_info["spawngiants"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        self.spawn_boss_areas = match map_info["spawnbosses"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        self.spawn_tank_areas = match map_info["spawntanks"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        self.max_tank_speed = match map_info["max_tank_speed"].as_f64() {
            None => 300.0,
            Some(value) => value,
        };
        self.engineers_enabled = match map_info["engies"].as_bool() {
            None => false,
            Some(value) => value,
        };
        self.gatebots_enabled = match map_info["gatebots"].as_bool() {
            None => false,
            Some(value) => value,
        };
        println!("took {:?} to parse map config", now.elapsed());
    }
    pub fn parse_mission_config(&mut self, selected_mission: &String){
        let now = Instant::now();
        let mission_config = fs::read_to_string("./config/missions/".to_string() + selected_mission).unwrap();
        let mission_info: serde_json::Value = serde_json::from_str(&mission_config).unwrap();

        if let Some(wave_amount) = mission_info.get("wave_amount") {
            self.wave_amount = wave_amount.as_i64().unwrap();
        }
        if let Some(starting_money) = mission_info.get("starting_money") {
            self.starting_money = starting_money.as_i64().unwrap();
        }
        if let Some(money_per_wave) = mission_info.get("money_per_wave") {
            self.money_per_wave = money_per_wave.as_str().unwrap().to_owned();
        }
        if let Some(bot_giant_chance) = mission_info.get("bot_giant_chance") {
            self.bot_giant_chance = bot_giant_chance.as_f64().unwrap();
        }
        if let Some(bot_boss_waves) = mission_info.get("bot_boss_waves") {
            self.bot_boss_waves = bot_boss_waves.as_i64().unwrap();
        }
        if let Some(bot_superboss_waves) = mission_info.get("bot_superboss_waves") {
            self.bot_superboss_waves = bot_superboss_waves.as_i64().unwrap();
        }
        self.global_attributes = match mission_info["attributes"].as_array() {
            None => vec![],
            Some(val) => val
                .iter()
                .map(|x| match x.as_array() {
                    Some(inner) => [
                        inner[0].as_str().unwrap().to_owned(),
                        inner[1].as_str().unwrap().to_owned(),
                    ],
                    None => panic!("WTF! - Failed to parse global attributes for this mission."),
                })
                .collect(),
        };
        self.wavespawn_tags = match mission_info["allowed_tags"].as_array() {
            None => vec!["normal".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };
        if let Some(mission_name) = mission_info.get("mission_name") {
            self.mission_name = mission_name.as_str().unwrap().to_owned();
        }
        if let Some(wavespawn_amount) = mission_info.get("wavespawn_amount"){
            self.wavespawn_amount = wavespawn_amount.as_i64().unwrap();
        }
        if let Some(tank_health_formula) = mission_info.get("tank_health_formula"){
            self.tank_health_formula = tank_health_formula.as_str().unwrap().to_owned();
        }
        if let Some(rarity_formula) = mission_info.get("rarity_formula"){
            self.rarity_formula = rarity_formula.as_str().unwrap().to_owned();
        }
        println!("took {:?} to parse mission config", now.elapsed());
    }

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
            starting_money: 400,
            money_per_wave: "100*wave".to_string(),
            bot_giant_chance: 0.1,
            bot_boss_waves: 3,
            bot_superboss_waves: 5,
            global_attributes: vec![],
            wavespawn_tags: vec!["normal".to_string()],
            wavespawn_amount: 6,
            mission_name: "".to_string(),
            tank_health_formula: "10000 * 1.2^(wave-1)".to_string(),
            gatebots_enabled: false,
            rarity_formula: "1".to_string()
        }
    }
}