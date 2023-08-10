#![no_std]

use embassy_stm32::gpio::{AnyPin, Input, Output};
use config::*;

mod config;

pub struct Keyboard<'d, const ROW_COUNT: usize, const COLUMN_COUNT: usize, const CAPACITY: usize> {
    rows: [Output<'d, AnyPin>; ROW_COUNT],
    columns: [Input<'d, AnyPin>; COLUMN_COUNT],
    fonts: [char; CAPACITY],

    state: KeyState,
    cur_pos: u8,
    prev_pos: u8,
    key_pos: u8,
}

impl<'d, const ROW_COUNT: usize, const COLUMN_COUNT: usize, const CAPACITY: usize> Keyboard<'d, ROW_COUNT, COLUMN_COUNT, CAPACITY> {
    pub fn default(rows: [Output<'d, AnyPin>; ROW_COUNT], columns: [Input<'d, AnyPin>; COLUMN_COUNT], fonts: [char; CAPACITY])
                   -> Keyboard<'d, ROW_COUNT, COLUMN_COUNT, CAPACITY> {
        let mut state: KeyState = KeyState::RELEASED;
        let mut cur_pos = 255;
        let mut prev_pos = 255;
        let mut key_pos: u8 = 255;

        Self { rows, columns, fonts, state, cur_pos, prev_pos, key_pos }
    }

    // Keyboard reading. Returns a char.
    pub fn get_key(&mut self) -> Option<char> {
        self.read();

        if self.state == KeyState::RELEASED && self.key_pos != 255 && self.key_pos < CAPACITY as u8 {
            let code: char = self.fonts[self.key_pos as usize];
            self.key_pos = 255;

            Some(code)
        } else {
            None
        }
    }

    fn read(&mut self) -> () {
        self.cur_pos = self.scan_keys();

        if self.cur_pos != 255 && self.cur_pos != self.prev_pos {
            self.state = KeyState::PRESSED;
        }

        if self.cur_pos == 255 && self.cur_pos != self.prev_pos {
            self.state = KeyState::RELEASED;
            self.key_pos = self.prev_pos;
        }

        self.prev_pos = self.cur_pos;
    }

    /*
     The algorithm for calling each key on the fact of pressing,
     returns the position of the key on the keyboard.
     */
    fn scan_keys(&mut self) -> u8 {
        let mut pos: u8 = 255;

        for i in 0..ROW_COUNT {
            self.rows[i].set_low();

            for j in 0..COLUMN_COUNT {
                if !self.columns[j].is_high() {
                    pos = (COLUMN_COUNT * i + j) as u8;
                }
            }

            self.rows[i].set_high();
        }

        pos
    }
}