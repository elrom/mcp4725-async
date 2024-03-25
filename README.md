# mcp4725-async
An async driver for the MCP4725 DAC using `embedded_hal_async`. It supports sending commands over I2C.

The driver can be initialized by calling `new` with an I2C interface and the chip address:
```rust
// Address corresponds to A2,A1=0, and A0 tied to Vss
let mut mcp = MCP4725::new(i2c, 0b1100000);
```

To quickly set the DAC output:
```rust
// Set DAC to 0xFFF = Full scale, don't write to eeprom
mcp.set_voltage(0xFFF, false);
// Set DAC to 0x800 = Half scale, don't write to eeprom
mcp.set_voltage(0x800, false);
// Set DAC to 0x000 = Zero volts, write to eeprom
mcp.set_voltage(0x000, true);
```

#### License
Licensed under either of
* Apache License, Version 2.0: [LICENSE-APACHE.txt](LICENSE-APACHE.txt)
* MIT License: [LICENSE-MIT.txt](LICENSE-MIT.txt)

at your option.
