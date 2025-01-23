#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use microbit::hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, MagOutputDataRate, Lsm303agr};
use heapless::Vec;
use nb::block;
use core::fmt::Write;

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

    #[cfg(feature = "v2")]
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut buffer: Vec<u8, 32> = Vec::new();
    
    loop {
        // Computer to micro bit
        let byte = block!(serial.read()).unwrap();
        buffer.push(byte);
        let command = str::from_utf8(&buffer).unwrap().trim();
        
        if command == "accelerometer" {
            if sensor.accel_status().unwrap().xyz_new_data {
                let data = sensor.accel_data().unwrap();
                write!(serial, "Accelerometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
                buffer.clear();
            }
        } else if command == "magnetometer" {
            if sensor.mag_status().unwrap().xyz_new_data {
                let data = sensor.mag_data().unwrap();
                write!(serial, "Magnetometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
                buffer.clear();
            }
        }
    }
}
