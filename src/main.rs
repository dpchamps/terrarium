mod data;
mod temperature_humidity;
mod util;
use std::time;
use temperature_humidity::EnvironmentSensor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sensor = EnvironmentSensor::init(13).unwrap();

    loop {
        // println!("------------{}-----------", process::id());
        match sensor.read_env_data().await {
            Ok(data) => {
                println!(
                    "{:?} - {}",
                    data.into_farenheit(),
                    chrono::offset::Local::now()
                );
                tokio::time::sleep(time::Duration::from_secs(10)).await;
            }
            _ => {}
        }
        // println!("-----------------------");
        // tokio::time::sleep(time::Duration::from_secs(1)).await;
    }
}
