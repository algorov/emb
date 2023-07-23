use embassy_stm32::gpio::{AnyPin, Flex, Level, Output, Pin, Pull, Speed};
use embassy_stm32::{into_ref, Peripheral};
use crate::tm1632::config::DISPLAY_COUNT;
use crate::tm1632::fonts::DigitSymbol;
use crate::tm1632::instructions::{AddressCommand, DataCommand, DisplayCommand};

mod instructions;
pub mod fonts;
mod config;

/*
 TODO:
  • Манипуляция с дисплеем                                                [+]
  • Манипуляция с яркостью                                                [+]
  • Манипуляции с сегментами                                              [+]
  • Манипуляции со светодиодами                                           [+]
  • Манипуляции с кнопками                                                [+]
  • Проверка значений на этапе компиляции                                 [-]
 */

pub struct LedAndKey<'d> {
    stbs: [Output<'d, AnyPin>; DISPLAY_COUNT],
    clk: Output<'d, AnyPin>,
    dio: Flex<'d, AnyPin>,
    displays: [bool; DISPLAY_COUNT],
    brightnesses: [u8; DISPLAY_COUNT],
}

impl<'d> LedAndKey<'d> {
    pub(crate) fn new(stb_one: AnyPin,
                      stb_two: AnyPin,
                      clk: AnyPin,
                      dio: AnyPin) -> LedAndKey<'d> {

        let mut clk: Output<AnyPin> = Output::new(clk, Level::Low, Speed::Low);
        let mut dio: Flex<AnyPin> = Flex::new(dio);

        dio.set_as_output(Speed::Low); // By default, in data transfer mode.
        dio.set_low();

        let mut stb_one: Output<AnyPin> = Output::new(stb_one, Level::High, Speed::Low);
        let mut stb_two: Output<AnyPin> = Output::new(stb_two, Level::High, Speed::Low);
        let stbs: [Output<'d, AnyPin>; DISPLAY_COUNT] = [stb_one, stb_two,];

        let displays: [bool; DISPLAY_COUNT] = [true, true,];
        let brightnesses: [u8; DISPLAY_COUNT] = [
            DisplayCommand::BRIGHTNESS_DEFAULT as u8,
            DisplayCommand::BRIGHTNESS_DEFAULT as u8,
        ];

        let mut driver = Self { stbs, clk, dio, displays, brightnesses };

        for id in 0..DISPLAY_COUNT {
            driver.push_display_ctrl_instr(id);
        }

        driver.cleanup();

        driver
    }

        // Includes display.
        pub(crate) fn display_on(&mut self, id: usize) -> () {
            self.displays[id] = true;
            self.push_display_ctrl_instr(id);
        }

        // Disable display.
        pub(crate) fn display_off(&mut self, id: usize) -> () {
            self.displays[id] = false;
            self.push_display_ctrl_instr(id);
        }

        // Sets all display registers to zero.
        pub(crate) fn cleanup(&mut self) -> () {
            for id in 0..DISPLAY_COUNT {
                self.push_data_write_instr(id, true);
                self.stbs[id].set_low();
                self.push_address_instr(AddressCommand::ADDRESS_DEFAULT as u8);

                for i in 0..15 {
                    self.write_byte(instructions::NULL);
                }

                self.stbs[id].set_high();
            }
        }

        /*
         Sets the brightness of the LEDs and segments.
         @value: 0..7
         */
        pub(crate) fn set_brightness(&mut self, id: usize, value: u8) -> () {
            self.brightnesses[id] = value;
            self.push_display_ctrl_instr(id);
        }

        /*
         Sets the value of the segment.
         @position: 0..7
         @state: 0..9 and A-Z
         */
        pub(crate) fn set_segment_value(&mut self, id: usize, position: u8, value: DigitSymbol) -> () {
            self.write(id, position << 1, value as u8);
        }

