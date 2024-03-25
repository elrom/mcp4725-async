#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
/// Power down modes
pub enum PowerDownMode {
    /// Normal mode
    Normal = 0b00,
    /// 1k ohm resistor to ground
    OneK = 0b01,
    /// 100k ohm resistor to ground
    OneHundredK = 0b10,
    /// 500k ohm resistor to ground
    FiveHundredK = 0b11,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// The DAC register status
pub struct RegisterStatus {
    /// Data registers
    data: u8,
    /// DAC value
    dac: u16,
}

impl RegisterStatus {
    pub(crate) fn new(data: [u8; 3]) -> Self {
        let dac = u16::from_be_bytes([data[1], data[2]]);
        Self {
            data: data[0],
            dac: dac >> 4,
        }
    }
    
    /// Get the raw read data
    #[inline(always)]
    pub fn read_data(&self) -> u8 {
        self.data
    }

    /// Is the EEPROM ready? Returns:
    /// * `true` if the previous EEPROM write has completed and it is ready for a new write
    /// * `false` if the previous EEPROM write has not completed and it is not ready for a new write
    #[inline(always)]
    pub fn eeprom_ready(&self) -> bool {
        self.data & 0b1000_0000 != 0
    }

    /// Get the power on reset state
    #[inline(always)]
    pub fn por(&self) -> bool {
        self.data & 0b0100_0000 != 0
    }

    /// Get the power down mode
    #[inline(always)]
    pub fn power_down_mode(&self) -> PowerDownMode {
        match (self.data & 0b0000_0110) >> 1 {
            0b00 => PowerDownMode::Normal,
            0b01 => PowerDownMode::OneK,
            0b10 => PowerDownMode::OneHundredK,
            0b11 => PowerDownMode::FiveHundredK,
            _ => unreachable!(), // Should never happen, since we've taken care of all 4 bit patterns
        }
    }

    /// Get the DAC value
    #[inline(always)]
    pub fn dac(&self) -> u16 {
        self.dac
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// The EEPROM status
pub struct EEPROMStatus {
    power_down_mode: PowerDownMode,
    dac: u16,
}

impl EEPROMStatus {
    pub(crate) fn new(data: [u8; 2]) -> Self {
        let power_down_mode = match (data[0] & 0b0110_0000) >> 5 {
            0b00 => PowerDownMode::Normal,
            0b01 => PowerDownMode::OneK,
            0b10 => PowerDownMode::OneHundredK,
            0b11 => PowerDownMode::FiveHundredK,
            _ => unreachable!(), // Should never happen, since we've taken care of all 4 bit patterns
        };
        let dac = u16::from_be_bytes(data) & 0xFFF;
        
        Self { power_down_mode, dac }
    }

    /// Get the power down mode
    #[inline(always)]
    pub fn power_down_mode(&self) -> PowerDownMode {
        self.power_down_mode
    }
    
    /// Get the DAC value
    #[inline(always)]
    pub fn dac(&self) -> u16 {
        self.dac
    }
}
