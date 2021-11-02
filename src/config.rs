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

#[derive(Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "DHT22_PIN", default = "13")]
    pub dht22_pin: u32,
    #[envconfig(from = "MISTER_PIN", default = "19")]
    pub mister_pin: u32,
    #[envconfig(from = "CO2_PIN", default = "10")]
    pub co2_pin: u32,
}
