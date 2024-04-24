#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = nb::block!(serial.read()).unwrap();

        // Receive a user request. Each user request ends with ENTER
        // Send back the reversed string
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.
        if (byte == 13) {
            for char in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                nb::block!(serial.write(*char)).unwrap();
            }

            nb::block!(serial.flush()).unwrap();
            buffer.clear();
        } else {
            buffer.push(byte).unwrap_or_else(|error: u8| {
                write!(serial, "Error ocurred while typing : {}.\r\n", error as char).unwrap();
            });
        }
    }
}
