#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config as Stm32Config};
use embassy_time::{Delay, Timer};
use hd44780_driver::{ HD44780 };

use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // init embassy and peripherals
    let config = Stm32Config::default();
    let p = embassy_stm32::init(config);

    // set i2c configuration
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000); // 100 kHz

    // lcd driver is synchronous so there is "new_blocking" call
    let i2c = I2c::new_blocking(
        p.I2C1,
        p.PB8, //SCL
        p.PB9, // SDA
        i2c_config,
    );

    let mut delay = Delay;

    let i2c_address = 0x27;
    let mut lcd = HD44780::new_i2c(
        i2c,
        i2c_address,
        &mut delay
    ).expect("Failed to init LCD");

    lcd.clear(&mut delay).unwrap();

    lcd.set_cursor_pos(0, &mut delay).unwrap();
    lcd.write_str("Hello, Rust!", &mut delay).unwrap();
    lcd.set_cursor_pos(42, &mut delay).unwrap();
    lcd.write_str("  =^_^=    ", &mut delay).unwrap();

    loop {
        Timer::after_millis(1000).await;
    }
}
