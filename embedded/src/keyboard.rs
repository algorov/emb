pub(crate) mod config;

use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use crate::keyboard::config::*;

pub struct Keyboard<'d> {
    rows: [Output<'d, AnyPin>; ROW_COUNT],
    columns: [Input<'d, AnyPin>; COLUMN_COUNT],
    fonts: [u8; FONTS_CAPACITY],

    state: KeyState,
    key_now_position: u8,
    key_was_position: u8,
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

        let columns: [Input<AnyPin>; COLUMN_COUNT] = [in_c1, in_c2, in_c3, in_c4];
        let rows: [Output<AnyPin>; ROW_COUNT] = [out_r5, out_r4, out_r3, out_r2, out_r1];
        let fonts: [u8; FONTS_CAPACITY] = FONTS;

        let mut state: KeyState = KeyState::RELEASED;
        let mut key_now_position = 255;
        let mut key_was_position = 255;
        let mut key_position: u8 = 255;

        Self { rows, columns, fonts, state, key_now_position, key_was_position, key_position }
    }

    // Keyboard reading. Returns the ASCII-code of a character.
    pub(crate) fn get_key(&mut self) -> Option<u8> {
        self.read();

        if self.state == KeyState::RELEASED && self.key_position != 255 {
            let code: u8 = FONTS[self.key_position as usize];
            self.key_position = 255;

            Some(code)
        } else {
            None
        }
    }

    fn read(&mut self) -> () {
        self.key_now_position = self.pressed_now_position();

        if self.key_now_position != 255 && self.key_now_position != self.key_was_position {
            self.state = KeyState::PRESSED;
        }

        if self.key_now_position == 255 && self.key_now_position != self.key_was_position {
            self.state = KeyState::RELEASED;
            self.key_position = self.key_was_position;
        }

        self.key_was_position = self.key_now_position;
    }

    /*
     The algorithm for calling each button on the fact of pressing,
     returns the position of the button on the keyboard.
     */
    pub(crate) fn pressed_now_position(&mut self) -> u8 {
        let mut position: u8 = 255;

        for i in 0..ROW_COUNT {
            self.rows[i].set_low();

            for j in 0..COLUMN_COUNT {
                if !self.columns[j].is_high() {
                    position = (i * 4 + j) as u8;
                }
            }

            self.rows[i].set_high();
        }

        position
    }
}
