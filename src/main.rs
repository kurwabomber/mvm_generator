use std::{fs::{File, self}, io::Read};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    map: String,
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    start_money: i32,
    #[arg(short, long)]
    waves: i32,
    #[arg(short, long)]
    respawnwavetime: i32,
    #[arg(short, long)]
    config: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //lord have mercy on memory
    {
        let map_config = fs::read_to_string("./config/maps.json")?;
        let map: serde_json::Value = serde_json::from_str(&map_config)?;
        let map_info = map.get(&args.map).unwrap();
        
    }
    {
        let weapon_config = fs::read_to_string("./config/weapons.json")?;
    }
    {
        let mission_config = fs::read_to_string("./config/".to_string() + &args.config)?;
    }

    Ok(())
}
