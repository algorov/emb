// const a: [char; 20] = ['']

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum KeyFont {
    F1 = 0x00,
    F2 = 0x01,
    GRATING = 0x02,
    STAR = 0x03,
    ONE = 0x04,
    TWO = 0x05,
    THREE = 0x06,
    ARROW_UP = 0x07,
    FOUR = 0x08,
    FIFE = 0x09,
    SIX = 0x0A,
    ARROW_DOWN = 0x0B,
    SEVEN = 0x0C,
    EIGHT = 0x0D,
    NINE = 0x0E,
    ESC = 0x0F,
    ARROW_LEFT = 0x10,
    ZERO = 0x11,
    ARROW_RIGHT = 0x12,
    ENTER = 0x13,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}


