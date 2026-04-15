// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! ESP32 EMAC register definitions.
//!
//! Three register blocks:
//! - MAC: frame config, addressing, flow control, MDIO
//! - DMA: descriptor lists, bus mode, interrupts
//! - EXT: clock, RMII/MII mode, GPIO, power

pub mod dma;
pub mod ext;
pub mod mac;
