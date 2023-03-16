use core::mem::MaybeUninit;

/// A Tcs37727 Color light-to-digital converter backed by RIOT's [driver implementation](https://doc.riot-os.org/group__drivers__tcs37727.html)
#[derive(Debug)]
pub struct Tcs37727 {
    dev: riot_sys::tcs37727_t,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Tcs37727Data {
    /// IR compensated channels red
    pub red: u32,
    /// IR compensated channels green
    pub green: u32,
    /// IR compensated channels blue
    pub blue: u32,
    /// channels clear
    pub clear: u32,
    /// Lux
    pub lux: u32,
    /// Color temperature
    pub ct: u32,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Tcs37727Error {
    /// Access to the configured I2C bus failed
    NoBus,
    /// No TCS37727 device found on the bus
    NoDev,
    /// An unknown error occurred
    Unknown,
}

fn init(params: &riot_sys::inline::tcs37727_params_t) -> Result<Tcs37727, Tcs37727Error> {
    let mut dev = riot_sys::tcs37727_t::default();

    let res = unsafe {
        riot_sys::tcs37727_init(
            &mut dev as *mut riot_sys::tcs37727_t,
            params as *const _ as *const riot_sys::tcs37727_params_t,
        )
    };
    match res {
        riot_sys::TCS37727_OK => Ok(Tcs37727 { dev }),
        riot_sys::TCS37727_NOBUS => Err(Tcs37727Error::NoBus),
        riot_sys::TCS37727_NODEV => Err(Tcs37727Error::NoDev),
        _ => Err(Tcs37727Error::Unknown),
    }
}

impl Tcs37727 {
    /// Creates and initializes a new Tcs37727 device.
    pub fn new() -> Result<Self, Tcs37727Error> {
        let params = unsafe { riot_sys::macro_TCS37727_PARAMS() };
        init(&params)
    }

    /// Creates and initializes a new Tcs37727 device.
    /// The `index` indicates which i2c device from the current board should be used.
    /// For information on how many such devices are available for this board please
    /// refer to its RIOT documentation.
    pub fn new_with_dev_index(index: usize) -> Result<Self, Tcs37727Error> {
        let mut params = unsafe { riot_sys::macro_TCS37727_PARAMS() };
        params.i2c = unsafe { riot_sys::macro_I2C_DEV(index as u32) };
        init(&params)
    }

    /// Set RGBC enable, this activates periodic RGBC measurements.
    pub fn set_rgbc_active(&self) {
        unsafe {
            riot_sys::tcs37727_set_rgbc_active(&self.dev as *const riot_sys::tcs37727_t);
        }
    }

    /// Set RGBC disable, this deactivates periodic RGBC measurements.
    pub fn set_rgbc_standby(&self) {
        unsafe {
            riot_sys::tcs37727_set_rgbc_standby(&self.dev as *const riot_sys::tcs37727_t);
        }
    }

    /// Read sensor's data.
    pub fn read(&mut self) -> Tcs37727Data {
        let mut data = MaybeUninit::<Tcs37727Data>::uninit();
        unsafe {
            riot_sys::tcs37727_read(
                &self.dev as *const riot_sys::tcs37727_t,
                &mut data as *mut _ as *mut riot_sys::tcs37727_data_t,
            )
        }
        unsafe { data.assume_init() }
    }

    /// returns the current amount of gain
    pub fn gain(&self) -> i32 {
        self.dev.again
    }
}
