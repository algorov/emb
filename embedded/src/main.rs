#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::export::{char, usize};
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
    digits: [u8; 16],
    tagged: [u8; 16],
    pointer: u8,
    step_pointer: u8,
    key_states: [u8; 16],
}

impl<
    'd,
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
        fonts: [char; FONT_CAPACITY],
    ) -> DivisionByTwo<'d, DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> {
        let display: LedAndKey<'d, DISPLAY_COUNT> = LedAndKey::default(stbs, clk, dio);
        let keyboard: Keyboard<'d, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> = Keyboard::default(rows, columns, fonts);

        let mut digits: [u8; 16] = [0; 16];
        let mut tagged: [u8; 16] = [0; 16];
        let mut pointer: u8 = 0;
        let mut step_pointer: u8 = 0;
        let mut key_states: [u8; 16] = [0; 16];

        Self { display, keyboard, digits, tagged, pointer, step_pointer, key_states }
    }

    fn run(&mut self) -> () {
        self.enter_numbers();
        self.step_pointer += 1;

        loop {
            match self.scan_keyboard() {
                'f' => {
                    if self.step_pointer == 1 {
                        self.set_tags();
                        self.step_pointer += 1;
                    }
                }
                'F' => {
                    if self.step_pointer == 2 {
                        self.division();
                        self.step_pointer += 1;
                    }
                }
                '-' => {
                    break;
                }
                _ => {}
            }
        }

        self.reset();
    }

    fn reset(&mut self) -> () {
        self.display.cleanup(0);
        self.display.cleanup(1);
        self.pointer = 0;
        self.step_pointer = 0;
        self.digits = [0; 16];
        self.tagged = [0; 16];
    }

    // Keyboard input.
    fn enter_numbers(&mut self) -> () {
        loop {
            let symbol: char = self.scan_keyboard();

            match char::to_digit(symbol, 10) {
                None => {}
                Some(d) => {
                    self.add_digit(d as u8);
                }
            }

            match symbol {
                '←' => {
                    self.remove_digit();
                }
                '+' => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn set_tags(&mut self) -> () {
        let mut i: u8 = 1;
        while i < self.pointer {
            if self.digits[i as usize - 1] % 2 == 1 {
                self.set_tag(i);
            }

            i += 1;
        }
    }

    // Puts a marker over a digit.
    fn set_tag(&mut self, pos: u8) -> () {
        self.tagged[pos as usize] = 1;
        self.print_tag(pos, 1);
    }

    // Step of dividing numbers by 2.
    fn division(&mut self) -> () {
        let mut pos: u8 = 0;

        while pos < self.pointer {
            let mut part = if self.tagged[pos as usize] == 1 { (self.digits[pos as usize] + 10) / 2 } else { self.digits[pos as usize] / 2 };
            let digit: char = self.convert_numb_to_char(part);
            self.digits[pos as usize] = part;
            self.print_symbol(pos, digit, false);

            if self.tagged[pos as usize] == 1 {
                self.print_tag(pos, 0);
            }

            pos += 1;
        }
    }

    // Adds a digit from the keyboard to the array and prints it on the display.
    fn add_digit(&mut self, number: u8) -> () {
        if self.pointer <= 15 {
            let symbol: char = self.convert_numb_to_char(number);

            self.digits[self.pointer as usize] = number;
            self.print_symbol(self.pointer, symbol, false);
            self.pointer += 1;
        }
    }

    // Removes a digit from the display.
    fn remove_digit(&mut self) -> () {
        if self.pointer != 0 {
            self.pointer -= 1;
            self.digits[self.pointer as usize] = 0;
            self.print_symbol(self.pointer, ' ', false);
        }
    }

    // Scans the keyboard. Returns the pressed key.
    fn scan_keyboard(&mut self) -> char {
        match self.keyboard.get_key() {
            Some(k) => { char::from(k) }
            None => { ' ' }
        }
    }

    // Prints digit on display.
    fn print_symbol(&mut self, pos: u8, value: char, add_point: bool) -> () {
        let address: (usize, u8) = self.def_address(pos);
        self.display.set_segment_value(address.0, address.1, value, add_point)
    }

    // Turns LED on specified position.
    fn print_tag(&mut self, pos: u8, value: u8) {
        let address: (usize, u8) = self.def_address(pos);
        self.display.set_led_state(address.0, address.1, value);
    }

    // Determines the display number and position for printing the value.
    fn def_address(&mut self, pos: u8) -> (usize, u8) {
        ((pos / 8) as usize, pos % 8)
    }

    // Converts a number to a char.
    fn convert_numb_to_char(&mut self, number: u8) -> char {
        match char::from_digit(number as u32, 10) {
            Some(c) => { c }
            None => { ' ' }
        }
    }
}


#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let stm: Peripherals = embassy_stm32::init(Default::default());

    let stbs: [Output<AnyPin>; DISPLAY_COUNT] = [
        Output::new(AnyPin::from(stm.PA1), Level::High, Speed::Low).degrade(),
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

    let mut teacher: DivisionByTwo<DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> =
        DivisionByTwo::init(
            stbs,
            AnyPin::from(stm.PA2),
            AnyPin::from(stm.PA3),
            rows,
            columns,
            FONTS,
        );

    loop {
        teacher.run();
    }
}
