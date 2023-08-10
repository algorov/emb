/*
    MSB           LSB
    7 6 5 4 3 2 1 0
   -----------------
    0 1 - - - - - -    Data command
    1 0 - - - - - -    Display control command
    1 1 - - - - - -    Address command

   7.1 Data Command Set

   MSB           LSB
    7 6 5 4 3 2 1 0
   -----------------
    0 1 0 0 0 - 0 0    Write display data
    0 1 0 0 0 - 1 0    Read key scan data
    0 1 0 0 0 0 - -    Auto address increment
    0 1 0 0 0 1 - -    Fixed address


   7.2 Address command set

   MSB           LSB
    7 6 5 4 3 2 1 0
   -----------------
    1 1 0 - A A A A    Address 0x00..0x0F


   7.3 Display Control

   MSB           LSB
    7 6 5 4 3 2 1 0
   -----------------
    1 0 0 0 - 0 0 0    Set the pulse width of 1 / 16
    1 0 0 0 - 0 0 1    Set the pulse width of 2 / 16
    1 0 0 0 - 0 1 0    Set the pulse width of 4 / 16
    1 0 0 0 - 0 1 1    Set the pulse width of 10 / 16
    1 0 0 0 - 1 0 0    Set the pulse width of 11 / 16
    1 0 0 0 - 1 0 1    Set the pulse width of 12 / 16
    1 0 0 0 - 1 1 0    Set the pulse width of 13 / 16
    1 0 0 0 - 1 1 1    Set the pulse width of 14 / 16
    1 0 0 0 0 - - -    Display off
    1 0 0 0 1 - - -    Display on
 */

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DisplayCommand {
    SET_DISPLAY_CTRL_INSTR = 0x80,
    DISPLAY_ON_INSTR = 0x08,
    DISPLAY_OFF_INSTR = 0x00,
    BRIGHTNESS_DEFAULT = 0x07,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DataCommand {
    SET_DATA_INSTR = 0x40,
    DATA_WRITE_INSTR = 0x00,
    DATA_READ_INSTR = 0x02,
    FIXED_ADDRESS = 0x04,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum AddressCommand {
    SET_ADDRESS_INSTR = 0xC0,
    ADDRESS_DEFAULT = 0x00,
}

/*
    ########### Null ###########
 */
pub const NULL: u8 = 0x00;
