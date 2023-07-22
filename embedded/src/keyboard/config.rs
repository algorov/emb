pub(crate) const ROW_COUNT: usize = 5;
pub(crate) const COLUMN_COUNT: usize = 4;
pub(crate) const CAPACITY: usize = ROW_COUNT * COLUMN_COUNT;

pub(crate) const KEY_CHARS_DEFAULT: [u8; 20] = [
    0x66, 0x46, 0x23, 0x2A,
    0x31, 0x32, 0x33, 0x18,
    0x34, 0x35, 0x36, 0x19,
    0x37, 0x38, 0x39, 0x1B,
    0x3C, 0x30, 0x3E, 0x0D,
];

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}


