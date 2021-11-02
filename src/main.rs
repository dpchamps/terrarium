mod config;
mod data;
mod temperature_humidity;
mod util;
use config::{AppConfig, ApplicationError, ApplicationState};
use envconfig::Envconfig;
use std::time;
use temperature_humidity::EnvironmentSensor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config: AppConfig = AppConfig::init_from_env().unwrap();
    let mut app_state = ApplicationState::default();
    let sensor = EnvironmentSensor::init(config.dht22_pin).unwrap();

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
