use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use std::convert::{TryFrom, Into};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError, LinuxI2CMessage};

pub enum RegisterAddress {
    Status,
    MeasMode,
    AlgResultData,
    RawData,
    EnvData,
    Ntc,
    Thresholds,
    Baseline,
    HwId,
    HwVersion,
    FwBootVersion,
    FwAppVersion,
    ErrorId,
    SwReset
}   

impl Into<u8> for RegisterAddress {
    fn into(self) -> u8 { 
        match self {
            RegisterAddress::Status => 0x0,
            RegisterAddress::MeasMode => 0x1,
            RegisterAddress::AlgResultData => 0x2,
            RegisterAddress::RawData => 0x3,
            RegisterAddress::EnvData => 0x5,
            RegisterAddress::Ntc => 0x6,
            RegisterAddress::Thresholds => 0x10,
            RegisterAddress::Baseline => 0x11,
            RegisterAddress::HwId => 0x20,
            RegisterAddress::HwVersion => 0x21,
            RegisterAddress::FwBootVersion => 0x23,
            RegisterAddress::FwAppVersion => 0x24,
            RegisterAddress::ErrorId => 0xE,
            RegisterAddress::SwReset => 0xFF,
        }
    }
}

pub struct Ccs811_Sensor {
    addr: u16,
    wak_pin: u16,
    device: LinuxI2CDevice
}

impl Debug for Ccs811_Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.debug_struct("CCS811")
        .field("addr", &self.addr)
        .field("wak_pin", &self.wak_pin)
        .finish()
    }
}

#[derive(Debug)]
pub enum SensorError {
    i2c_error(LinuxI2CError)
}

impl Ccs811_Sensor {
    pub fn init(addr: u16, wak_pin: u16) -> Result<Self, SensorError> {
        let device = LinuxI2CDevice::new("/dev/i2c-1", addr).map_err(|x| SensorError::i2c_error(x))?;
        
        
        Ok(Ccs811_Sensor {
            addr,
            wak_pin,
            device
        })
    }

    pub fn status(&mut self) -> Result<u8, SensorError> {
        self.read(RegisterAddress::Status)
    }

    fn read(&mut self, address: RegisterAddress) -> Result<u8, SensorError> {
        let mut read_data = [0; 1];
        let mut transaction = [
            LinuxI2CMessage::write(&[address.into()]),
            LinuxI2CMessage::read(&mut read_data)
        ];

        println!("Sending transactoin");

        self.device.transfer(&mut transaction).map_err(SensorError::i2c_error)?;

        Ok(read_data[0])
    }
}