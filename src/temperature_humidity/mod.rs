use crate::util::vec_to_int;
use futures::stream::StreamExt;
use gpio_cdev::{
    AsyncLineEventHandle, Chip, EventRequestFlags, EventType, Line, LineEvent, LineRequestFlags,
};
use itertools::Itertools;
use std::future::Future;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};
use tokio;

#[derive(Debug, PartialEq)]
pub struct EnvironmentData {
    temp: f32,
    humidity: f32,
}

#[derive(Debug)]
pub struct EnvironmentSensor {
    gpio_pin: u32,
    line: Line,
}

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    BadChecksum,
    UnexpectedInput,
}

#[derive(Debug)]
pub enum SensorError {
    InvalidAck,
    BadRead,
    GpioError(gpio_cdev::Error),
    TimeoutError,
}

#[derive(Debug)]
pub enum ReadError {
    Sensor(SensorError),
    Conversion(ConversionError),
}

impl EnvironmentSensor {
    pub fn init(gpio_pin: u32) -> Result<Self, SensorError> {
        let mut chip = Chip::new("/dev/gpiochip0").map_err(|e| SensorError::GpioError(e))?;
        let line = chip
            .get_line(gpio_pin)
            .map_err(|e| SensorError::GpioError(e))?;

        Ok(EnvironmentSensor { gpio_pin, line })
    }

    pub async fn read_env_data(&self) -> Result<EnvironmentData, ReadError> {
        let data = match tokio::time::timeout(time::Duration::from_millis(150), self.read()).await {
            Ok(result) => result.map_err(|e| ReadError::Sensor(e)),
            Err(_) => Err(ReadError::Sensor(SensorError::TimeoutError)),
        }?;

        EnvironmentData::from_raw_output(&data).map_err(|x| ReadError::Conversion(x))
    }

    async fn read(&self) -> Result<Vec<u8>, SensorError> {
        // println!("Line Info: {:?}", self.line.info());

        Self::send_start_signal(&self.line).await.map_err(|e| SensorError::GpioError(e))?;

        let line_evt_handle = self
            .line
            .events(
                LineRequestFlags::INPUT,
                EventRequestFlags::BOTH_EDGES,
                "read-sensor-data",
            )
            .map_err(|e| SensorError::GpioError(e))?;

        let mut async_events =
            AsyncLineEventHandle::new(line_evt_handle).map_err(|e| SensorError::GpioError(e))?;

        match async_events.next().await.ok_or(SensorError::BadRead)? {
            Err(e) => Err(SensorError::GpioError(e)),
            Ok(e) if e.event_type() != EventType::FallingEdge => Err(SensorError::InvalidAck),
            Ok(_) => {
                let mut result: Vec<u8> = Vec::new();

                for _ in 0..40 {
                    let edge_one = async_events
                        .next()
                        .await
                        .ok_or(SensorError::BadRead)?
                        .map_err(|e| SensorError::GpioError(e))?;
                    let edge_two = async_events
                        .next()
                        .await
                        .ok_or(SensorError::BadRead)?
                        .map_err(|e| SensorError::GpioError(e))?;

                    let bit = Self::line_evt_tuple_to_bit((edge_one, edge_two));

                    result.push(bit);
                }
                Ok(result)
            }
        }
    }

    async fn send_start_signal(line: &Line) -> Result<(), gpio_cdev::Error> {
        // initialize sensor, [add link to docs]
        let handle = line.request(LineRequestFlags::OUTPUT, 1, "init-sequence")?;

        handle.set_value(0)?;
        tokio::time::sleep(time::Duration::from_millis(2)).await;
        Ok(())
    }

    fn line_evt_tuple_to_bit((edge_one, edge_two): (LineEvent, LineEvent)) -> u8 {
        // timestamps in nano seconds
        match edge_two.timestamp() - edge_one.timestamp() {
            x if x < 40000 => 0,
            _ => 1,
        }
    }
}

impl EnvironmentData {
    pub fn from_raw_output(output: &[u8]) -> Result<Self, ConversionError> {
        if output.len() != 40 {
            return Err(ConversionError::UnexpectedInput);
        }

        let data: Vec<u8> = output.chunks(8).map(vec_to_int).collect();

        Self::validate(&data)?;

        let result: Vec<u16> = data[0..4]
            .chunks(2)
            .map(|byte_pair| ((byte_pair[0] as u16) << 8) + (byte_pair[1] as u16))
            .collect();

        return Ok(EnvironmentData {
            humidity: result[0] as f32 / 10.0,
            temp: result[1] as f32 / 10.0,
        });
    }

    fn validate(converted: &[u8]) -> Result<(), ConversionError> {
        // compare reading with checksum. If they aren't equal
        // it was either a bad read or a malfunctioning sensor;

        let checksum = converted.last().ok_or(ConversionError::UnexpectedInput)?;
        let sum = converted[0..4]
            .into_iter()
            .fold(0 as u8, |sum, &byte| sum.overflowing_add(byte).0);

        if *checksum != sum {
            return Err(ConversionError::BadChecksum);
        }

        Ok(())
    }

    pub fn into_farenheit(&self) -> Self {
        EnvironmentData {
            temp: (self.temp * 1.8) + 32.0,
            humidity: self.humidity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use examples from spec sheet
    #[test]
    fn spec_sheet_expectations() {
        let input = vec![
            // humidity
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, // temperature
            0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, // checksum
            1, 1, 1, 0, 1, 1, 1, 0,
        ];

        assert_eq!(
            EnvironmentData::from_raw_output(&input).unwrap(),
            EnvironmentData {
                humidity: 65.2,
                temp: 35.1
            }
        );
    }

    #[test]
    fn bad_checksum() {
        let input = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
        ];

        assert_eq!(
            EnvironmentData::from_raw_output(&input),
            Err(ConversionError::BadChecksum)
        )
    }

    #[test]
    fn bad_input() {
        let input = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 1, 0,
        ];

        assert_eq!(
            EnvironmentData::from_raw_output(&input),
            Err(ConversionError::UnexpectedInput)
        )
    }
}
