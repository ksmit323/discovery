#![no_main]
#![no_std]

use core::str::from_utf8;
use cortex_m_rt::entry;
use rtt_target::{rprint, rtt_init_print};
use panic_rtt_target as _;
use core::fmt::Write;
use embedded_hal_nb::serial::Write as _; 
use rtt_target::rprintln;
use embedded_hal_nb::serial::Read;
use heapless::Vec;

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
    
    let mut buffer: Vec<u8, 32> = Vec::new();
    
    loop {
        // Computer to micro bit
        let byte = nb::block!(serial.read()).unwrap();
                
        if byte == 13 {
            // Micro bit (server) back to the computer (client)
            for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                nb::block!(serial.write(*byte)).unwrap();
            }
            nb::block!(serial.flush()).unwrap();    
            buffer.clear();
        } else {
            match buffer.push(byte) {
                Ok(()) => continue,
                Err(e) => {
                    write!(serial, "error: buffer full\r\n").unwrap();
                    buffer.clear();
                }
            }
        }
    }
}