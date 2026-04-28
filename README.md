# esp-emac

ESP32 EMAC bare-metal Ethernet MAC driver for `#![no_std]` Rust.

## Status

This crate is in **Phase 1** of a migration that aims to replace
`ph-esp32-mac` end-to-end with home-grown crates. Today it is a thin
facade over `ph-esp32-mac`:

- `Emac::init` configures APLL 50 MHz and the RMII clock GPIO with our
  own code (these aren't covered by ph-esp32-mac), then delegates the
  rest of MAC/DMA bring-up to `ph_esp32_mac::Emac::init`.
- The optional `mdio-phy` and `embassy-net` modules
  (`mdio::EspMdio`, `embassy::EmacDriver`) ship our own
  implementations, but they currently exhibit a cold-boot regression
  on the JXD-PM380-E1ETH stand (DHCP works, unicast RX wedged). Until
  phases 3.x replace ph-esp32-mac's PHY/MDIO/embassy code piece by
  piece, the firmware reaches into the inner driver through the
  `#[doc(hidden)] Emac::inner_mut` escape hatch and uses
  ph-esp32-mac's runtime path.

The migration plan lives at
`docs/plans/esp-emac-migration.md` in the firmware repository.

## Features

- DMA ring management with static allocation (const generics)
- RMII interface with configurable clock (APLL internal or external)
- MDIO/SMI controller for PHY register access (Phase 1: under
  diagnosis, not used by firmware yet)
- Optional `eth-mdio-phy` MdioBus trait implementation (`mdio-phy`
  feature)
- Optional `embassy-net` driver integration (`embassy-net` feature)

## License

Licensed under either of:
- GNU General Public License, Version 2.0 or later ([LICENSE-GPL](LICENSE-GPL))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
