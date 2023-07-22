use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};

const ROW_COUNT: usize = 5;
const COLUMN_COUNT: usize = 4;

pub struct Keyboard<'d> {
    row: [Output<'d, AnyPin>; ROW_COUNT],
    column: [Input<'d, AnyPin>; COLUMN_COUNT],
    pub pressed_key_now: u8,
}

impl<'d> Keyboard<'d> {
    pub(crate) fn new(
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
        let mut pressed_key_now = 255;

        Self { row, column, pressed_key_now }
    }

    pub(crate) fn pressed_now_position(&mut self) -> () {
        for i in 0..ROW_COUNT {
            self.row[i].set_low();

            for j in 0..COLUMN_COUNT {
                if !self.column[j].is_high() {
                    self.pressed_key_now = (i * 4 + j) as u8;
                }
            }

            self.row[i].set_high();
        }
    }
}
