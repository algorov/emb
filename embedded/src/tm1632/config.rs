/*
    ####################### Can be changed #######################
*/
pub(crate) const DISPLAY_COUNT: usize = 1;


/*
    ####################### I forbid changing #######################
 */
pub(crate) const LED_COUNT: usize = 8;
pub(crate) const SEGMENT_COUNT: usize = LED_COUNT;

pub(crate) const KEY_COUNT: usize = 8 * DISPLAY_COUNT;

#[derive(Copy, Clone, PartialEq)]
pub enum BrightnessLevel {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
}