mod data;
mod temperature_humidity;
mod util;
use std::{thread, time, process};
use temperature_humidity::EnvironmentSensor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sensor = EnvironmentSensor::init(13).unwrap();

    // println!("Starting Read {:?}", time::Instant::now());
    // let result = sensor.read().unwrap();
    // println!(
    //     "Ending Read {:?}",
    //     time::Instant::now(),
    // );
    // let data = EnvironmentData::from_raw_output(&result).unwrap();
    // println!("Environment data: ${:?}", data.into_farenheit());\

    // println!("{:?}", sensor.read_env_data().unwrap().into_farenheit());

    loop {
        println!("------------{}-----------", process::id());
        match sensor.read_env_data().await {
            Ok(data) => println!("{:?}", data.into_farenheit()),
            Err(e) => println!("Failed to read with: {:?}", e),
        }
        println!("-----------------------");
        thread::sleep(time::Duration::from_secs(2));
    }
}
