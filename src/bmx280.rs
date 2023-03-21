/// A Bosch BMP280 or BME280 temperature, pressure and humidity sensor backed by RIOT's [driver implementation](https://doc.riot-os.org/group__drivers__bmx280.html)
#[derive(Debug)]
pub struct Bmx280 {
    dev: riot_sys::bmx280_t,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Bmx280Error {
    /// A bus error occurred
    Bus,
    /// Did not detect BME280 or BMP280 device
    NoDev,
    /// An unknown error occurred
    Unknown,
}


#[cfg(any(riot_module_bmp280_i2c, riot_module_bme280_i2c))]
fn set_dev(params: &mut riot_sys::inline::bmx280_params_t, index: u32) {
    params.i2c_dev = unsafe { riot_sys::macro_I2C_DEV(index) };
}

#[cfg(any(riot_module_bmp280_spi, riot_module_bme280_spi))]
fn set_dev(params: &mut riot_sys::inline::bmx280_params_t, index: u32) {
    params.spi = unsafe { riot_sys::macro_SPI_DEV(index) };
}

fn init(params: &riot_sys::inline::bmx280_params_t) -> Result<Bmx280, Bmx280Error> {
    let mut dev = riot_sys::bmx280_t::default();

    let res = unsafe {
        riot_sys::bmx280_init(
            &mut dev as *mut riot_sys::bmx280_t,
            params as *const _ as *const riot_sys::bmx280_params_t,
        )
    };
    match res {
        riot_sys::BMX280_OK => Ok(Bmx280 { dev }),
        riot_sys::BMX280_ERR_BUS => Err(Bmx280Error::Bus),
        riot_sys::BMX280_ERR_NODEV => Err(Bmx280Error::NoDev),
        _ => Err(Bmx280Error::Unknown),
    }
}

impl Bmx280 {
    /// Creates and initializes a new Bmx280 device using the default values
    /// specified in RIOT's driver implementation.
    pub fn new() -> Result<Self, Bmx280Error> {
        let params = unsafe { riot_sys::macro_BMX280_PARAMS() };
        init(&params)
    }

    /// Creates and initializes a new Bmx280 device.
    /// The `index` indicates which i2c or spi device from the current board should be used.
    /// For information on how many such devices are available for this board please
    /// refer to its RIOT documentation.
    pub fn new_with_dev_index(index: usize) -> Result<Self, Bmx280Error> {
        let mut params = unsafe { riot_sys::macro_BMX280_PARAMS() };
        set_dev(&mut params, index as u32);
        init(&params)
    }

    /// Read temperature value from the BMX280 device
    pub fn read_temperature(&mut self) -> Result<i16, Bmx280Error> {
        let res =
            unsafe { riot_sys::bmx280_read_temperature(&mut self.dev as *mut riot_sys::bmx280_t) };
        match res {
            i16::MIN => Err(Bmx280Error::Unknown),
            temperature => Ok(temperature),
        }
    }

    /// Read air pressure value from the BMX280 device.
    pub fn read_pressure(&mut self) -> Result<u32, Bmx280Error> {
        let res =
            unsafe { riot_sys::bmx280_read_pressure(&mut self.dev as *mut riot_sys::bmx280_t) };
        match res {
            u32::MAX => Err(Bmx280Error::Unknown),
            pressure => Ok(pressure),
        }
    }

    #[cfg(any(riot_module_bme280_spi, riot_module_bme280_i2c))]
    /// Read humidity value from the BME280 device.
    pub fn read_humidity(&mut self) -> Result<u16, Bmx280Error> {
        let res =
            unsafe { riot_sys::bme280_read_humidity(&mut self.dev as *mut riot_sys::bmx280_t) };
        Ok(res)
    }
}
