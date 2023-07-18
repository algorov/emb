#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod led_and_key;
mod fonts;

use core::ptr::null;
use defmt::println;
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

    let mut array: [bool; 8] = [false, false, false, false, false, false, false, false];



    driver.set_segment_value(0, fonts::DIGIT_H);
    driver.set_segment_value(1, fonts::DIGIT_E);
    driver.set_segment_value(2, fonts::DIGIT_H);
    driver.set_segment_value(3, fonts::DIGIT_E);
    driver.set_segment_value(4, fonts::DIGIT_H);
    driver.set_segment_value(5, fonts::DIGIT_E);
    driver.set_segment_value(6, 0x00);
    driver.set_segment_value(7, 0x00);



    loop {

        for i in 0..8 {
            let tmp: &mut [bool; 8] = driver.def_pressed_keys(&mut array);
            println!("{:?}", tmp);
            driver.set_led_state(i, 1);
            Timer::after(Duration::from_millis(20)).await;
            Timer::after(Duration::from_millis(100)).await;
            driver.set_led_state(i, 0);
            Timer::after(Duration::from_millis(10)).await;
        }

    }
}
