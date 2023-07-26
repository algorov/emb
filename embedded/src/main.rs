#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::export::char;
use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_stm32::Peripherals;
use embassy_time::{Duration, Timer};

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
    values: [u8; 16],
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
        let mut values: [u8; 16] = [0; 16];

        Self { display, keyboard, values }
    }

    pub fn run(&mut self) -> () {
        self.begin();
    }

    fn begin(&mut self) -> () {
        self.print_to_display();
    }

    // Печатает весь массив в дисплеи.
    fn print_to_display(&mut self) -> () {
        for pos in 0..self.values.len() {
            let symb: char = self.convert_numb_to_char(self.values[pos]);

            self.print_symbol(pos as u8, symb, false);
        }
    }

    // Конвертирует число в символьное представление.
    fn convert_numb_to_char(&mut self, number: u8) -> char {
        match char::from_digit(number as u32, 10) {
            Some(x) => { x }
            None => { ' ' }
        }
    }

    // Адаптационная (для двух дисплеев) печать числа.
    fn print_symbol(&mut self, position: u8, value: char, add_point: bool) -> () {
        let address: (usize, u8) = self.def_address(position);
        println!("pos {} id {} posi {}", position, address.0, address.1);
        self.display.set_segment_value(address.0, address.1, value, add_point)
    }

    // Вычисляет номер дисплея и позицию сегмента для записи числа.
    fn def_address(&mut self, position: u8) -> (usize, u8) {
        let coeff = position / 8;
        (coeff as usize, position - 8 * coeff)
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
