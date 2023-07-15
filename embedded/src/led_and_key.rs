use embassy_stm32::gpio::{Flex, Level, Output, Pin, Pull, Speed};
use embassy_stm32::{into_ref, Peripheral};
use instructions::{BRIGHTNESS, DIGIT_2, DIGIT_3, DIGIT_5, DISPLAY_CTRL_INSTR, DISPLAY_ON_INSTR, NULL};
use crate::led_and_key::instructions::{ADDRESS_SET_INSTR, DATA_WRITE_INSTR, DISPLAY_OFF_INSTR};

mod instructions;

/*
 TODO:
  Манипуляция с дисплеем                [+]
  Манипуляция с яркостью                [+]
  Манипуляция с сегментом               [+]
  Манипуляция со светодиодом            [+]
  Манипуляция с кнопками                [-]
  Проверка значений на этапе компиляции [-]
 */

pub struct LedAndKey<'d, STB: Pin, CLK: Pin, DIO: Pin> {
    stb: Output<'d, STB>,
    clk: Output<'d, CLK>,
    dio: Flex<'d, DIO>,
    display: bool,
    brightness: u8,
}

impl<'d, STB: Pin, CLK: Pin, DIO: Pin> LedAndKey<'d, STB, CLK, DIO> {
    pub(crate) fn new(stb: impl Peripheral<P=STB> + 'static,
                      clk: impl Peripheral<P=CLK> + 'static,
                      dio: impl Peripheral<P=DIO> + 'static) -> LedAndKey<'d, STB, CLK, DIO> {
        into_ref!(stb, clk, dio);

        let mut clk: Output<CLK> = Output::new(clk, Level::Low, Speed::Low);
        let mut dio: Flex<DIO> = Flex::new(dio);
        let mut stb: Output<STB> = Output::new(stb, Level::Low, Speed::Low);
        let mut display: bool = true;
        let mut brightness: u8 = BRIGHTNESS;

        stb.set_high();
        dio.set_low();
        clk.set_low();
        dio.set_as_input_output(Speed::Low, Pull::Up);

        let mut driver = Self { stb, dio, clk, display, brightness };
        driver.push_display_ctrl_instr();
        driver.cleanup();

        driver
    }

    // Sets all display registers to zero.
    pub(crate) fn cleanup(&mut self) -> () {
        self.push_data_write_instr();
        self.stb.set_low();
        self.push_address_instr(NULL);

        for i in 0..15 {
            self.write_byte(NULL);
        }

        self.stb.set_low();
    }

    // Includes display.
    pub(crate) fn display_on(&mut self) -> () {
        self.display = true;
        self.push_display_ctrl_instr();
    }

    // Disable display.
    pub(crate) fn display_off(&mut self) -> () {
        self.display = false;
        self.push_display_ctrl_instr();
    }

    /*
     Sets the brightness of the LEDs and segments.
     @value: 0..7
     */
    pub(crate) fn set_brightness(&mut self, value: u8) -> () {
        self.brightness = value;
        self.push_display_ctrl_instr();
    }

    /*
     Sets the value of the segment.
     @position: 0..7
     @state: 0..9 and A-Z
     */
    pub(crate) fn set_segment_value(&mut self, position: u8, state: u8) -> () {
        self.write(position << 1, state);
    }

    /*
     Sets the LED's value.
     @position: 0..7
     @state: 0 or 1
     */
    pub(crate) fn set_led_value(&mut self, position: u8, state: u8) -> () {
        self.write((position << 1) + 1, state);
    }

    /*
     Write a byte to the display register.
     @position: 0..15
     */
    fn write(&mut self, position: u8, data: u8) -> () {
        self.push_data_write_instr();

        self.stb.set_low();
        self.push_address_instr(position);
        self.write_byte(data);
        self.stb.set_high();
    }

    /*
     Display configuration instruction.
     Default:
     ~ display on
     ~ brightness max (0x07)
     */
    fn push_display_ctrl_instr(&mut self) -> () {
        self.stb.set_high();
        self.dio.set_low();
        self.clk.set_low();

        let display_instr: u8;

        if self.display { display_instr = DISPLAY_ON_INSTR; } else { display_instr = DISPLAY_OFF_INSTR }

        self.push_instruction(DISPLAY_CTRL_INSTR | display_instr | self.brightness);
    }

    /*
     Sets the value in the memory address.
     Data command: AUTOMATIC address increment, normal mode.
     */
    fn push_data_write_instr(&mut self) -> () {
        self.push_instruction(DATA_WRITE_INSTR);
    }

    // Sets the address to write the value to.
    fn push_address_instr(&mut self, address: u8) -> () {
        self.write_byte(ADDRESS_SET_INSTR | address);
    }

    // Push a instruction to the TM1638.
    fn push_instruction(&mut self, instruction: u8) -> () {
        self.stb.set_low();
        self.write_byte(instruction);
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
