pub(crate) mod config;

use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use crate::keyboard::config::*;

pub struct Keyboard<'d> {
    row: [Output<'d, AnyPin>; ROW_COUNT],
    column: [Input<'d, AnyPin>; COLUMN_COUNT],
    fonts: [u8; FONTS_CAPACITY],

    state: KeyState,
    pressed_key_now_position: u8,
    pressed_key_was_position: u8,
    key_position: u8,
}

impl<'d> Keyboard<'d> {
    // For 5x4 keyboard.
    pub(crate) fn default(
        pin1: AnyPin, pin2: AnyPin, pin3: AnyPin, pin4: AnyPin,
        pin5: AnyPin, pin6: AnyPin, pin7: AnyPin, pin8: AnyPin, pin9: AnyPin) -> Keyboard<'d> {
        let mut in_c1 = Input::new(pin1, Pull::Up);
        let mut in_c2 = Input::new(pin2, Pull::Up);
        let mut in_c3 = Input::new(pin3, Pull::Up);
        let mut in_c4 = Input::new(pin4, Pull::Up);
        let mut out_r1 = Output::new(pin5, Level::High, Speed::Low);
        let mut out_r2 = Output::new(pin6, Level::High, Speed::Low);
        let mut out_r3 = Output::new(pin7, Level::High, Speed::Low);
        let mut out_r4 = Output::new(pin8, Level::High, Speed::Low);
        let mut out_r5 = Output::new(pin9, Level::High, Speed::Low);
        let mut column: [Input<AnyPin>; COLUMN_COUNT] = [in_c1, in_c2, in_c3, in_c4];
        let mut row: [Output<AnyPin>; ROW_COUNT] = [out_r5, out_r4, out_r3, out_r2, out_r1];

        let fonts: [u8; FONTS_CAPACITY] = FONTS;

        let mut state: KeyState = KeyState::RELEASED;
        let mut pressed_key_now_position = 255;
        let mut pressed_key_was_position = 255;
        let mut key_position: u8 = 255;

        Self { row, column, fonts, state, pressed_key_now_position, pressed_key_was_position, key_position }
    }

    // Keyboard reading.
    pub(crate) fn get_key(&mut self) -> Option<u8> {
        self.read();

        if self.state == KeyState::RELEASED && self.key_position != 255 {
            let askii_code: u8 = FONTS[self.key_position as usize];
            self.key_position = 255;
            Some(askii_code)
        } else {
            None
        }
    }

    fn read(&mut self) -> () {
        self.pressed_key_now_position = self.pressed_now_position();

        if self.pressed_key_now_position != 255 && self.pressed_key_now_position != self.pressed_key_was_position {
            self.state = KeyState::PRESSED;
        }

        if self.pressed_key_now_position == 255 && self.pressed_key_now_position != self.pressed_key_was_position {
            self.state = KeyState::RELEASED;
            self.key_position = self.pressed_key_was_position;
        }

        self.pressed_key_was_position = self.pressed_key_now_position;
    }

    // Determines the position of the currently pressed button on the keyboard.
    pub(crate) fn pressed_now_position(&mut self) -> u8 {
        let mut position: u8 = 255;

        for i in 0..ROW_COUNT {
            self.row[i].set_low();

            for j in 0..COLUMN_COUNT {
                if !self.column[j].is_high() {
                    position = (i * 4 + j) as u8;
                }
            }

            self.row[i].set_high();
        }

        position
    }
}