        /*
         Sets the LED's value.
         @position: 0..7
         @state: 0 or 1
         */
        pub(crate) fn set_led_state(&mut self, id: usize, position: u8, state: u8) -> () {
            self.write(id, (position << 1) + 1, state);
        }

        /*
         Determines the key pressed.
         Returns an array of states for each key, from left to right: 1 - pressed, 0 - otherwise.
        */
        pub(crate) fn def_pressed_keys<'a>(&'a mut self, keys_array: &'a mut [u8; 8 * DISPLAY_COUNT])
            -> &mut [u8; 8 * DISPLAY_COUNT] {
            for id in 0..DISPLAY_COUNT {
                let mut data: u32 = self.scan_keys(id);

                for i in 0..4 {
                    keys_array[i + 8 * id] = if (data >> (8 * i) & 1) == 1 { 1 } else { 0 };
                    keys_array[i + 4 + 8 * id] = if (data >> (8 * i + 4) & 1) == 1 { 1 } else { 0 };
                }
            }

            keys_array
        }

        /*
         Write a byte to the display register.
         @position: 0..15
         */
        fn write(&mut self, id: usize, position: u8, data: u8) -> () {
            self.push_data_write_instr(id, false);

            self.stbs[id].set_low();
            self.push_address_instr(position);
            self.write_byte(data);
            self.stbs[id].set_high();
        }

        // Reads the values of each button.
        pub(crate) fn scan_keys(&mut self, id: usize) -> u32 {
            self.stbs[id].set_low();
            self.write_byte(DataCommand::SET_DATA_INSTR as u8 | DataCommand::DATA_READ_INSTR as u8);

            let mut data: u32 = 0;
            for i in 0..4 { data |= (self.read_byte() as u32) << (i * 8); }

            self.stbs[id].set_high();

            data
        }

        /*
         Display configuration instruction.
         Default:
         ~ display on
         ~ brightness max (0x07)
         */
        fn push_display_ctrl_instr(&mut self, id: usize) -> () {
            let display_instr: u8;

            if self.displays[id] {
                display_instr = DisplayCommand::DISPLAY_ON_INSTR as u8;
            } else {
                display_instr = DisplayCommand::DISPLAY_OFF_INSTR as u8;
            }

            self.push_instruction(DisplayCommand::SET_DISPLAY_CTRL_INSTR as u8 |
                display_instr | self.brightnesses[id], id);
        }

        /*
         Sends instructions for subsequent recording.
         Data command:
         if @autoincrement is true, then AUTOMATIC address increment mode, else fixed address mode.
         */
        fn push_data_write_instr(&mut self, id: usize, autoincrement: bool) -> () {
            self.push_instruction(DataCommand::SET_DATA_INSTR as u8 |
                if autoincrement { instructions::NULL } else { DataCommand::FIXED_ADDRESS as u8 } |
                DataCommand::DATA_WRITE_INSTR as u8, id);
        }

        // Sets the address to write the value to.
        fn push_address_instr(&mut self, address: u8) -> () {
            self.write_byte(AddressCommand::SET_ADDRESS_INSTR as u8 | address);
        }

        // Push a instruction to the TM1638.
        fn push_instruction(&mut self, instruction: u8, id: usize) -> () {
            self.stbs[id].set_low();
            self.write_byte(instruction);
            self.stbs[id].set_high();
        }

        // Write 1 byte of information to the TM1638.
        fn write_byte(&mut self, byte: u8) -> () {
            for i in 0..8 {
                self.clk.set_low();

                if (byte >> i) & 1 == 0 { self.dio.set_low(); } else { self.dio.set_high(); }

                self.clk.set_high();
            }
        }

        // Read 1 byte of information from TM1638.
        fn read_byte(&mut self) -> u8 {
            self.dio.set_as_input(Pull::Up);

            let mut byte: u8 = 0;
            for i in 0..8 {
                self.clk.set_low();
                self.clk.set_high();

                if self.dio.is_high() { byte |= 1 << i; }
            }

            self.dio.set_as_output(Speed::Low);

            byte
        }
    }