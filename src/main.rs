use std::fs;
use clap::Parser;
use serde_json::Value;
mod mission;
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
    config: String
}
//Hierarchy: Mission -> Waves -> Wavespawns -> Bots
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut mission:Mission = Mission { ..Default::default() };
    //lord have mercy on memory
    {
        let map_config = fs::read_to_string("./config/maps.json")?;
        let map: serde_json::Value = serde_json::from_str(&map_config)?;
        let map_info = map.get(&args.map).unwrap().clone();

        //does anyone know a better way to do this...
        if let Some(path_length) = map_info.get("bot_path_length") {
            mission.bot_path_length = path_length.as_f64().unwrap();
        }
        if let Some(spawn_bot_areas) = map_info.get("spawnbots") {
            mission.spawn_bot_areas = spawn_bot_areas.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_owned()).collect();
        }
        if let Some(spawn_giants_areas) = map_info.get("spawngiants") {
            mission.spawn_giants_areas = spawn_giants_areas.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_owned()).collect();
        }
        if let Some(spawn_boss_areas) = map_info.get("spawnbosses") {
            mission.spawn_boss_areas = spawn_boss_areas.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_owned()).collect();
        }
        if let Some(spawn_tank_areas) = map_info.get("spawntanks") {
            mission.spawn_tank_areas = spawn_tank_areas.as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_owned()).collect();
        }
        if let Some(max_tank_speed) = map_info.get("max_tank_speed") {
            mission.max_tank_speed = max_tank_speed.as_f64().unwrap();
        }
        if let Some(engineers_enabled) = map_info.get("engies") {
            mission.engineers_enabled = engineers_enabled.as_bool().unwrap();
        }
    }
    {
        let weapon_config = fs::read_to_string("./config/weapons.json")?;
    }
    {
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
    }

    Ok(())
}