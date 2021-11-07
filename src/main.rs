mod config;
mod data;
mod temperature_humidity;
mod ccs811;
mod util;
use config::{AppConfig, ApplicationError, ApplicationState};
use envconfig::Envconfig;
use std::time;
use temperature_humidity::EnvironmentSensor;
use ccs811::Ccs811_Sensor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config: AppConfig = AppConfig::init_from_env().unwrap();
    let mut app_state = ApplicationState::default();
    let sensor = EnvironmentSensor::init(config.dht22_pin).unwrap();
    let mut ccs811 = Ccs811_Sensor::init(config.ccs811_i2c_addr, config.ccs811_wak_pin).unwrap();

    let status = ccs811.status().unwrap();

    println!("ccs811 status: {}", status);

    loop {
        match sensor.read_env_data().await {
            Ok(data) => {
                println!(
                    "{:?} - {}",
                    data.into_farenheit(),
                    chrono::offset::Local::now()
                );
                app_state.remove_error(ApplicationError::DHT22_TIMEOUT)
            }
            Err(e) => {
                println!("Failed read DHT22 sensor data {:?}", e);
                app_state.add_error(ApplicationError::DHT22_TIMEOUT)
            }
        }
        tokio::time::sleep(time::Duration::from_secs(10)).await;
    }
}
