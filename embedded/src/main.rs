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

use crate::tm1632::LedAndKey;
use crate::tm1632::fonts::DigitSymbol;


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut driver = LedAndKey::new(p.PA1, p.PA2, p.PA3);

    // let mut array: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    //
    // driver.cleanup();

    let mut inC1 = Input::new(AnyPin::from(p.PA4), Pull::Up);
    let mut inC2 = Input::new(AnyPin::from(p.PA5), Pull::Up);
    let mut inC3 = Input::new(AnyPin::from(p.PA6), Pull::Up);
    let mut inC4 = Input::new(AnyPin::from(p.PA7), Pull::Up);
    let mut outR1 = Output::new(AnyPin::from(p.PA8), Level::High, Speed::Low);
    let mut outR2 = Output::new(AnyPin::from(p.PA9), Level::High, Speed::Low);
    let mut outR3 = Output::new(AnyPin::from(p.PA10), Level::High, Speed::Low);
    let mut outR4 = Output::new(AnyPin::from(p.PA11), Level::High, Speed::Low);
    let mut outR5 = Output::new(AnyPin::from(p.PA12), Level::High, Speed::Low);

    let mut column = [inC1, inC2, inC3, inC4];
    let mut rows = [outR5, outR4, outR3, outR2, outR1];
    let mut a = 0;
    loop {

        for i in 0..5 {
            rows[i].set_low();

            for j in 0..4 {
                if !column[j].is_high() {
                    println!("Координата: {}", i * 4 + j);
                    a += 1;
                    println!("Итерация: {}", a );

                }
            }

            rows[i].set_high();
        }

    //
    //     Timer::after(Duration::from_millis(500)).await;


        // for i in 0..8 {
        //     array = *driver.def_pressed_keys(&mut array);
        //
        //     if array[3] == 1 {
        //         driver.set_segment_value(0, DigitSymbol::DIGIT_1);
        //     } else if array[7] == 1 {
        //         driver.cleanup();
        //     }
        //
        //     driver.set_led_state(i, 1);
        //     Timer::after(Duration::from_millis(20)).await;
        //     Timer::after(Duration::from_millis(100)).await;
        //     driver.set_led_state(i, 0);
        //     Timer::after(Duration::from_millis(10)).await;
        // }
    }
}
