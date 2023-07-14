use defmt::export::usize;
use defmt::println;
use embassy_stm32::gpio::{Flex, Level, Output, Pin, Pull, Speed};
use embassy_stm32::{into_ref, Peripheral};
use embassy_time::{Duration, Timer};
use instructions::{BRIGHTNESS, DIGIT_3, DISPLAY_CTRL_INSTR, DISPLAY_ON_INSTR, NULL};
use crate::led_and_key::instructions::{ADDRESS_SET_INSTR, DATA_WRITE_INSTR};

mod instructions;

pub struct LedAndKey<'d, STB: Pin, CLK: Pin, DIO: Pin> {
    pub(crate) stb: Output<'d, STB>,
    pub(crate) clk: Output<'d, CLK>,
    pub(crate) dio: Flex<'d, DIO>,
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
        let mut brightness = BRIGHTNESS;

        stb.set_high();
        dio.set_low();
        clk.set_low();

        dio.set_as_input_output(Speed::Low, Pull::Up);

        let mut driver = Self { stb, dio, clk, brightness };

        driver.write_display_ctrl_instruction();
        driver.push_cmd(0x40);

        driver.write(0, 0x3F);

        driver
    }

    pub(crate) fn set_led_value(&mut self, position: u8, state: u8) -> () {
        self.write((position << 1) + 1, state);
    }

    fn write(&mut self, position: u8, data: u8) -> () {
        self.stb.set_low();

        for i in 0..10 {
            self.set_address_instruction(position + i);
            self.write_byte(data);
        }

        self.stb.set_high();
    }


    // display command: display on, set brightness.
    fn write_display_ctrl_instruction(&mut self) -> () {
        self.push_cmd(DISPLAY_CTRL_INSTR | DISPLAY_ON_INSTR | self.brightness);
    }

    // Sets the value in the memory address.
    fn write_data_instruction(&mut self) -> () {
        // data command: automatic address increment, normal mode.
        self.push_cmd(DATA_WRITE_INSTR);
    }

    // Sets the address to write the value to.
    fn set_address_instruction(&mut self, address: u8) -> () {
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
