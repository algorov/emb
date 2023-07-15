#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod led_and_key;

use embassy_executor::Spawner;
use embassy_stm32::{
    self,
    gpio::{Level, Output, Speed},
};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use crate::led_and_key::LedAndKey;


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(p.PA3, p.PA4, p.PA5);

    Timer::after(Duration::from_secs(5)).await;



    loop {
        for i in 0..8 {
            driver.set_led_value(i, 1);
            Timer::after(Duration::from_millis(20)).await;
            driver.set_brightness(3);
            Timer::after(Duration::from_millis(100)).await;
            driver.set_led_value(i, 0);
            driver.set_brightness(7);
            Timer::after(Duration::from_millis(10)).await;
        }

        driver.get_keys();
    }
}
