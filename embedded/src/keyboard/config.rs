/*
    ####################### Can be changed #######################
 */
pub(crate) const ROW_COUNT: usize = 5;
pub(crate) const COLUMN_COUNT: usize = 4;

// Chars for each position on the keyboard, presented in ASCII-format.
pub(crate) const FONTS: [u8; FONTS_CAPACITY] = [
    0x66, 0x46, 0x23, 0x2A,
    0x31, 0x32, 0x33, 0x5E,
    0x34, 0x35, 0x36, 0x5F,
    0x37, 0x38, 0x39, 0x1B,
    0x3C, 0x30, 0x3E, 0xAC,
];


/*
    ####################### I forbid changing #######################
 */
pub(crate) const FONTS_CAPACITY: usize = ROW_COUNT * COLUMN_COUNT;

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}