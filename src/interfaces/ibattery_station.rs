use serde::{Deserialize, Serialize};
use derive_more::{Constructor};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Constructor)]
pub(crate) struct BatteryStation {
    pub station_id: u8,
    pub ping_successful: u8,
    pub plc_ok: u8,
    pub in_alarm: u8,
    pub charger_enabled: u8,
    pub current_percentage: u8,
    pub lgv_aligned: u8,
    pub emergency: u8
}