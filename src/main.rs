use std::{fs, path};
use clap::Parser;
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
        let map_info = map.get(&args.map).unwrap();

        let path_length = map_info.get("bot_path_length");
        if path_length != None{
            mission.bot_path_length = path_length.unwrap().as_f64().unwrap();
        }
    }
    {
        let weapon_config = fs::read_to_string("./config/weapons.json")?;
    }
    {
        let mission_config = fs::read_to_string("./config/".to_string() + &args.config)?;
    }

    Ok(())
}