use clap::Parser;
use std::fs;
use std::time::Instant;
mod bot;
mod mission;
use crate::bot::Bot;
use crate::mission::Mission;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    map: String,
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    start_money: i32,
    #[arg(short, long)]
    config: String,
}
//Hierarchy: Mission -> Waves -> Wavespawns -> Bots
//Test: cargo run -- -m mvm_decoy -n lol -s 10000 -c test
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut mission: Mission = Mission {
        ..Default::default()
    };
    let mut bots: Vec<Bot> = vec![];
    //lord have mercy on memory
    {
        let now = Instant::now();
        let map_config = fs::read_to_string("./config/maps.json")?;
        let map: serde_json::Value = serde_json::from_str(&map_config)?;
        let map_info = &map[&args.map];

        mission.bot_path_length = match map_info["bot_path_length"].as_f64() {
            None => 1200.0,
            Some(value) => value,
        };
        mission.spawn_bot_areas = match map_info["spawnbots"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        mission.spawn_giants_areas = match map_info["spawngiants"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        mission.spawn_boss_areas = match map_info["spawnbosses"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        mission.spawn_tank_areas = match map_info["spawntanks"].as_array() {
            None => vec!["spawnbot".to_string()],
            Some(value) => value
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect(),
        };

        mission.max_tank_speed = match map_info["max_tank_speed"].as_f64() {
            None => 300.0,
            Some(value) => value,
        };
        mission.engineers_enabled = match map_info["engies"].as_bool() {
            None => false,
            Some(value) => value,
        };
        println!("took {:?} to parse map config", now.elapsed());
    }
    {
        let now = Instant::now();

        let bot_config = fs::read_to_string("./config/bots.json")?;
        let bot_info_string: serde_json::Value = serde_json::from_str(&bot_config)?;
        let bot_infos = &bot_info_string.as_object().unwrap();
        for value in *bot_infos {
            let mut new_bot: Bot = Bot {
                ..Default::default()
            };
            new_bot.name = value.0.to_string();

            new_bot.class = match value.1["class"].as_str() {
                None => "".to_string(),
                Some(val) => val.to_string(),
            };
            new_bot.weapons = match value.1["weapons"].as_array() {
                None => vec![],
                Some(val) => val.iter().map(|x| x.as_str().unwrap().to_owned()).collect(),
            };
            new_bot.difficulty = match value.1["difficulty"].as_i64() {
                None => 1,
                Some(val) => val,
            };
            new_bot.weapon_restriction = match value.1["difficulty"].as_i64() {
                None => 1,
                Some(val) => val,
            };
            new_bot.bot_attributes = match value.1["bot_attributes"].as_array() {
                None => vec![],
                Some(val) => val.iter().map(|x| x.as_str().unwrap().to_owned()).collect(),
            };
            new_bot.health = match value.1["health"].as_str() {
                None => "".to_string(),
                Some(val) => val.to_string(),
            };
            new_bot.scale = match value.1["scale"].as_f64() {
                None => 1.0,
                Some(val) => val,
            };
            new_bot.max_vision_range = match value.1["scale"].as_i64() {
                None => 1200,
                Some(val) => val,
            };

            new_bot.class = match value.1["class"].as_str() {
                None => new_bot.class.to_owned(),
                Some(val) => val.to_string(),
            };

            new_bot.auto_jump_min = match value.1["auto_jump_min"].as_i64() {
                None => 0,
                Some(val) => val,
            };
            new_bot.auto_jump_max = match value.1["auto_jump_max"].as_i64() {
                None => 0,
                Some(val) => val,
            };

            new_bot.weight = match value.1["weight"].as_f64() {
                None => 1.0,
                Some(val) => val,
            };
            new_bot.rarity = match value.1["rarity"].as_i64() {
                None => 1,
                Some(val) => val,
            };

            new_bot.is_boss = match value.1["is_boss"].as_bool() {
                None => false,
                Some(val) => val,
            };
            new_bot.is_giant = match value.1["is_giant"].as_bool() {
                None => false,
                Some(val) => val,
            };

            new_bot.attributes = match value.1["attributes"].as_array() {
                None => vec![],
                Some(val) => val
                    .iter()
                    .map(|x| match x.as_array() {
                        Some(inner) => [
                            inner[0].as_str().unwrap().to_owned(),
                            inner[1].as_str().unwrap().to_owned(),
                        ],
                        None => panic!("WTF! - Failed to parse attributes for {}", new_bot.name),
                    })
                    .collect(),
            };
            bots.push(new_bot);
        }

        println!("took {:?} to parse bot config", now.elapsed());
    }
    {
        let now = Instant::now();
        let mission_config = fs::read_to_string("./config/".to_string() + &args.config)?;
        let mission_info: serde_json::Value = serde_json::from_str(&mission_config)?;

        if let Some(wave_amount) = mission_info.get("wave_amount") {
            mission.wave_amount = wave_amount.as_i64().unwrap();
        }
        if let Some(money_per_wave) = mission_info.get("money_per_wave") {
            mission.money_per_wave = money_per_wave.as_str().unwrap().to_owned();
        }
        if let Some(bot_giant_chance) = mission_info.get("bot_giant_chance") {
            mission.bot_giant_chance = bot_giant_chance.as_f64().unwrap();
        }
        if let Some(bot_boss_waves) = mission_info.get("bot_boss_waves") {
            mission.bot_boss_waves = bot_boss_waves.as_i64().unwrap();
        }
        if let Some(bot_superboss_waves) = mission_info.get("bot_superboss_waves") {
            mission.bot_superboss_waves = bot_superboss_waves.as_i64().unwrap();
        }
        if let Some(bot_damage_outgoing) = mission_info.get("bot_damage_outgoing") {
            mission.bot_damage_outgoing = bot_damage_outgoing.as_str().unwrap().to_owned();
        }
        if let Some(bot_damage_incoming) = mission_info.get("bot_damage_incoming") {
            mission.bot_damage_incoming = bot_damage_incoming.as_str().unwrap().to_owned();
        }
        if let Some(bot_speed_multiplier) = mission_info.get("bot_speed_multiplier") {
            mission.bot_speed_multiplier = bot_speed_multiplier.as_str().unwrap().to_owned();
        }

        println!("took {:?} to parse mission config", now.elapsed());
    }

    Ok(())
}
