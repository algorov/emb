mod requirements;

use defmt::println;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use crate::keyboard::requirements::{KeyFont, KeyState};

const ROW_COUNT: usize = 5;
const COLUMN_COUNT: usize = 4;

pub struct Keyboard<'d> {
    row: [Output<'d, AnyPin>; ROW_COUNT],
    column: [Input<'d, AnyPin>; COLUMN_COUNT],
    state: KeyState,
    key_position: u8,
    pressed_key_now_position: u8,
    pressed_key_was_position: u8,
}

impl<'d> Keyboard<'d> {
    pub(crate) fn default(
        pin1: AnyPin,
        pin2: AnyPin,
        pin3: AnyPin,
        pin4: AnyPin,
        pin5: AnyPin,
        pin6: AnyPin,
        pin7: AnyPin,
        pin8: AnyPin,
        pin9: AnyPin) -> Keyboard<'d> {
        let mut inC1 = Input::new(AnyPin::from(pin1), Pull::Up);
        let mut inC2 = Input::new(AnyPin::from(pin2), Pull::Up);
        let mut inC3 = Input::new(AnyPin::from(pin3), Pull::Up);
        let mut inC4 = Input::new(AnyPin::from(pin4), Pull::Up);
        let mut outR1 = Output::new(AnyPin::from(pin5), Level::High, Speed::Low);
        let mut outR2 = Output::new(AnyPin::from(pin6), Level::High, Speed::Low);
        let mut outR3 = Output::new(AnyPin::from(pin7), Level::High, Speed::Low);
        let mut outR4 = Output::new(AnyPin::from(pin8), Level::High, Speed::Low);
        let mut outR5 = Output::new(AnyPin::from(pin9), Level::High, Speed::Low);

        let row: [Output<AnyPin>; 5] = [outR5, outR4, outR3, outR2, outR1];
        let column: [Input<AnyPin>; 4] = [inC1, inC2, inC3, inC4];
        let mut state: KeyState = KeyState::RELEASED;
        let mut pressed_key_now_position = 255;
        let mut pressed_key_was_position = 255;
        let mut key_position = 255;

        Self { row, column, state, key_position, pressed_key_now_position, pressed_key_was_position }
    }

    pub(crate) fn getKey(&mut self) -> Option<u8> {
        self.read();

        if self.state == KeyState::RELEASED && self.key_position != 255 {
            let temp = self.key_position;
            self.key_position = 255;
            Some(temp)
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

    // Определяет позицию нажатой в данный момент кнопки на клавиатуре.
    pub(crate) fn pressed_now_position(&mut self) -> u8{
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
