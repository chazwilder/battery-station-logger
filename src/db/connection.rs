use futures::TryStreamExt;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use dotenvy::dotenv;
use std::env;
use crate::interfaces::ibattery_station::BatteryStation;


pub async fn mssql(bc: BatteryStation) -> Result<(), anyhow::Error> {
    dotenv().ok();
    let mut config = Config::new();
    config.host(env::var("SERVER_NAME").unwrap_or_default());
    config.database(env::var("DATABASE_NAME").unwrap_or_default());
    config.port(1433);
    config.authentication(AuthMethod::Integrated);
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let sql = "
        INSERT INTO BATTERY_CHARGER_LOG (
            STATION_ID,
            PING_SUCCESSFUL,
            PLC_OK,
            IN_ALARM,
            CHARGER_ENABLED,
            CURRENT_PERCENTAGE,
            LGV_ALIGNED,
            EMERGENCY
        )
        VALUES (
            @P1,
            @P2,
            @P3,
            @P4,
            @P5,
            @P6,
            @P7,
            @P8
        )";

    let result = client
        .execute(sql,&[
            &bc.station_id,
            &bc.ping_successful,
            &bc.plc_ok,
            &bc.in_alarm,
            &bc.charger_enabled,
            &bc.current_percentage,
            &bc.lgv_aligned,
            &bc.emergency,
        ]).await?;
    println!("{:?}", result);
    Ok(())
}