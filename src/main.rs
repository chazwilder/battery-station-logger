mod db;
mod interfaces;
mod domain;
use tokio;
use dotenvy::dotenv;
use std::env;
use log::{info, error};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let stations_ip_env = env::var("BATTERY_IPS").expect("BATTERY_IPS environment variable not set");
    let stations: Vec<String> = stations_ip_env.to_owned().split(',').map(|s| s.to_owned()).collect();

    let mut tasks = Vec::new();

    for charger in stations.to_owned() {
        let task = tokio::spawn(async move {
            info!("Processing battery station: {:?}", charger);
            match domain::get_battery_station(&charger).await {
                Ok(battery_station) => {
                    info!("Battery station data: {:?}", battery_station);
                    if let Err(e) = db::connection::mssql(battery_station).await {
                        error!("Error inserting battery station data: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error getting battery station {}: {}", charger, e);
                    let ip_suffix: Vec<_> = charger.split(".")
                        .map(|s| s.parse::<u8>().unwrap())
                        .collect();
                    let battery_station = interfaces::ibattery_station::BatteryStation::new(ip_suffix[3]-100,
                                                                                            0,0,0,0,0,0,0);
                    if let Err(e) = db::connection::mssql(battery_station).await {
                        error!("Error inserting battery station data: {}", e);
                    }
                }
            }
        });
        tasks.push(task);
    }

    futures::future::join_all(tasks).await;
}