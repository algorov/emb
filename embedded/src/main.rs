#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::{
    self,
    gpio::{Level, Output, Speed},
};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PC13, Level::Low, Speed::Low);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(400)).await;
        led.set_low();
        Timer::after(Duration::from_millis(50)).await;
    }
}
