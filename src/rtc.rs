/// Rust Wrapper based on [RIOT's RTC](https://doc.riot-os.org/group__drivers__periph__rtc.html)
use core::ffi::c_void;
use core::mem::MaybeUninit;

use riot_sys::tm;

#[derive(Debug)]
pub struct RTCTime {
    tm: riot_sys::tm,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RTCError {
    /// RTC could not be set to given time
    SetTimeFailed,
    /// Invalid time parameter is given
    InvalidTimeParameter,
    /// An unknown error did occur
    Unknown,
}

pub struct RTCTimeBuilder {
    time: RTCTime,
}

impl RTCTimeBuilder {
    pub fn new() -> Self {
        Self {
            time: RTCTime {
                tm: Default::default(),
            },
        }
    }
    pub fn seconds(mut self, sec: i32) -> Self {
        self.time.tm.tm_sec = sec;
        self
    }
    pub fn minutes(mut self, min: i32) -> Self {
        self.time.tm.tm_min = min;
        self
    }
    pub fn hour(mut self, hour: i32) -> Self {
        self.time.tm.tm_hour = hour;
        self
    }
    pub fn mday(mut self, mday: i32) -> Self {
        self.time.tm.tm_mday = mday;
        self
    }
    pub fn mon(mut self, mon: i32) -> Self {
        self.time.tm.tm_mon = mon;
        self
    }
    pub fn year(mut self, year: i32) -> Self {
        self.time.tm.tm_year = year;
        self
    }
    pub fn wday(mut self, wday: i32) -> Self {
        self.time.tm.tm_wday = wday;
        self
    }
    pub fn yday(mut self, yday: i32) -> Self {
        self.time.tm.tm_yday = yday;
        self
    }
    pub fn isdst(mut self, isdst: i32) -> Self {
        self.time.tm.tm_isdst = isdst;
        self
    }
    pub fn build(self) -> RTCTime {
        self.time
    }
}

/// Set RTC to given time
/// Arguments
/// * `time` - RTC Time struct
pub fn set_time(time: &mut RTCTime) -> Result<(), RTCError> {
    match unsafe { riot_sys::rtc_set_time(&mut time.tm as *mut _) } {
        0 => Ok(()),
        -1 => Err(RTCError::SetTimeFailed),
        _ => Err(RTCError::Unknown),
    }
}

/// Get current RTC time
/// Arguments
pub fn rtc_get_time() -> Result<RTCTime, RTCError> {
    let mut tm = MaybeUninit::<tm>::uninit();
    match unsafe { riot_sys::rtc_get_time(tm.as_mut_ptr() as *mut _) } {
        0 => Ok(RTCTime {
            tm: unsafe { tm.assume_init() },
        }),
        _ => Err(RTCError::Unknown),
    }
}

/// Get current RTC time with sub-second component.
/// Requires the periph_rtc_ms feature.
///#[cfg(riot_module_periph_rtc_ms)]
pub fn rtc_get_time_ms() -> Result<(RTCTime, u16), RTCError> {
    let mut ms: u16 = 0;
    let mut tm = MaybeUninit::<tm>::uninit();
    match unsafe { riot_sys::rtc_get_time_ms(tm.as_mut_ptr() as *mut _, &mut ms as *mut _) } {
        0 => Ok((
            RTCTime {
                tm: unsafe { tm.assume_init() },
            },
            ms,
        )),
        _ => Err(RTCError::Unknown),
    }
}

/// This is the callback gets called when the alarm hits
/// Arguments
/// * `user_callback` - The address pointing to the user defined callback
unsafe extern "C" fn alarm_callback<F>(user_callback: *mut c_void)
where
    F: FnMut(),
{
    (*(user_callback as *mut F))();
}

/// This function sets an alarm for RTC to a specified value
/// WARNING: Any already set alarm will be OVERWRITTEN.
/// Arguments
/// * `time` - The value to trigger an alarm when hit as a RTC Time struct.
/// * ùser_callback` - The user Arguments that gets passed to callback when alarm is hit.
pub fn set_alarm<F, M, R>(time: &mut RTCTime, user_callback: &mut F, main: M) -> Result<R, RTCError>
where
    F: FnMut() + Sync,
    M: FnOnce() -> R,
{
    match unsafe {
        riot_sys::rtc_set_alarm(
            &mut time.tm as *mut _,
            Some(alarm_callback::<F>),
            user_callback as *mut _ as *mut c_void,
        )
    } {
        0 => {
            let ans = (main)();
            unsafe {
                riot_sys::rtc_clear_alarm();
            }
            Ok(ans)
        }
        -2 => Err(RTCError::InvalidTimeParameter),
        _ => Err(RTCError::Unknown),
    }
}

/// Gets the current alarm setting.
pub fn rtc_get_alarm() -> Result<RTCTime, RTCError> {
    let mut tm = MaybeUninit::<tm>::uninit();
    match unsafe { riot_sys::rtc_get_alarm(tm.as_mut_ptr() as *mut _) } {
        0 => Ok(RTCTime {
            tm: unsafe { tm.assume_init() },
        }),
        _ => Err(RTCError::Unknown),
    }
}

/// Turns the RTC hardware module on.
pub fn rtc_poweron() {
    unsafe {
        riot_sys::rtc_poweron();
    }
}

/// Turns the RTC hardware module off.
pub fn rtc_poweroff() {
    unsafe {
        riot_sys::rtc_poweroff();
    }
}
