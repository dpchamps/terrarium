mod data;
mod temperature_humidity;
mod util;
use std::{process, thread, time};
use temperature_humidity::EnvironmentSensor;
use chrono;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sensor = EnvironmentSensor::init(13).unwrap();

    loop {
        // println!("------------{}-----------", process::id());
        match sensor.read_env_data().await {
            Ok(data) => {
                println!("{:?} - {}", data.into_farenheit(), chrono::offset::Local::now());
                tokio::time::sleep(time::Duration::from_secs(5)).await;
            },
            Err(_) => {
                tokio::time::sleep(time::Duration::from_millis(100)).await;
            },
        }
        // println!("-----------------------");
        
    }
}
