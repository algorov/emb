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

    loop {}
}
