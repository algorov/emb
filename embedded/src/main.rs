#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::export::{char, usize};
use defmt::println;
use embassy_executor::Spawner;
use embassy_stm32::{self};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_stm32::Peripherals;

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
        fonts: [u8; FONT_CAPACITY],
    ) -> DivisionByTwo<'d, DISPLAY_COUNT, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> {
        let display: LedAndKey<'d, DISPLAY_COUNT> = LedAndKey::default(stbs, clk, dio);
        let keyboard: Keyboard<'d, ROW_COUNT, COLUMN_COUNT, FONT_CAPACITY> = Keyboard::default(rows, columns, fonts);

        let mut values: [u8; 16] = [0; 16];
        let mut key_states: [u8; 16] = [0; 16];

        Self { display, keyboard, values, key_states }
    }

    pub fn run(&mut self) -> () {
        self.begin();

        loop {
            let states = self.scan_keys();
            match states.0 {
                '1' => {
                    self.step1();
                }
                '2' => {
                    self.step2();
                }
                '3' => {
                    self.step3();
                }
                '4' => {
                    self.step4();
                }
                '8' => {
                    println!("End.");
                    break;
                }
                _ => {}
            }
        }
        // let states = self.scan_keys();
        // if states.0 == 'f'
        // for i in self.scan_keys() {
        //     println!("{:?}", i);
        // }
    }

    fn step1(&mut self) -> () {
        println!("Step 1");
        let mut pointer: u8 = 0;
        loop {
            if pointer <= 15 {
                let mut key: char = self.scan_keyboard();
                match key {
                    '0' => {
                        self.values[pointer as usize] = 0;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '1' => {
                        self.values[pointer as usize] = 1;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '2' => {
                        self.values[pointer as usize] = 2;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '3' => {
                        self.values[pointer as usize] = 3;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '4' => {
                        self.values[pointer as usize] = 4;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '5' => {
                        self.values[pointer as usize] = 5;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '6' => {
                        self.values[pointer as usize] = 6;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '7' => {
                        self.values[pointer as usize] = 7;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '8' => {
                        self.values[pointer as usize] = 8;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '9' => {
                        self.values[pointer as usize] = 9;
                        self.print_symbol(pointer, key, false);
                        pointer += 1;
                    }
                    '<' => {
                        println!("Step 1 end");
                        break;
                    }
                    _ => {}
                }
            }

            println!("{:?}", self.values);
        }
    }

    fn step2(&mut self) -> () {
        println!("Step 2");
    }

    fn step3(&mut self) -> () {
        println!("Step 3");
    }

    fn step4(&mut self) -> () {
        println!("Step 4");
    }

    fn begin(&mut self) -> () {
        self.print_values();
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
    fn print_flag(&mut self, pos: u8, value: u8) {
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

    division.run();

    loop {
        // match matrix_keyboard.get_key() {
        //     Some(x) => println!("Result: {}", char::from(x)),
        //     None => {}
        // }
    }
}
