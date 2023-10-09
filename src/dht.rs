/// A humidity and temperature sensor of the DHT Family backed by RIOT's [driver implementation](https://doc.riot-os.org/group__drivers__dht.html)
#[derive(Debug)]
pub struct Dht {
    dev: riot_sys::dht_t,
}

#[derive(Debug, Default)]
pub struct DhtData {
    /// relative percent
    pub humidity: i16,
    /// temperature in deci-Celsius
    pub temperature: i16,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DhtError {
    /// Checksum error
    NoChecksum,
    /// Communication timed out
    Timeout,
    /// An unknown error occurred
    Unknown,
}

impl Dht {
    fn init(params: &riot_sys::inline::dht_params_t) -> Result<Self, DhtError> {
        let mut dev = riot_sys::dht_t::default();

        let res = unsafe {
            riot_sys::dht_init(
                &mut dev as *mut riot_sys::dht_t,
                params as *const _ as *const riot_sys::dht_params_t,
            )
        };

        match res {
            riot_sys::DHT_OK => Ok(Dht { dev }),
            riot_sys::DHT_NOCSUM => Err(DhtError::NoChecksum),
            riot_sys::DHT_TIMEOUT => Err(DhtError::Timeout),
            _ => Err(DhtError::Unknown),
        }
    }

    /// Creates and initializes a new Dht device using the default values
    /// specified in RIOT's driver implementation.
    pub fn new() -> Result<Self, DhtError> {
        let params = unsafe { riot_sys::macro_DHT_PARAMS() };
        Self::init(&params)
    }

    /// Creates and initializes a new Dht device using the specified gpio pin.
    pub fn new_with_pin(pin: crate::gpio::GPIO) -> Result<Self, DhtError> {
        let mut params = unsafe { riot_sys::macro_DHT_PARAMS() };
        params.pin = unsafe { riot_sys::macro_GPIO_PIN(0, pin.to_c()) };
        Self::init(&params)
    }

    /// Read the sensor values from the Dht Device.
    pub fn read(&mut self) -> Result<DhtData, DhtError> {
        let mut temp: i16 = 0;
        let mut hum: i16 = 0;

        let res = unsafe {
            riot_sys::dht_read(
                &mut self.dev as *mut riot_sys::dht_t,
                &mut temp as *mut i16,
                &mut hum as *mut i16,
            )
        };

        match res {
            riot_sys::DHT_OK => Ok(DhtData {
                humidity: hum,
                temperature: temp,
            }),
            riot_sys::DHT_NOCSUM => Err(DhtError::NoChecksum),
            riot_sys::DHT_TIMEOUT => Err(DhtError::Timeout),
            _ => Err(DhtError::Unknown),
        }
    }
}
