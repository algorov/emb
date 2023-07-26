#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_stm32::Peripherals;

use {defmt_rtt as _, panic_probe as _};
use config::*;

use keyboard::*;
use tm1638::*;

mod config;

struct DivisionByTwo<'d,
    const DISPLAY_COUNT: usize,
    const ROW_COUNT: usize,
    const COLUMN_COUNT: usize,
    const FONT_CAPACITY: usize> {
    display: LedAndKey<'d, DISPLAY_COUNT>,
    keyboard: Keyboard<'d, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY>,
}

impl<'d,
    const DISPLAY_COUNT: usize,
    const ROW_COUNT: usize,
    const COLUMN_COUNT: usize,
    const FONT_CAPACITY: usize> DivisionByTwo<'d, DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> {
    fn init(
        stbs: [Output<'d, AnyPin>; DISPLAY_COUNT],
        clk: AnyPin,
        dio: AnyPin,
        rows: [Output<'d, AnyPin>; ROW_COUNT],
        columns: [Input<'d, AnyPin>; COLUMN_COUNT],
        fonts: [u8; FONT_CAPACITY],
    ) -> DivisionByTwo<'d, DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> {
        let display: LedAndKey<'d, DISPLAY_COUNT> = LedAndKey::default(stbs, clk, dio);
        let keyboard: Keyboard<'d, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> = Keyboard::default(rows, columns, fonts);

        Self { display, keyboard }
    }

    pub fn run(&mut self) -> () {
        println!("AHAHAHAHAHAHAHAHA");
    }
}


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let stm: Peripherals = embassy_stm32::init(Default::default());

    let stbs: [Output<AnyPin>; DISPLAY_COUNT] = [
        Output::new(AnyPin::from(stm.PA1), Level::High, Speed::Low),
        Output::new(AnyPin::from(stm.PA15), Level::High, Speed::Low),
    ];
    let rows: [Output<AnyPin>; ROW_COUNT] = [
        Output::new(AnyPin::from(stm.PA8), Level::High, Speed::Low),
        Output::new(AnyPin::from(stm.PA9), Level::High, Speed::Low),
        Output::new(AnyPin::from(stm.PA10), Level::High, Speed::Low),
        Output::new(AnyPin::from(stm.PA11), Level::High, Speed::Low),
        Output::new(AnyPin::from(stm.PA12), Level::High, Speed::Low),
    ];
    let columns: [Input<AnyPin>; COLUMN_COUNT] = [
        Input::new(AnyPin::from(stm.PA4), Pull::Up),
        Input::new(AnyPin::from(stm.PA5), Pull::Up),
        Input::new(AnyPin::from(stm.PA6), Pull::Up),
        Input::new(AnyPin::from(stm.PA7), Pull::Up),
    ];

    let mut game: DivisionByTwo<DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> =
        DivisionByTwo::init(
            stbs,
            AnyPin::from(stm.PA2),
            AnyPin::from(stm.PA3),
            rows,
            columns,
            FONTS,
        );

    game.run();

    loop {
        // match matrix_keyboard.get_key() {
        //     Some(x) => println!("Result: {}", char::from(x)),
        //     None => {}
        // }

        // for id in 0..DISPLAY_COUNT {
        //     let mut temp: [u8; 8] = [0; 8];
        //     tm1638.get_pressed_keys(id, &mut temp);
        //
        //     for i in 0..8 {
        //         key_states[i + id * 8] = temp[i];
        //     }
        // }
        //
        // println!("{:?}", key_states);
    }
}
