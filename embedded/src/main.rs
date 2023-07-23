#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod tm1632;
mod keyboard;

use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self, Peripheral};
use embassy_stm32::gpio::{AnyPin, Pin};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

use crate::keyboard::Keyboard;
use crate::tm1632::LedAndKey;
use crate::tm1632::config;
use crate::tm1632::config::BrightnessLevel::{L1, L7};


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(
        AnyPin::from(p.PA1),
        // AnyPin::from(p.PA15),
        AnyPin::from(p.PA2),
        AnyPin::from(p.PA3),
    );

    let mut array: [u8; config::KEY_COUNT] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        // 0, 0, 0, 0, 0, 0, 0, 0,
    ];


    driver.cleanup();
    driver.set_text_in_segments(0, ['R', ' ', 'U', ' ', 'S', ' ', 'T', ' ']);

    let mut k = Keyboard::default(
        AnyPin::from(p.PA4),
        AnyPin::from(p.PA5),
        AnyPin::from(p.PA6),
        AnyPin::from(p.PA7),
        AnyPin::from(p.PA8),
        AnyPin::from(p.PA9),
        AnyPin::from(p.PA10),
        AnyPin::from(p.PA11),
        AnyPin::from(p.PA12),
    );

    loop {
        match k.get_key() {
            Some(x) => println!("Result: {}", char::from(x)),
            None => {}
        }
        driver.set_led_state(0, 0, 23);
        driver.def_pressed_keys(&mut array);
        println!("{:?}", array);

        if array[0] == 1 && array[7] == 1 {
            driver.set_text_in_segments(0, ['A', 'H', 'A', 'H', 'A', 'H', 'A', 'H']);
        }

        if array[1] == 1 && array[2] == 1 {
            driver.set_text_in_segments(0, ['Q', 'U', 'E', 'S', 'O', 'O', 'N', 'N']);
        }
        Timer::after(Duration::from_millis(1000)).await;
        driver.set_brightness(0, L1);
        Timer::after(Duration::from_millis(1000)).await;
        driver.set_brightness(0, L7);
        Timer::after(Duration::from_millis(1000)).await;

    }
}
