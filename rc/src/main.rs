#![no_std]
#![no_main]

use embassy_executor::{Spawner};
use embassy_stm32::rcc::{Pll, PllMul, PllPreDiv, PllRDiv};
use embassy_stm32::{Config, bind_interrupts};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Pull};
use embassy_stm32::exti::InterruptHandler;
use embassy_stm32::rcc::PllSource;
use embassy_stm32::rcc::Sysclk::{PLL1_R};
use embassy_stm32::rcc::MSIRange::{RANGE4M};
use embassy_time::Instant;
use infrared::Receiver;
use infrared::protocol::nec::{Nec16Command,};
use infrared::receiver::{NoPin};
use infrared::protocol::{Nec16};

use defmt_rtt as _;
use panic_probe as _;

use defmt::info;


bind_interrupts!(struct Irqs {
    EXTI0 => InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Start");
    let mut config: Config = Config::default();

    config.rcc.msi = Some(RANGE4M);
    config.rcc.sys = PLL1_R;
    config.rcc.pll = Some(Pll{
        source: PllSource::MSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL40,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2)
    });

    let p: embassy_stm32::Peripherals = embassy_stm32::init(config);

    let mut ir_pin = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up, Irqs);


    let mut receiver= Receiver::<Nec16, NoPin, u32, Nec16Command>::new(1_000_000);

    let mut last_tick = Instant::now();

    loop {
        ir_pin.wait_for_any_edge().await;
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_micros() as u32;
        last_tick = now;

        let edge = ir_pin.is_low();

        match receiver.event(dt, edge) {
                Ok(Some(cmd)) => {
                    info!("SUCCESS! Debug : {}", cmd.cmd);
                }
                Ok(None) => {}
                Err(_e) => {}
            }
    }
}
