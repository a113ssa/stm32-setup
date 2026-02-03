#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{ i2c::I2c, pac, prelude::*, i2c::Config, delay::Delay };
use hd44780_driver::{ HD44780 };

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    // SCL
    let scl = gpiob.pb8
        .into_alternate::<4>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh)
        .set_open_drain();

    //SDA
    let sda = gpiob.pb9
        .into_alternate(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh)
        .set_open_drain();

    // I2C
    let i2c_config = Config::new(100.kHz(), clocks);
    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        i2c_config,
        &mut rcc.apb1r1,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let i2c_address = 0x27;
    let mut lcd = HD44780::new_i2c(i2c, i2c_address, &mut delay).expect("Failed to ini LCD");

    lcd.clear(&mut delay).unwrap();

    lcd.set_cursor_pos(0, &mut delay).unwrap();
    lcd.write_str("Hello, Rust", &mut delay).unwrap();

    lcd.set_cursor_pos(40, &mut delay).unwrap();
    lcd.write_str("Embedded Rocks", &mut delay).unwrap();

    loop {
        cortex_m::asm::nop();
    }
}
