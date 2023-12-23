#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl, entry, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*, Delay,
};
use esp_backtrace as _;
use esp_println::println;

#[derive(Debug)]
pub struct Measurement {
    pub co2: f32,
    pub humidity: f32,
    pub temperature: f32,
}

#[entry]
fn main() -> ! {
    // init peripherals
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // init i2c pins
    let sda = io.pins.gpio21;
    let scl = io.pins.gpio22;

    // open i2c connection
    let mut i2c = I2C::new(peripherals.I2C1, sda, scl, 1u32.kHz(), &clocks);

    // delay configuration
    let mut delay = Delay::new(&clocks);

    // begin measurement reading loop
    loop {
        // measurement are returned in big endian format and are 18 bytes long
        let mut data = [0u8; 18];

        // write to sensor at address 0x61
        // 0x0300 is the command to read measurement
        i2c.write_read(0x61, &[0x03, 0x00], &mut data).ok();

        // decode the bytes into floats
        let m = Measurement {
            co2: f32::from_bits(u32::from_be_bytes([data[0], data[1], data[3], data[4]])),
            temperature: f32::from_bits(u32::from_be_bytes([data[6], data[7], data[9], data[10]])),
            humidity: f32::from_bits(u32::from_be_bytes([data[12], data[13], data[15], data[16]])),
        };

        // report the measurement
        println!("CO₂: {:?} ppm", m.co2);
        delay.delay_ms(2_000u32);
    }
}
