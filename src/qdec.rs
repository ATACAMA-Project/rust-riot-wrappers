#![allow(dead_code)]

use core::ffi::c_void;
use core::marker::PhantomData;
use riot_sys::libc::c_uint;
use riot_sys::*;

/// Represents various states of the `QDEC` device
#[derive(Debug)]
#[non_exhaustive]
pub enum QDECStatus {
    Success,
    InitError,
}

impl QDECStatus {
    fn from_c(n: i32) -> Self {
        match n {
            0 => Self::Success,
            _ => Self::InitError,
        }
    }
}

/// Represents the available modes of the `QDEC` device. For more info visit: [qdec_mode_t](https://doc.riot-os.org/group__drivers__periph__qdec.html#ga5ce11edd1c986b4c6228b23cf8ef1fef)
#[derive(Debug)]
#[non_exhaustive]
pub enum QDECMode {
    X1,
    X2,
    X4,
}


impl QDECMode {
    fn to_c(self) -> qdec_mode_t {
        match self {
            Self::X1 => qdec_mode_t_QDEC_X1,
            Self::X2 => qdec_mode_t_QDEC_X2,
            Self::X4 => qdec_mode_t_QDEC_X4,
        }
    }
}

/// A `QDEC` device backed by RIOT’s [QDEC implementation](https://doc.riot-os.org/group__drivers__periph__qdec.html)
#[derive(Debug)]
pub struct QDEC<'scope> {
    dev: qdec_t,
    _scope: PhantomData<&'scope ()>,
}


impl<'scope> QDEC<'scope> {
    /// Construct a new `QDEC` device from the given parameters
    ///
    /// This is the scoped version of [`new()`] that can be used if you want to use short-lived callbacks, such as
    /// closures or anything containing references. The QDECDevice is deconfigured when the internal main function
    /// terminates. A common pattern around this kind of scoped functions is that `main` contains the application's
    /// main loop, and never terminates (in which case the clean-up code is eliminated during compilation).
    ///
    /// # Arguments
    /// * `dev` - The qdec_t handle to the hardware device
    /// * `mode` - The mode used for the device
    /// * `user_callback` The user defined callback that gets called from the os whenever the ˋQDECˋ Timer overflows
    /// * `main` The mainloop that is executed inside the wrapper
    ///
    /// # Examples
    /// ```
    /// use riot_wrappers::qdec::{QDECMode, QDEC};
    /// let mut cb = || {
    ///     //do something when the QDEC timer overflows
    /// };
    /// let mut scoped_main = |self_: &mut QDEC| loop {};
    /// let qdec = QDEC::new(0, QDECMode::X1, &mut cb, scoped_main)
    ///     .unwrap_or_else(|e| panic!("Error initializing QDEC: {e:?}"));
    /// loop {}
    /// ```
    pub fn new<F, Main, RMain>(
        index: usize,
        mode: QDECMode,
        user_callback: &'scope mut F,
        main: Main,
    ) -> Result<RMain, QDECStatus>
    where
        F: FnMut() + Sync + 'scope,
        Main: FnOnce(&mut Self) -> RMain,
    {
        let mut self_ = unsafe {
            let dev = macro_QDEC_DEV(index as c_uint);
            match QDECStatus::from_c(qdec_init(
                dev,
                mode.to_c(),
                Some(Self::qdec_overflow_callback::<F>),
                user_callback as *mut _ as *mut c_void,
            )) {
                QDECStatus::Success => Ok(Self {
                    dev,
                    _scope: PhantomData,
                }),
                QDECStatus::InitError => Err(QDECStatus::InitError),
            }
        }?;
        let ret = Ok((main)(&mut self_));
        qdec_init(
            dev,
            mode.to_c(),
            Some(Self::qdec_overflow_callback::<F>),
            || {} as *mut _ as *mut c_void,
        );
        ret
    }

    // This callback is called directly from the kernel
    unsafe extern "C" fn qdec_overflow_callback<F>(user_callback: *mut c_void)
    where
        F: FnMut() + 'scope,
    {
        (*(user_callback as *mut F))();
    }

    /// Reads the value from the `QDEC` and returns it
    pub fn read(&self) -> i32 {
        unsafe { qdec_read(self.dev) }
    }

    /// Reads the value from the `QDEC`, returns it and reset the counter
    pub fn read_and_reset(&mut self) -> i32 {
        unsafe { qdec_read_and_reset(self.dev) }
    }

    /// Start the given qdec timer. This function is only needed if the qdec timer was stopped manually before.
    pub unsafe fn start(&mut self) {
        qdec_start(self.dev);
    }

    /// Stop the given qdec timer. This will effect all of the timer's channels.
    pub unsafe fn stop(&mut self) {
        qdec_stop(self.dev);
    }
}
