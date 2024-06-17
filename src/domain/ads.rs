use ads::{Client, AmsAddr, AmsNetId, AdsState, Handle};
use anyhow;
use crate::interfaces::ibattery_station::BatteryStation;
use std::time::Duration;

pub(crate) async fn get_battery_station(ip: &str) -> Result<BatteryStation, anyhow::Error> {
    let ip_suffix: Vec<_> = ip.split(".")
        .map(|s| s.parse().unwrap())
        .collect();
    let ams_net_id = AmsNetId::new(ip_suffix[0], ip_suffix[1], ip_suffix[2], ip_suffix[3], 1, 1);

    let client = match Client::new(
        (ip, ads::PORT),
        ads::Timeouts::new(Duration::from_secs(5)),
        ads::Source::Auto,
    ) {
        Ok(client) => client,
        Err(e) => {
            let station_id = ip_suffix[3] - 100;
            let icharger = BatteryStation::new(station_id, 0, 0, 0, 0, 0, 0, 0);
            println!("{:?}, {:?}", icharger,e);
            return Ok(icharger);
        }
    };

    let device = client.device(AmsAddr::new(ams_net_id, 801));
    let state = device.get_state()?;

    if state.0 == AdsState::Run {
        let ping_successful: u8 = 1;
        let station_id = ip_suffix[3] - 100;
        let plc_handle = Handle::new(device.clone(), ".OUT_PLC_OK")?.read_value::<u8>()?;
        let alarm_handle = Handle::new(device.clone(), ".OUT_Alarm")?.read_value::<u8>()?;
        let charger_enable_handle = Handle::new(device.clone(), ".OUT_CB_Enable")?.read_value::<u8>()?;
        let charge_percentage_handle = Handle::new(device.clone(), ".OUT_CurrentPercentage")?.read_value::<u8>()?;
        let lgv_aligned_handle = Handle::new(device.clone(), ".IN_LGV_Aligned_With_CB")?.read_value::<u8>()?;
        let emergency_handle = Handle::new(device.clone(), ".IN_Emergency")?.read_value::<u8>()?;
        let icharger = BatteryStation::new(
            station_id,
            ping_successful,
            plc_handle,
            alarm_handle,
            charger_enable_handle,
            charge_percentage_handle,
            lgv_aligned_handle,
            emergency_handle,
        );
        Ok(icharger)
    }else  {
        let ping_successful: u8 = 0;
        let station_id = ip_suffix[3] - 100;
        let icharger = BatteryStation::new(station_id, ping_successful, 0, 0, 0, 0, 0, 0);
        Err(anyhow::anyhow!("PLC is not in the run state"))
    }
}