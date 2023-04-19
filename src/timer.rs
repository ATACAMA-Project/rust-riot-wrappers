//! This module allows access to a boards hardware timer. It is intended to be used when timing is very
//! critical e.g. hardware driver and the precision of zTimer is not sufficient or one wants to be independent from the zTimers configuration.
//! In contrast to the zTimer however the hardware timer is not inherently thread safe!
//! It wraps RIOTs [hardware timer interface](https://doc.riot-os.org/group__drivers__periph__timer.html)

use core::{ffi::c_void, marker::PhantomData, ptr};

pub use fugit::HertzU32;
use riot_sys::tim_t;

#[derive(Debug)]
pub struct HardwareTimer<F> {
    c_timer: riot_sys::tim_t,
    _phantom: PhantomData<F>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum HardwareTimerError {
    /// Either the chosen frequency is not applicable or
    /// the given device index was not valid
    InitError,

    /// Something went wrong when setting a timeout for a channel
    SetError,

    /// An unknown error occurred
    Unknown,
}

impl<F> HardwareTimer<F> {
    fn init(
        index: u32,
        freq: HertzU32,
        cb: unsafe extern "C" fn(*mut c_void, i32),
        args: *mut c_void,
    ) -> Result<Self, HardwareTimerError> {
        let dev: tim_t = unsafe { riot_sys::macro_TIMER_DEV(index) };

        let ret = unsafe { riot_sys::timer_init(dev, freq.to_Hz(), Some(cb), args) };

        match ret {
            0 => Ok(HardwareTimer {
                c_timer: dev,
                _phantom: PhantomData {},
            }),
            -1 => Err(HardwareTimerError::InitError),
            _ => Err(HardwareTimerError::Unknown),
        }
    }

    /// Read the current absolute tick value.
    pub fn read(&mut self) -> u32 {
        unsafe { riot_sys::timer_read(self.c_timer) }
    }

    /// The timer starts automatically. This method is only necessary
    /// to restart when [`stop`] was called.
    pub fn start(&mut self) {
        unsafe {
            riot_sys::timer_start(self.c_timer);
        }
    }

    /// Stops the entire timer device. This affects all channels
    /// of this device.
    pub fn stop(&mut self) {
        unsafe {
            riot_sys::timer_stop(self.c_timer);
        }
    }
}

impl HardwareTimer<()> {
    /// Convenience method which just sets an empty callback
    pub fn new_without_cb(index: u32, freq: HertzU32) -> Result<Self, HardwareTimerError> {
        HardwareTimer::init(index, freq, empty_cb, ptr::null_mut())
    }
}

impl<F> HardwareTimer<F> {
    /// Creates a timer and makes it accessible in the given `main` closure.
    ///
    /// Only in there methods like `set` can be called.
    /// The registered callback will only be registered while in the `main` closure!
    /// At the end of the `main` closure the callback is deregistered! So in order to
    /// receive callbacks the `main` closure must not be left.
    pub fn new_scoped<Main, RMain>(
        index: u32,
        freq: HertzU32,
        call_back: &mut F,
        main: Main,
    ) -> Result<RMain, HardwareTimerError>
    where
        F: FnMut(i32) + Send,
        Main: FnOnce(HardwareTimer<F>) -> RMain,
    {
        let timer: HardwareTimer<F> =
            HardwareTimer::init(index, freq, c_callback::<F>, call_back as *mut _ as *mut _)?;

        let ret = (main)(timer);

        // this should never fail, because we would have failed on this call above
        HardwareTimer::<F>::init(index, freq, empty_cb, ptr::null_mut())?;

        Ok(ret)
    }

    /// Sets a timeout for a channel of the timer device. When the given amount of ticks
    /// has passed the callback given in [`new_scoped`] will be called.
    pub fn set(&mut self, channel: i32, timeout_ticks: u32) -> Result<(), HardwareTimerError> {
        match unsafe { riot_sys::timer_set(self.c_timer, channel, timeout_ticks) } {
            0 => Ok(()),
            -1 => Err(HardwareTimerError::SetError),
            _ => Err(HardwareTimerError::Unknown),
        }
    }

    /// Sets a absolute tick timestamp for a channel of the timer device. When the timer reaches the
    /// given timestamp the callback given in [`new_scoped`] will be called.
    ///
    /// A timer that can not handle the full range of `u32` will truncate the value.
    pub fn set_absolute(&mut self, channel: i32, timestamp: u32) -> Result<(), HardwareTimerError> {
        match unsafe { riot_sys::timer_set_absolute(self.c_timer, channel, timestamp) } {
            0 => Ok(()),
            -1 => Err(HardwareTimerError::SetError),
            _ => Err(HardwareTimerError::Unknown),
        }
    }
}

extern "C" fn empty_cb(_args: *mut c_void, _channel: i32) {}

extern "C" fn c_callback<F: FnMut(i32)>(args: *mut c_void, channel: i32) {
    unsafe { (*(args as *mut F))(channel) }
}
