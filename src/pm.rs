/// Reboot the MCU
pub fn reboot() {
    unsafe {
        riot_sys::pm_reboot();
    }
}

/// Turn off MCU completely.
pub fn off() {
    unsafe {
        riot_sys::pm_off();
    }
}

/// Switches the MCU to the lowest possible power mode.
pub fn set_lowest() {
    unsafe {
        riot_sys::pm_set_lowest();
    }
}
