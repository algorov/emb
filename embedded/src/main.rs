#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::export::{char, usize};
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
    values: [u8; 16],
    tagged: [u8; 16],
    pointer: u8,
    step_pointer: u8,
    key_states: [u8; 16],
}
//
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

        let mut values: [u8; 16] = [0; 16];
        let mut tagged: [u8; 16] = [0; 16];
        let mut pointer: u8 = 0;
        let mut step_pointer: u8 = 0;
        let mut key_states: [u8; 16] = [0; 16];

        Self { display, keyboard, values, tagged, pointer, step_pointer, key_states }
    }

    pub fn run(&mut self) -> () {
        self.reset();

        loop {
            let states = self.scan_keys();
            match states.0 {
                'f' => {
                    if self.step_pointer == 0 {
                        self.enter_numbers();
                        self.step_pointer += 1;
                    }

                }
                'F' => {
                    if self.step_pointer == 1 {
                        self.set_tags();
                        self.step_pointer += 1;
                    }

                }
                '#' => {
                    if self.step_pointer == 2 {
                        self.grouping();
                        self.step_pointer += 1;
                    }
                }
                '*' => {
                    if self.step_pointer == 3 || self.step_pointer == 2 {
                        self.division();
                        self.step_pointer += if self.step_pointer == 2 { 2 } else { 1 };
                    }
                }
                '-' => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn reset(&mut self) -> () {
        self.display.cleanup(0);
        self.display.cleanup(1);
        self.pointer = 0;
        self.step_pointer = 0;
        self.values = [0; 16];
        self.tagged = [0; 16];
        self.print_values();
    }

    // Keyboard input.
    fn enter_numbers(&mut self) -> () {
        loop {
            match self.scan_keyboard() {
                '0' => {
                    self.add_numb(0);
                }
                '1' => {
                    self.add_numb(1);
                }
                '2' => {
                    self.add_numb(2);
                }
                '3' => {
                    self.add_numb(3);
                }
                '4' => {
                    self.add_numb(4);
                }
                '5' => {
                    self.add_numb(5);
                }
                '6' => {
                    self.add_numb(6);
                }
                '7' => {
                    self.add_numb(7);
                }
                '8' => {
                    self.add_numb(8);
                }
                '9' => {
                    self.add_numb(9);
                }
                '←' => {
                    self.remove_numb();
                }
                '+' => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn set_tags(&mut self) -> () {
        let mut k: u8 = 1;
        while k < self.pointer {
            loop {
                match self.scan_keyboard() {
                    '→' => { break }
                    _ => {}
                }
            }

            if self.values[k as usize - 1] % 2 == 1 {
                self.set_tag(k);


            }

            k += 1;
        }
    }

    fn set_tag(&mut self, pos: u8) -> () {
        self.tagged[pos as usize] = 1;
        self.print_tag(pos, 1);
    }

    fn grouping(&mut self) -> () {
        loop {
            let keys = self.scan_keys();

            match keys.0 {
                '+' => { break }
                _ => {}
            }

            for pos in 0..16 {
                if self.key_states[pos] == 1 {
                    if (pos as u8) < self.pointer {
                        let symbol = self.convert_numb_to_char(self.values[pos as usize]);
                        self.print_symbol(pos as u8, symbol, true);
                    }
                }
            }
        }
    }

    fn division(&mut self) -> () {
        let mut pos: u8 = 0;
        while  pos < self.pointer {
            loop {
                match self.scan_keyboard() {
                    '→' => { break }
                    _ => {}
                }
            }

            let mut part = if self.tagged[pos as usize] == 1 {(self.values[pos as usize] + 10) / 2} else { self.values[pos as usize] / 2 };
            let temp = self.convert_numb_to_char(part);
            self.values[pos as usize] = part;
            self.print_symbol(pos, temp, false);

            if self.tagged[pos as usize] == 1 {
                self.print_tag(pos, 0);
            }

            pos += 1;
        }
    }

    // Adds the number entered from the keyboard to the array and prints it on the display.
    fn add_numb(&mut self, numb: u8) -> () {
        if self.pointer <= 15 {
            let symbol: char = self.convert_numb_to_char(numb);

            self.values[self.pointer as usize] = numb;
            self.print_symbol(self.pointer, symbol, false);
            self.pointer += 1;
        }
    }


    fn remove_numb(&mut self) -> () {
        if self.pointer != 0 {
            self.pointer -= 1;
            self.values[self.pointer as usize] = 0;

            let symbol: char = self.convert_numb_to_char(0);
            self.print_symbol(self.pointer, symbol, false);
        }
    }

    // Scans input devices.
    fn scan_keys(&mut self) -> (char, &mut [u8; 16]) {
        self.scan_display_keys();
        (self.scan_keyboard(), &mut self.key_states)
    }

    // Scan the matrix keyboard. Returns the pressed key.
    fn scan_keyboard(&mut self) -> char {
        match self.keyboard.get_key() {
            Some(x) => { char::from(x) }
            None => { ' ' }
        }
    }

    // Scans keys from displays.
    fn scan_display_keys(&mut self) -> () {
        for id in 0..DISPLAY_COUNT {
            let mut temp: [u8; 8] = [0; 8];
            self.display.get_pressed_keys(id, &mut temp);

            for key in 0..8 {
                self.key_states[key + id * 8] = temp[key];
            }
        }
    }

    // Prints all values from an array.
    fn print_values(&mut self) -> () {
        for pos in 0..self.values.len() {
            let symb: char = self.convert_numb_to_char(self.values[pos]);

            self.print_symbol(pos as u8, symb, false);
        }
    }

    // Adapted (for two displays) value printing in segment.
    fn print_symbol(&mut self, pos: u8, value: char, add_point: bool) -> () {
        let address: (usize, u8) = self.def_address(pos);
        self.display.set_segment_value(address.0, address.1, value, add_point)
    }

    // Adapted (for two displays) value printing in LED.
    fn print_tag(&mut self, pos: u8, value: u8) {
        let address: (usize, u8) = self.def_address(pos);
        self.display.set_led_state(address.0, address.1, value);
    }

    // Defines the display number and segment position for writing the value.
    fn def_address(&mut self, pos: u8) -> (usize, u8) {
        let coeff = pos / 8;
        (coeff as usize, pos - 8 * coeff)
    }

    // Converts a number to a char.
    fn convert_numb_to_char(&mut self, numb: u8) -> char {
        match char::from_digit(numb as u32, 10) {
            Some(x) => { x }
            None => { ' ' }
        }
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

    let mut division: DivisionByTwo<DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> =
        DivisionByTwo::init(
            stbs,
            AnyPin::from(stm.PA2),
            AnyPin::from(stm.PA3),
            rows,
            columns,
            FONTS,
        );

    loop {
        division.run();
    }
}
