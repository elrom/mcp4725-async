extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use embedded_hal_async::i2c::{ErrorKind, ErrorType, Operation};
use crate::*;

const ADDR: u8 = 0b1100001;
const DAC: u16 = 0b1010_1010_1010;

enum I2cTransaction {
    Write(SevenBitAddress, Vec<u8>),
    Read(SevenBitAddress, Vec<u8>),
}

struct I2cMock<'a> {
    transactions: &'a [I2cTransaction],
    index: usize,
}
impl <'a> I2cMock<'a> {
    pub fn new(transactions: &'a [I2cTransaction]) -> I2cMock<'a> {
        Self {
            transactions,
            index: 0,
        }
    }
    
    pub fn assert_done(&self) {
        assert_eq!(self.index, self.transactions.len());
    }
}
impl <'a> ErrorType for I2cMock<'a> {
    type Error = ErrorKind;
}
impl <'a> I2c for I2cMock<'a> {
    async fn transaction(&mut self, address: SevenBitAddress, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        for op in operations {
            match (op, &self.transactions[self.index]) {
                (Operation::Write(buf), I2cTransaction::Write(addr, data)) => {
                    assert_eq!(address, *addr);
                    assert_eq!(buf, data);
                }
                (Operation::Read(buf), I2cTransaction::Read(addr, data)) => {
                    assert_eq!(address, *addr);
                    assert_eq!(buf.len(), data.len());
                    buf.copy_from_slice(data);
                }
                _ => panic!(),
            }
            self.index += 1;
        }
        Ok(())
    }
}

#[test]
fn test_fast_write() {
    futures_lite::future::block_on(async {
        let expected = [
            I2cTransaction::Write(ADDR, vec![
                0b0000_1010, // Fast mode (0, 0), pd normal (0, 0)
                0b1010_1010,
                0b0000_1010,
                0b1010_1010,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0001_1010, // Fast mode (0, 0), pd 1k (0, 1)
                0b1010_1010,
                0b0001_1010,
                0b1010_1010,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0010_1010, // Fast mode (0, 0), pd 100k (1, 0)
                0b1010_1010,
                0b0010_1010,
                0b1010_1010,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0011_1010, // Fast mode (0, 0), pd 500k (1, 1)
                0b1010_1010,
                0b0011_1010,
                0b1010_1010,
            ]),
        ];
        let i2c = I2cMock::new(&expected);
        
        let mut mcp = MCP4725::new(i2c, ADDR);
        mcp.fast_write(PowerDownMode::Normal, DAC).await.unwrap();
        mcp.fast_write(PowerDownMode::OneK, DAC).await.unwrap();
        mcp.fast_write(PowerDownMode::OneHundredK, DAC).await.unwrap();
        mcp.fast_write(PowerDownMode::FiveHundredK, DAC).await.unwrap();
        
        mcp.destroy().assert_done();
    })
}

#[test]
fn test_slow_write_reg() {
    futures_lite::future::block_on(async {
        let expected = [
            I2cTransaction::Write(ADDR, vec![
                0b0100_0000, // Write reg (0, 1, 0), pd normal (0, 0)
                0b1010_1010,
                0b1010_0000,
                0b0100_0000,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0100_0010, // Write reg (0, 1, 0), pd 1k (0, 1)
                0b1010_1010,
                0b1010_0000,
                0b0100_0010,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0100_0100, // Write reg (0, 1, 0), pd 100k (1, 0)
                0b1010_1010,
                0b1010_0000,
                0b0100_0100,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0100_0110, // Write reg (0, 1, 0), pd 500k (1, 1)
                0b1010_1010,
                0b1010_0000,
                0b0100_0110,
                0b1010_1010,
                0b1010_0000,
            ]),
        ];
        let i2c = I2cMock::new(&expected);

        let mut mcp = MCP4725::new(i2c, ADDR);
        mcp.write(PowerDownMode::Normal, DAC, false).await.unwrap();
        mcp.write(PowerDownMode::OneK, DAC, false).await.unwrap();
        mcp.write(PowerDownMode::OneHundredK, DAC, false).await.unwrap();
        mcp.write(PowerDownMode::FiveHundredK, DAC, false).await.unwrap();

        mcp.destroy().assert_done();
    })
}

#[test]
fn test_slow_write_eeprom() {
    futures_lite::future::block_on(async {
        let expected = [
            I2cTransaction::Write(ADDR, vec![
                0b0110_0000, // Write eeprom (0, 1, 1), pd normal (0, 0)
                0b1010_1010,
                0b1010_0000,
                0b0110_0000,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0110_0010, // Write eeprom (0, 1, 1), pd 1k (0, 1)
                0b1010_1010,
                0b1010_0000,
                0b0110_0010,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0110_0100, // Write eeprom (0, 1, 1), pd 100k (1, 0)
                0b1010_1010,
                0b1010_0000,
                0b0110_0100,
                0b1010_1010,
                0b1010_0000,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0110_0110, // Write eeprom (0, 1, 1), pd 500k (1, 1)
                0b1010_1010,
                0b1010_0000,
                0b0110_0110,
                0b1010_1010,
                0b1010_0000,
            ]),
        ];
        let i2c = I2cMock::new(&expected);

        let mut mcp = MCP4725::new(i2c, ADDR);
        mcp.write(PowerDownMode::Normal, DAC, true).await.unwrap();
        mcp.write(PowerDownMode::OneK, DAC, true).await.unwrap();
        mcp.write(PowerDownMode::OneHundredK, DAC, true).await.unwrap();
        mcp.write(PowerDownMode::FiveHundredK, DAC, true).await.unwrap();

        mcp.destroy().assert_done();
    })
}

#[test]
fn test_set_voltage() {
    futures_lite::future::block_on(async {
        let expected = [
            I2cTransaction::Write(ADDR, vec![
                0b0000_1010, // Fast mode (0, 0), pd normal (0, 0)
                0b1010_1010,
                0b0000_1010,
                0b1010_1010,
            ]),
            I2cTransaction::Write(ADDR, vec![
                0b0110_0000, // Write eeprom (0, 1, 1), pd normal (0, 0)
                0b1010_1010,
                0b1010_0000,
                0b0110_0000,
                0b1010_1010,
                0b1010_0000,
            ]),
        ];
        let i2c = I2cMock::new(&expected);

        let mut mcp = MCP4725::new(i2c, ADDR);
        mcp.set_voltage(DAC, false).await.unwrap();
        mcp.set_voltage(DAC, true).await.unwrap();

        mcp.destroy().assert_done();
    })
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_read() {
    futures_lite::future::block_on(async {
        let expected = [
            I2cTransaction::Read(ADDR, vec![
                0b0000_0000, // RDY=0, POR=0, PD=00 (Normal)
                0b1010_1010,
                0b1010_0000,
                0b0000_1010, // PD=00 (Normal)
                0b1010_1010,
            ]),
            I2cTransaction::Read(ADDR, vec![
                0b1100_0110, // RDY=1, POR=1, PD=11 (500k)
                0b1010_1010,
                0b1010_0000,
                0b0110_1010, // PD=11 (500k)
                0b1010_1010,
            ]),
            I2cTransaction::Read(ADDR, vec![
                0b0000_0010, // RDY=0, POR=0, PD=01 (1k)
                0b1010_1010,
                0b1010_0000,
                0b0010_1010, // PD=11 (500k)
                0b1010_1010,
            ]),
            I2cTransaction::Read(ADDR, vec![
                0b0000_0100, // RDY=0, POR=0, PD=10 (100k)
                0b1010_1010,
                0b1010_0000,
                0b0100_1010, // PD=11 (500k)
                0b1010_1010,
            ])
        ];
        let i2c = I2cMock::new(&expected);
        
        let mut mcp = MCP4725::new(i2c, ADDR);
        let (reg, eeprom) = mcp.read().await.unwrap();
        assert_eq!(reg.eeprom_ready(), false);
        assert_eq!(reg.por(), false);
        assert_eq!(reg.power_down_mode(), PowerDownMode::Normal);
        assert_eq!(reg.dac(), DAC);
        assert_eq!(eeprom.power_down_mode(), PowerDownMode::Normal);
        assert_eq!(eeprom.dac(), DAC);
        
        let (reg, eeprom) = mcp.read().await.unwrap();
        assert_eq!(reg.eeprom_ready(), true);
        assert_eq!(reg.por(), true);
        assert_eq!(reg.power_down_mode(), PowerDownMode::FiveHundredK);
        assert_eq!(reg.dac(), DAC);
        assert_eq!(eeprom.power_down_mode(), PowerDownMode::FiveHundredK);
        assert_eq!(eeprom.dac(), DAC);
        
        let (reg, eeprom) = mcp.read().await.unwrap();
        assert_eq!(reg.eeprom_ready(), false);
        assert_eq!(reg.por(), false);
        assert_eq!(reg.power_down_mode(), PowerDownMode::OneK);
        assert_eq!(reg.dac(), DAC);
        assert_eq!(eeprom.power_down_mode(), PowerDownMode::OneK);
        assert_eq!(eeprom.dac(), DAC);

        let (reg, eeprom) = mcp.read().await.unwrap();
        assert_eq!(reg.eeprom_ready(), false);
        assert_eq!(reg.por(), false);
        assert_eq!(reg.power_down_mode(), PowerDownMode::OneHundredK);
        assert_eq!(reg.dac(), DAC);
        assert_eq!(eeprom.power_down_mode(), PowerDownMode::OneHundredK);
        assert_eq!(eeprom.dac(), DAC);
        
        mcp.destroy().assert_done();
    })
}
