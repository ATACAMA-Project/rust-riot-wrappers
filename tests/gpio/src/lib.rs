#![no_std]

use riot_wrappers::gpio::{InputMode, OutputMode, GPIO};
use riot_wrappers::println;
use riot_wrappers::riot_main;

use embedded_hal::digital::{InputPin, OutputPin, PinState};

riot_main!(main);

fn main() {
    let (out_port, out_pin, in_port, in_pin, in_mode) = match riot_wrappers::BOARD {
        // Won't work -- currently, native GPIO don't do anything (but let's not panic already)
        "native" => (0, 0, 0, 1, InputMode::In),
        // 0.17 is LED1, 0.13 is button 1
        "nrf52dk" => (0, 17, 0, 13, InputMode::InPullUp),

        // Better safe than drive pins that were not supposed to be driven
        _ => panic!("For this board, no GPIO pins were deemed safe to reconfigure."),
    };
    let mut p_out = GPIO::from_port_and_pin(out_port, out_pin)
        .expect("Out pin does not exist")
        .configure_as_output(OutputMode::Out)
        .expect("Out pin could not be configured");
    let mut p_in = GPIO::from_port_and_pin(in_port, in_pin)
        .expect("In pin does not exist")
        .configure_as_input(in_mode)
        .expect("In pin could not be configured");

    loop {
        let value = p_in.is_high().unwrap();
        println!("Read GPIO value {}, writing it to the out port", value);
        p_out
            .set_state(if value { PinState::High } else { PinState::Low })
            .unwrap();
    }
}
