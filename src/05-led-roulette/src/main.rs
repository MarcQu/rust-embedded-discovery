#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut light_led = (0,0);
    let mut matrix: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        // Show light_it_all for 100ms
        display.show(&mut timer, matrix, 100);
        // clear the display again
        display.clear();
        timer.delay_ms(100_u32);

        matrix[light_led.0][light_led.1] = 0;

        light_led = match (light_led.0, light_led.1) {
            (0, col) if col != 4 => (0, col + 1),
            (row, 4) if row != 4 => (row + 1, 4),
            (4, col) if col != 0 => (4, col - 1),
            (row, 0) if row != 0 => (row - 1, 0),
            otherwise => otherwise,
        };

        matrix[light_led.0][light_led.1] = 1;
    }
}
