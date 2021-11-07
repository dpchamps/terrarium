use envconfig::Envconfig;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
pub enum ApplicationError {
    DHT22_TIMEOUT,
}

const DHT22_TIMEOUT: ApplicationError = ApplicationError::DHT22_TIMEOUT;

// This tracks runtime, mutable values
pub struct ApplicationState {
    target_temp: u32,
    target_humidity: u32,
    target_co2: u32,
    application_errors: HashSet<ApplicationError>,
}

impl ApplicationState {
    pub fn add_error(&mut self, error: ApplicationError) {
        match error {
            ApplicationError::DHT22_TIMEOUT => self.application_errors.insert(DHT22_TIMEOUT),
        };
    }

    pub fn remove_error(&mut self, error: ApplicationError) {
        match error {
            ApplicationError::DHT22_TIMEOUT => self.application_errors.remove(&DHT22_TIMEOUT),
        };
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            target_temp: 0,
            target_humidity: 0,
            target_co2: 0,
            application_errors: HashSet::new(),
        }
    }
}

/** Immutable config needed for app startup */
#[derive(Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "DHT22_PIN", default = "13")]
    pub dht22_pin: u32,
    #[envconfig(from = "MISTER_PIN", default = "19")]
    pub mister_pin: u32,
    #[envconfig(from = "CCS811_I2C_ADDR", default = "90" /* 0x5A = 90 */)]
    pub ccs811_i2c_addr: u16,
    #[envconfig(from = "CCS811_WAK_PIN", default = "6")]
    pub ccs811_wak_pin: u16,
    #[envconfig(from = "SDA_PIN", default = "2")]
    pub sda_pin: u16,
    #[envconfig(from = "SCL_PIN", default = "3")]
    pub scl_pin: u16
}
