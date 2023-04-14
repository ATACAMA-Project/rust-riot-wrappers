/// A DS18 temperature sensor backed by RIOT's [driver implementation](https://doc.riot-os.org/group__drivers__ds18.html)
#[derive(Debug)]
pub struct Ds18 {
    dev: riot_sys::ds18_t,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Ds18Error {
    /// An unknown error occurred
    Unknown,
}

impl Ds18 {
    fn init(params: &riot_sys::inline::ds18_params_t) -> Result<Self, Ds18Error> {
        let mut dev = riot_sys::ds18_t::default();

        let res = unsafe {
            riot_sys::ds18_init(
                &mut dev as *mut riot_sys::ds18_t,
                params as *const _ as *const riot_sys::ds18_params_t,
            )
        };
        match res as u32 {
            riot_sys::DS18_OK => Ok(Self { dev }),
            _ => Err(Ds18Error::Unknown),
        }
    }

    /// Creates and initializes a new DS18 device using the default values
    /// specified in RIOT's driver implementation.
    pub fn new() -> Result<Self, Ds18Error> {
        let params = unsafe { riot_sys::macro_DS18_PARAMS_DEFAULT() };
        Self::init(&params)
    }

    /// Creates and initializes a new DS18 device using the specified gpio pin
    /// for data transmission on the onewire bus. If with_pullup is true, the
    /// internal pullup resistor will be used when reading data.
    pub fn new_with_pin(pin: crate::gpio::GPIO, with_pullup: bool) -> Result<Self, Ds18Error> {
        let params = riot_sys::inline::ds18_params_t {
            pin: pin.to_c(),
            out_mode: match with_pullup {
                true => riot_sys::gpio_mode_t_GPIO_OD_PU,
                false => riot_sys::gpio_mode_t_GPIO_OUT,
            },
            in_mode: match with_pullup {
                true => riot_sys::gpio_mode_t_GPIO_IN_PU,
                false => riot_sys::gpio_mode_t_GPIO_IN,
            },
        };
        Self::init(&params)
    }

    /// Triggers a conversion and reads the temperature value.
    pub fn read_temperature(&self) -> Result<i16, Ds18Error> {
        let mut temp: i16 = 0;
        let res = unsafe {
            riot_sys::ds18_get_temperature(
                &self.dev as *const riot_sys::ds18_t,
                &mut temp as *mut i16,
            )
        };

        match res as u32 {
            riot_sys::DS18_OK => Ok(temp),
            _ => Err(Ds18Error::Unknown),
        }
    }
}
