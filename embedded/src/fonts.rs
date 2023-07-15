/*
    ########### Segments ###########

    The bits are displayed by mapping bellow

                 -- 0 --
                |       |
                5       1
                 -- 6 --
                4       2
                |       |
                 -- 3 --  .7

    Example:
             [76543210]
            0b01101101 = 0x6D = 109 = show "5"


    ########### Numbers ###########
*/
pub(crate) const DIGIT_1: u8 = 0x06; // 1
pub(crate) const DIGIT_2: u8 = 0x5B; // 2
pub(crate) const DIGIT_3: u8 = 0x4F; // 3
pub(crate) const DIGIT_4: u8 = 0x66; // 4
pub(crate) const DIGIT_5: u8 = 0x6D; // 5
pub(crate) const DIGIT_6: u8 = 0x7D; // 6
pub(crate) const DIGIT_7: u8 = 0x07; // 7
pub(crate) const DIGIT_8: u8 = 0x7F; // 8
pub(crate) const DIGIT_9: u8 = 0x6F; // 9
pub(crate) const DIGIT_0: u8 = 0x3F; // 0

/*
    ########### Letters ###########
 */
pub(crate) const DIGIT_A: u8 = 0x77; // A
pub(crate) const DIGIT_b: u8 = 0x7c; // b
pub(crate) const DIGIT_C: u8 = 0x39; // C
pub(crate) const DIGIT_d: u8 = 0x5E; // d
pub(crate) const DIGIT_E: u8 = 0x79; // E
pub(crate) const DIGIT_F: u8 = 0x71; // F
pub(crate) const DIGIT_G: u8 = 0x3D; // G
pub(crate) const DIGIT_H: u8 = 0x76; // H
pub(crate) const DIGIT_I: u8 = 0x06; // I
pub(crate) const DIGIT_J: u8 = 0x1F; // J
pub(crate) const DIGIT_K: u8 = 0x76; // K (same as H)
pub(crate) const DIGIT_L: u8 = 0x38; // L
pub(crate) const DIGIT_M: u8 = 0x15; // M
pub(crate) const DIGIT_N: u8 = 0x54; // n
pub(crate) const DIGIT_O: u8 = 0x3F; // O
pub(crate) const DIGIT_P: u8 = 0x73; // P
pub(crate) const DIGIT_Q: u8 = 0x67; // Q
pub(crate) const DIGIT_R: u8 = 0x50; // r
pub(crate) const DIGIT_S: u8 = 0x6D; // S
pub(crate) const DIGIT_T: u8 = 0x78; // t
pub(crate) const DIGIT_U: u8 = 0x3E; // U
pub(crate) const DIGIT_V: u8 = 0x1C; // V
pub(crate) const DIGIT_W: u8 = 0x2A; // W
pub(crate) const DIGIT_X: u8 = 0x76; // X (same as H)
pub(crate) const DIGIT_Y: u8 = 0x6E; // Y
pub(crate) const DIGIT_Z: u8 = 0x5B; // Z

/*
    ########### Other symbols ###########
 */
pub(crate) const DIGIT_POINT: u8 = 0x80; // .