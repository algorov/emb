#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod tm1632;
mod keyboard;

use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self, Peripheral, Peripherals};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use crate::keyboard::Keyboard;

use crate::tm1632::LedAndKey;
use crate::tm1632::fonts::DigitSymbol;


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(p.PA1, p.PA2, p.PA3);
    let mut array: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    driver.cleanup();

    let mut k = Keyboard::new(
        AnyPin::from(p.PA4),
        AnyPin::from(p.PA5),
        AnyPin::from(p.PA6),
        AnyPin::from(p.PA7),
        AnyPin::from(p.PA8),
        AnyPin::from(p.PA9),
        AnyPin::from(p.PA10),
        AnyPin::from(p.PA11),
        AnyPin::from(p.PA12)
    );

    loop {
        k.find_pressed_key_now();
        println!("{}", k.pressed_key_now);
    //
        Timer::after(Duration::from_millis(500)).await;


        for i in 0..8 {
            array = *driver.def_pressed_keys(&mut array);

            if array[3] == 1 {
                driver.set_segment_value(0, DigitSymbol::DIGIT_1);
            } else if array[7] == 1 {
                driver.cleanup();
            }

            driver.set_led_state(i, 1);
            Timer::after(Duration::from_millis(20)).await;
            Timer::after(Duration::from_millis(100)).await;
            driver.set_led_state(i, 0);
            Timer::after(Duration::from_millis(10)).await;
        }
    }
}
