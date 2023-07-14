pub(crate) const NULL: u8 = 0x00; // 0b00000000

pub(crate) const DATA_WRITE_INSTR: u8 = 0x40; // 0b01000000 // запись в регистры отображения

pub(crate) const ADDRESS_SET_INSTR: u8 = 0xC0; // 0b11000000
pub(crate) const ADDRESS: u8 = 0x05;           // 0b00000101 (5) Адрес можем изменять от 0 до 15

pub(crate) const DISPLAY_CTRL_INSTR: u8 = 0x80; // 0b10000000
pub(crate) const DISPLAY_ON_INSTR: u8 = 0x08;   // 0b10001000
pub(crate) const BRIGHTNESS: u8 = 0x01;         // 0b00000111 (0) Можно изменять яркость от 0 до 7

/*
          A
         ---
      F |   | B
         -G-
      E |   | C
         ---  * H
          D

      HGFEDCBA
    0b01101101 = 0x6D = 109 = show "5"

*/

// For LED segments;
const DIGIT_0: u8 = 0x3F; // 0
const DIGIT_1: u8 = 0x06; // 1
pub(crate) const DIGIT_2: u8 = 0x5B; // 2
pub(crate) const DIGIT_3: u8 = 0x4F; // 3
const DIGIT_4: u8 = 0x66; // 4
const DIGIT_5: u8 = 0x6D; // 5
const DIGIT_6: u8 = 0x7D; // 6
const DIGIT_7: u8 = 0x07; // 7
const DIGIT_8: u8 = 0x7F; // 8
const DIGIT_9: u8 = 0x6F; // 9
const DIGIT_A: u8 = 0x77; // A
const DIGIT_b: u8 = 0x7c; // b
const DIGIT_C: u8 = 0x39; // C
const DIGIT_d: u8 = 0x5E; // d
const DIGIT_E: u8 = 0x79; // E
const DIGIT_F: u8 = 0x71; // F