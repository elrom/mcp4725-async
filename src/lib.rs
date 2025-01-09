#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
mod tests;
mod types;

pub use types::*;

use embedded_hal_async::i2c::{I2c, SevenBitAddress};

/// The MCP4725 device
pub struct MCP4725<I2C> {
    i2c: I2C,
    address: SevenBitAddress,
}

impl<I2C> MCP4725<I2C>
where
    I2C: I2c,
{
    /// Create a new device from an I2C peripheral and address.
    pub fn new(i2c: I2C, address: SevenBitAddress) -> Self {
        Self { i2c, address }
    }

    /// Destroy this device and get the I2C instance back.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Convenience function to just set the output voltage. Only the lower 12 bits of the DAC value
    /// are used. If `write_eeprom` is true, the DAC value will also be written to the EEPROM and
    /// the EEPROM power down mode will be set to `Normal`.
    pub async fn set_voltage(&mut self, dac: u16, write_eeprom: bool) -> Result<(), I2C::Error> {
        if write_eeprom {
            self.write(PowerDownMode::Normal, dac, write_eeprom).await
        } else {
            self.fast_write(PowerDownMode::Normal, dac).await
        }
    }

    /// Perform a read command. This will return the value in the DAC register and the EEPROM data.
    pub async fn read(&mut self) -> Result<(RegisterStatus, EEPROMStatus), I2C::Error> {
        let mut packet = [0u8; 5];
        self.i2c.read(self.address, &mut packet).await?;

        Ok((
            RegisterStatus::new([packet[0], packet[1], packet[2]]),
            EEPROMStatus::new([packet[3], packet[4]]),
        ))
    }

    /// Perform a fast write. This can set the power down mode and the DAC value. This only changes
    /// the DAC register and does not affect the EEPROM. Only the lower 12 bits of the DAC value
    /// are used.
    pub async fn fast_write(
        &mut self,
        power_down_mode: PowerDownMode,
        dac: u16,
    ) -> Result<(), I2C::Error> {
        let data = (dac & 0xFFF) | ((power_down_mode as u16) << 12);
        let data = data.to_be_bytes();
        self.i2c.write(self.address, &data).await
    }

    /// Perform a normal write. This can set the power down mode and the DAC value, and optionally
    /// write to the EEPROM. Only the lower 12 bits of the DAC value are used.
    pub async fn write(
        &mut self,
        power_down_mode: PowerDownMode,
        dac: u16,
        write_eeprom: bool,
    ) -> Result<(), I2C::Error> {
        let data =
            ((if write_eeprom { 0b011 } else { 0b010 }) << 5) | ((power_down_mode as u8) << 1);
        let dac = dac << 4;

        let dac = dac.to_be_bytes();
        let packet = [data, dac[0], dac[1]];

        self.i2c.write(self.address, &packet).await
    }
}
