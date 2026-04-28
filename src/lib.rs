// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! ESP32 EMAC Ethernet MAC driver.
//!
//! # Phase 1 status
//!
//! [`Emac::init`](crate::emac::Emac::init) configures APLL 50 MHz and
//! the RMII clock GPIO via our own [`clock`] module, and then delegates
//! the rest of MAC/DMA bring-up (SMI/RMII pins, DPORT clock, software
//! reset, MAC/DMA defaults, descriptor chains, MAC address filter) to
//! [`ph_esp32_mac::Emac::init`]. This is the only Phase 1 piece that is
//! known to be cold-boot stable on the JXD-PM380-E1ETH stand.
//!
//! The runtime data path — PHY driver, MDIO bus, and embassy-net glue
//! — is still provided by ph-esp32-mac in the firmware: our own
//! [`EspMdio`] / [`embassy::EmacDriver`] and the sibling
//! `eth-phy-lan87xx` crate currently wedge unicast RX after a power
//! cycle. Their replacement is tracked by phases 3.x in the migration
//! plan. Until then, firmware reaches into ph-esp32-mac through the
//! escape hatch [`Emac::inner_mut`](crate::emac::Emac::inner_mut).
//!
//! See `docs/plans/esp-emac-migration.md` in the firmware repository
//! for the staged rewrite roadmap.

#![no_std]

pub mod clock;
pub mod config;
pub mod emac;
#[cfg(feature = "embassy-net")]
pub mod embassy;
pub mod error;
pub mod mdio;

// Legacy modules kept for reference during the phased rewrite. They are
// not wired into the current implementation (the facade delegates to
// ph-esp32-mac) and will be removed once the per-module rewrite lands.
#[doc(hidden)]
#[allow(clippy::assertions_on_constants)]
pub mod dma;
#[doc(hidden)]
#[allow(clippy::assertions_on_constants)]
pub mod regs;

pub use config::{ClkGpio, EmacConfig, RmiiClockConfig, RmiiPins};
pub use emac::{Duplex, Emac, EmacDefault, EmacSmall, EmacState, Speed};
pub use error::EmacError;
pub use mdio::{EspMdio, MdcClockDivider};
