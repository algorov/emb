/*
    ####################### I forbid changing #######################
 */
pub const LED_COUNT: usize = 8;
pub const SEGMENT_COUNT: usize = 8;
pub const KEY_COUNT: usize = 8;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum BrightnessLevel {
    L1 = 0,
    L2 = 1,
    L3 = 2,
    L4 = 3,
    L5 = 4,
    L6 = 5,
    L7 = 6,
    L8 = 7,
}