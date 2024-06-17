mod db;
mod interfaces;
mod domain;
use tokio;
use dotenvy::dotenv;
use std::env;
use rayon::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let stations_ip_env = env::var("BATTERY_IPS").ok().unwrap();
    let stations: Vec<&str> = stations_ip_env.split(',').collect();
    let mut records: Vec<interfaces::ibattery_station::BatteryStation> = Vec::new();
    for charger in stations {
        println!("{:?}", charger);
        match domain::get_battery_station(charger).await {
            Ok(battery_station) => {
                records.push(battery_station);
            }
            Err(e) => {
                eprintln!("Error getting battery station {}: {}", charger, e);
                let ip_suffix: Vec<_> = charger.split(".")
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect();
                let battery_station = interfaces::ibattery_station::BatteryStation::new(ip_suffix[3]-100,
                0,0,0,0,0,0,0);
                records.push(battery_station);
            }
        }
    }
    for charger in records {
        db::connection::mssql(charger).await.unwrap_or_default();
    }

}
