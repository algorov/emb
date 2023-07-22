#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod tm1632;
mod keyboard;

use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self, Peripheral};
use embassy_stm32::gpio::{AnyPin, Pin};

use {defmt_rtt as _, panic_probe as _};

use crate::keyboard::Keyboard;
use crate::tm1632::LedAndKey;


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(p.PA1, p.PA2, p.PA3);
    // let mut array: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    driver.cleanup();

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
    }
}
