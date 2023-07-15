#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod led_and_key;
mod fonts;

use embassy_executor::Spawner;
use embassy_stm32::{
    self,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use crate::led_and_key::LedAndKey;


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(p.PA3, p.PA4, p.PA5);


    // let mut keys: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];





    loop {
        driver.set_segment_value(0, fonts::DIGIT_C);
        driver.set_segment_value(1, fonts::DIGIT_Y);
        driver.set_segment_value(2, fonts::DIGIT_K);
        driver.set_segment_value(3, fonts::DIGIT_A);

        for i in 0..8 {
            driver.set_led_value(i, 1);
            Timer::after(Duration::from_millis(20)).await;
            Timer::after(Duration::from_millis(100)).await;
            driver.set_led_value(i, 0);
            Timer::after(Duration::from_millis(10)).await;
        }

        driver.cleanup();
        Timer::after(Duration::from_millis(1000)).await;
    }
}
