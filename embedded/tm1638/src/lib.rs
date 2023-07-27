#![no_std]

use embassy_stm32::gpio::{AnyPin, Flex, Level, Output, Pull, Speed};

use AddressCommand::*;
use BrightnessLevel::*;
use config::*;
use DataCommand::*;
use DigitSymbol::*;
use DisplayCommand::*;
use fonts::*;
use instructions::*;

mod instructions;
mod fonts;
mod config;

pub struct LedAndKey<'d, const COUNT: usize> {
    stbs: [Output<'d, AnyPin>; COUNT],
    clk: Output<'d, AnyPin>,
    dio: Flex<'d, AnyPin>,
}

impl<'d, const COUNT: usize> LedAndKey<'d, COUNT> {
    pub fn default(stbs: [Output<'d, AnyPin>; COUNT], clk: AnyPin, dio: AnyPin) -> LedAndKey<'d, COUNT> {
        let mut clk: Output<AnyPin> = Output::new(clk, Level::Low, Speed::Low);
        let mut dio: Flex<AnyPin> = Flex::new(dio);

        dio.set_as_output(Speed::Low); // By default, in data transfer mode.
        dio.set_low();

        let mut driver: LedAndKey<COUNT> = Self { stbs, clk, dio };

        for id in 0..COUNT {
            driver.push_display_ctrl_instr(
                id,
                true,
                L7 as u8,
            );
            driver.cleanup(id);
        }

        driver
    }

    // Draws the entered word on segments.
    pub fn set_word_in_segments(&mut self, id: usize, data: &mut [char; SEGMENT_COUNT]) -> () {
        for pos in 0..SEGMENT_COUNT {
            self.set_segment_value(
                id,
                pos as u8,
                data[pos],
                false,
            );
        }
    }

    // Includes display.
    pub fn display_on(&mut self, id: usize) -> () {
        if id < COUNT {
            self.push_display_ctrl_instr(
                id,
                true,
                L7 as u8,
            );
        }
    }

    // Disable display.
    pub fn display_off(&mut self, id: usize) -> () {
        if id < COUNT {
            self.push_display_ctrl_instr(
                id,
                false,
                NULL,
            );
        }
    }

    // Set display registers to zero.
    pub fn cleanup(&mut self, id: usize) -> () {
        self.push_data_write_instr(id, true);
        self.stbs[id].set_low();
        self.push_address_instr(ADDRESS_DEFAULT as u8);

        for i in 0..16 {
            self.write_byte(NULL);
        }

        self.stbs[id].set_high();
    }

    // Sets the brightness of the LEDs and segments.
    pub fn set_brightness(&mut self, id: usize, level: BrightnessLevel) -> () {
        if id < COUNT {
            self.push_display_ctrl_instr(
                id,
                true,
                level as u8,
            );
        }
    }

    // Sets the value of the segment.
    pub fn set_segment_value(&mut self, id: usize, position: u8, value: char, add_point: bool) -> () {
        if id < COUNT && position < SEGMENT_COUNT as u8 {
            let value: DigitSymbol = match value.to_ascii_lowercase() {
                '0' => { DIGIT_0 }
                '1' => { DIGIT_1 }
                '2' => { DIGIT_2 }
                '3' => { DIGIT_3 }
                '4' => { DIGIT_4 }
                '5' => { DIGIT_5 }
                '6' => { DIGIT_6 }
                '7' => { DIGIT_7 }
                '8' => { DIGIT_8 }
                '9' => { DIGIT_9 }
                'a' => { DIGIT_A }
                'b' => { DIGIT_b }
                'c' => { DIGIT_C }
                'd' => { DIGIT_d }
                'e' => { DIGIT_E }
                'f' => { DIGIT_F }
                'g' => { DIGIT_G }
                'h' => { DIGIT_H }
                'i' => { DIGIT_1 }
                'j' => { DIGIT_J }
                'k' => { DIGIT_H }
                'l' => { DIGIT_L }
                'm' => { DIGIT_M }
                'n' => { DIGIT_N }
                'o' => { DIGIT_0 }
                'p' => { DIGIT_P }
                'q' => { DIGIT_Q }
                'r' => { DIGIT_R }
                's' => { DIGIT_5 }
                't' => { DIGIT_T }
                'u' => { DIGIT_U }
                'v' => { DIGIT_V }
                'w' => { DIGIT_W }
                'x' => { DIGIT_H }
                'y' => { DIGIT_Y }
                'z' => { DIGIT_2 }
                _ => { EMPTY }
            };
            self.write(
                id,
                position << 1,
                value as u8 | if add_point { POINT as u8 } else { NULL },
            );
        }
    }

    /*
     Sets the LED's value.
     @position: 0..7
     @state: 0 - off, otherwise - on
    */
    pub fn set_led_state(&mut self, id: usize, position: u8, state: u8) -> () {
        if id < COUNT && position < LED_COUNT as u8 {
            self.write(
                id,
                (position << 1) + 1,
                if state == 0 { 0 } else { 1 },
            );
        }
    }

    /*
     Determines the key pressed.
     From left to right: 1 - pressed, 0 - otherwise.
    */
    pub fn get_pressed_keys(&mut self, id: usize, key_states: &mut [u8; KEY_COUNT]) -> () {
        let data: u32 = self.scan_keys(id);
        for i in 0..4 {
            key_states[i] = if (data >> (8 * i) & 1) == 1 { 1 } else { 0 };
            key_states[i + 4] = if (data >> (8 * i + 4) & 1) == 1 { 1 } else { 0 };
        }
    }

    /*
     Write a byte to the display register.
     @position: 0..15
     */
    fn write(&mut self, id: usize, position: u8, data: u8) -> () {
        self.push_data_write_instr(id, false);

        self.stbs[id].set_low();
        self.push_address_instr(position);
        self.write_byte(data);
        self.stbs[id].set_high();
    }

    // Reads the values of each button.
    fn scan_keys(&mut self, id: usize) -> u32 {
        self.stbs[id].set_low();
        self.write_byte(SET_DATA_INSTR as u8 | DATA_READ_INSTR as u8);

        let mut data: u32 = 0;
        for i in 0..4 { data |= (self.read_byte() as u32) << (i * 8); }

        self.stbs[id].set_high();

        data
    }

    // Display configuration instruction.
    fn push_display_ctrl_instr(&mut self, id: usize, display_on: bool, brightness: u8) -> () {
        self.push_instruction(
            id,
            SET_DISPLAY_CTRL_INSTR as u8 |
                if display_on { DISPLAY_ON_INSTR as u8 } else { DISPLAY_OFF_INSTR as u8 } |
                brightness,
        );
    }

    /*
     Sends instructions for recording.
     Data command:
     if @autoincrement is true, then AUTOMATIC address increment mode, else fixed address mode.
     */
    fn push_data_write_instr(&mut self, id: usize, autoincrement: bool) -> () {
        self.push_instruction(
            id,
            SET_DATA_INSTR as u8 |
                if autoincrement { NULL } else { FIXED_ADDRESS as u8 } |
                DATA_WRITE_INSTR as u8,
        );
    }

    // Sets the address to write the value to.
    fn push_address_instr(&mut self, address: u8) -> () {
        self.write_byte(SET_ADDRESS_INSTR as u8 | address);
    }

    // Push a instruction to the device.
    fn push_instruction(&mut self, id: usize, instruction: u8) -> () {
        self.stbs[id].set_low();
        self.write_byte(instruction);
        self.stbs[id].set_high();
    }

    // Write 1 byte of information to the device.
    fn write_byte(&mut self, byte: u8) -> () {
        for i in 0..8 {
            self.clk.set_low();

            if (byte >> i) & 1 == 0 { self.dio.set_low(); } else { self.dio.set_high(); }

            self.clk.set_high();
        }
    }

    // Read 1 byte of information from device.
    fn read_byte(&mut self) -> u8 {
        self.dio.set_as_input(Pull::Up);

        let mut byte: u8 = 0;
        for i in 0..8 {
            self.clk.set_low();
            self.clk.set_high();

            if self.dio.is_high() { byte |= 1 << i; }
        }

        self.dio.set_as_output(Speed::Low);

        byte
    }
}
