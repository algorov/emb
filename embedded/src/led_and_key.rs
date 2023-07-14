use core::f64::DIGITS;
use defmt::export::usize;
use defmt::println;
use embassy_stm32::gpio::{Flex, Level, Output, Pin, Pull, Speed};
use embassy_stm32::{into_ref, Peripheral};
use embassy_time::{Duration, Timer};
use instructions::{BRIGHTNESS, DIGIT_2, DIGIT_3, DIGIT_5, DISPLAY_CTRL_INSTR, DISPLAY_ON_INSTR, NULL};
use crate::led_and_key::instructions::{ADDRESS_SET_INSTR, DATA_WRITE_INSTR, DISPLAY_OFF_INSTR};

mod instructions;

pub struct LedAndKey<'d, STB: Pin, CLK: Pin, DIO: Pin> {
    pub(crate) stb: Output<'d, STB>,
    pub(crate) clk: Output<'d, CLK>,
    pub(crate) dio: Flex<'d, DIO>,
    pub(crate) display: bool,
    pub(crate) brightness: u8,
}

impl <'d, STB: Pin, CLK: Pin, DIO: Pin> LedAndKey<'d, STB, CLK, DIO> {
    pub(crate) fn new(stb: impl Peripheral<P=STB> + 'static,
                      clk: impl Peripheral<P=CLK> + 'static,
                      dio: impl Peripheral<P=DIO> + 'static) -> LedAndKey<'d, STB, CLK, DIO> {
        into_ref!(stb, clk, dio);

        let mut clk = Output::new(clk, Level::Low, Speed::Low);
        let mut dio = Flex::new(dio);
        let mut stb = Output::new(stb, Level::Low, Speed::Low);
        let mut display = true;
        let mut brightness = BRIGHTNESS;

        stb.set_high();
        dio.set_low();
        clk.set_low();

        dio.set_as_input_output(Speed::Low, Pull::Up);

        let mut driver = Self { stb, dio, clk, display, brightness };

        driver.push_display_ctrl_instr();

        driver
    }

    // Includes display.
    pub(crate) fn display_on(&mut self) {
        self.display = true;
        self.push_display_ctrl_instr();
    }

    // Disable display.
    pub(crate) fn display_off(&mut self) {
        self.display = false;
        self.push_display_ctrl_instr();
    }

    /*
     Sets the brightness of the LEDs and segments.
     @value: 0..7
     */
    pub(crate) fn set_brightness(&mut self, value: u8) {
        self.brightness = value;
        self.push_display_ctrl_instr();
    }

    /*
     Sets the value of the segment.
     @position: 0..7
     @state: 0...9 and A-Z
     */
    pub(crate) fn set_segment_value(&mut self, position: u8, state: u8) -> () {
        self.write(position << 1, state);
    }

    /*
     Sets the value of the LED.
     @position: 0..7
     @state: 0 or 1
     */
    pub(crate) fn set_led_value(&mut self, position: u8, state: u8) -> () {
        self.write((position << 1) + 1, state);
    }

    // Write a byte to the display register.
    fn write(&mut self, position: u8, data: u8) -> () {
        self.push_write_data_instr();

        self.stb.set_low();

        self.push_address_instr(position);
        self.write_byte(data);

        self.stb.set_high();
    }

    // Display command: display on, set brightness.
    fn push_display_ctrl_instr(&mut self) -> () {
        self.stb.set_high();
        self.dio.set_low();
        self.clk.set_low();

        let display_instr: u8;

        if self.display { display_instr = DISPLAY_ON_INSTR; } else { display_instr = DISPLAY_OFF_INSTR}

        self.push_cmd(DISPLAY_CTRL_INSTR | display_instr | self.brightness);
    }

    /*
     Sets the value in the memory address.
     Data command: automatic address increment, normal mode.
     */
    fn push_write_data_instr(&mut self) -> () {
        self.push_cmd(DATA_WRITE_INSTR);
    }

    // Sets the address to write the value to.
    fn push_address_instr(&mut self, address: u8) -> () {
        self.write_byte(ADDRESS_SET_INSTR | address);
    }

    // Push a command to the TM1638.
    fn push_cmd(&mut self, cmd: u8) -> () {
        self.stb.set_low();
        self.write_byte(cmd);
        self.stb.set_high();
    }

    // Write 1 byte of information to the TM1638.
    fn write_byte(&mut self, byte: u8) -> () {
        for i in 0..8 {
            self.clk.set_low();

            if (byte >> i) & 1 == 0 { self.dio.set_low(); } else { self.dio.set_high(); }

            self.clk.set_high();
        }
    }
}
