/*
    ########### Segments ###########

    The bits are displayed by mapping bellow

                -- [0] --
                |       |
               [5]      [1]
                -- [6] --
               [4]     [2]
                |       |
                -- [3] --  .[7]

    Example:
             |76543210|
            0b01101101 [BIN] = 0x6D [HEX] = 109 [DEC] = show "5" [SYMBOL]



    ####################### Can be changed #######################
*/
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DigitSymbol {
    EMPTY = 0x00,
    DIGIT_0 = 0x3F, // same as "O"
    DIGIT_1 = 0x06, // same as "I"
    DIGIT_2 = 0x5B, // same as "Z"
    DIGIT_3 = 0x4F,
    DIGIT_4 = 0x66,
    DIGIT_5 = 0x6D, // same as "S"
    DIGIT_6 = 0x7D,
    DIGIT_7 = 0x07,
    DIGIT_8 = 0x7F,
    DIGIT_9 = 0x6F,
    DIGIT_A = 0x77,
    DIGIT_b = 0x7c,
    DIGIT_C = 0x39,
    DIGIT_d = 0x5E,
    DIGIT_E = 0x79,
    DIGIT_F = 0x71,
    DIGIT_G = 0x3D,
    DIGIT_H = 0x76, // same as "K" and "X"
    DIGIT_J = 0x1F,
    DIGIT_L = 0x38,
    DIGIT_M = 0x15,
    DIGIT_N = 0x54,
    DIGIT_P = 0x73,
    DIGIT_Q = 0x67,
    DIGIT_R = 0x50,
    DIGIT_T = 0x78,
    DIGIT_U = 0x3E,
    DIGIT_V = 0x1C,
    DIGIT_W = 0x2A,
    DIGIT_Y = 0x6E,
}