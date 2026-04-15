# esp-emac

ESP32 EMAC bare-metal Ethernet MAC driver for `#![no_std]` Rust.

## Features

- DMA ring management with static allocation (const generics)
- RMII interface with configurable clock (APLL internal or external)
- MDIO/SMI controller for PHY register access
- Optional `eth-mdio-phy` MdioBus trait implementation (`mdio-phy` feature)
- Optional `embassy-net` driver integration (`embassy-net` feature)

## License

Licensed under either of:
- GNU General Public License, Version 2.0 or later ([LICENSE-GPL](LICENSE-GPL))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
