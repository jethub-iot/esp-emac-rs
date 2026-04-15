// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! MDIO (Management Data Input/Output) controller.
//!
//! Communicates with Ethernet PHY chips via the ESP32 EMAC's
//! built-in SMI (Station Management Interface).

use crate::error::EmacError;

/// MDC clock divider based on system clock frequency.
///
/// The MDC clock must not exceed 2.5 MHz per IEEE 802.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdcClockDivider {
    /// Clock/16 (20-35 MHz system clock)
    Div16 = 2,
    /// Clock/26 (35-60 MHz system clock)
    Div26 = 3,
    /// Clock/42 (60-100 MHz system clock) — ESP32 @ 80 MHz
    Div42 = 0,
    /// Clock/62 (100-150 MHz system clock)
    Div62 = 1,
    /// Clock/102 (150-250 MHz system clock)
    Div102 = 4,
    /// Clock/124 (250-300 MHz system clock)
    Div124 = 5,
}

impl MdcClockDivider {
    /// Select the appropriate divider for a given system clock frequency.
    pub const fn from_sys_clock_hz(hz: u32) -> Self {
        if hz < 35_000_000 {
            Self::Div16
        } else if hz < 60_000_000 {
            Self::Div26
        } else if hz < 100_000_000 {
            Self::Div42
        } else if hz < 150_000_000 {
            Self::Div62
        } else if hz < 250_000_000 {
            Self::Div102
        } else {
            Self::Div124
        }
    }

    /// Get the register value for this divider.
    pub const fn reg_value(self) -> u32 {
        self as u32
    }
}

impl Default for MdcClockDivider {
    /// Default: Div42 (correct for ESP32 @ 80 MHz).
    fn default() -> Self {
        Self::Div42
    }
}

/// Maximum valid PHY address (5-bit field).
pub const MAX_PHY_ADDR: u8 = 31;

/// Maximum valid register address (5-bit field).
pub const MAX_REG_ADDR: u8 = 31;

/// MDIO timeout in polling iterations.
const MDIO_TIMEOUT_ITERS: u32 = 1000;

// ── EMAC MAC register offsets for MDIO ─────────────────────────────────────

/// ESP32 MAC register base address.
const MAC_BASE: usize = 0x3FF6_A000;

/// GMACMIIADDR register offset from MAC_BASE.
const GMACMIIADDR_OFFSET: usize = 0x10;
/// GMACMIIDATA register offset from MAC_BASE.
const GMACMIIDATA_OFFSET: usize = 0x14;

// GMACMIIADDR bit fields
const GMACMIIADDR_PA_SHIFT: u32 = 11;
const GMACMIIADDR_PA_MASK: u32 = 0x1F << 11;
const GMACMIIADDR_GR_SHIFT: u32 = 6;
const GMACMIIADDR_GR_MASK: u32 = 0x1F << 6;
const GMACMIIADDR_CR_SHIFT: u32 = 2;
const GMACMIIADDR_CR_MASK: u32 = 0x0F << 2;
const GMACMIIADDR_GW: u32 = 1 << 1;
const GMACMIIADDR_GB: u32 = 1 << 0;

/// ESP32 MDIO controller.
///
/// Provides read/write access to PHY registers via the EMAC's
/// built-in SMI (Station Management Interface).
///
/// # Safety
///
/// This struct accesses memory-mapped EMAC registers directly.
/// It must only be used on ESP32 with the EMAC peripheral clock enabled.
pub struct EspMdio {
    clock_divider: MdcClockDivider,
}

impl EspMdio {
    /// Create a new MDIO controller with default clock divider (Div42 for 80 MHz).
    pub fn new() -> Self {
        Self {
            clock_divider: MdcClockDivider::default(),
        }
    }

    /// Create with a specific clock divider.
    pub fn with_clock_divider(divider: MdcClockDivider) -> Self {
        Self {
            clock_divider: divider,
        }
    }

    /// Read a PHY register via MDIO/SMI.
    pub fn read(&mut self, phy_addr: u8, reg_addr: u8) -> Result<u16, EmacError> {
        if phy_addr > MAX_PHY_ADDR {
            return Err(EmacError::InvalidPhyAddress);
        }
        if reg_addr > MAX_REG_ADDR {
            return Err(EmacError::InvalidConfig);
        }

        self.wait_not_busy()?;

        let addr = self.build_mii_addr(phy_addr, reg_addr, false);
        self.write_mii_addr(addr);

        self.wait_not_busy()?;

        Ok((self.read_mii_data() & 0xFFFF) as u16)
    }

    /// Write a PHY register via MDIO/SMI.
    pub fn write(&mut self, phy_addr: u8, reg_addr: u8, value: u16) -> Result<(), EmacError> {
        if phy_addr > MAX_PHY_ADDR {
            return Err(EmacError::InvalidPhyAddress);
        }
        if reg_addr > MAX_REG_ADDR {
            return Err(EmacError::InvalidConfig);
        }

        self.wait_not_busy()?;

        self.write_mii_data(value as u32);

        let addr = self.build_mii_addr(phy_addr, reg_addr, true);
        self.write_mii_addr(addr);

        self.wait_not_busy()
    }

    /// Build GMACMIIADDR register value.
    ///
    /// Public within crate for testing.
    pub(crate) fn build_mii_addr(&self, phy_addr: u8, reg_addr: u8, is_write: bool) -> u32 {
        let mut addr = 0u32;
        addr |= ((phy_addr as u32) << GMACMIIADDR_PA_SHIFT) & GMACMIIADDR_PA_MASK;
        addr |= ((reg_addr as u32) << GMACMIIADDR_GR_SHIFT) & GMACMIIADDR_GR_MASK;
        addr |= (self.clock_divider.reg_value() << GMACMIIADDR_CR_SHIFT) & GMACMIIADDR_CR_MASK;
        if is_write {
            addr |= GMACMIIADDR_GW;
        }
        addr |= GMACMIIADDR_GB;
        addr
    }

    /// Wait for MDIO operation to complete (busy bit cleared).
    fn wait_not_busy(&self) -> Result<(), EmacError> {
        for _ in 0..MDIO_TIMEOUT_ITERS {
            if self.read_mii_addr() & GMACMIIADDR_GB == 0 {
                return Ok(());
            }
        }
        Err(EmacError::Timeout)
    }

    // ── Hardware register access (ESP32 only) ────────────────────────────

    #[inline(always)]
    fn read_mii_addr(&self) -> u32 {
        unsafe { core::ptr::read_volatile((MAC_BASE + GMACMIIADDR_OFFSET) as *const u32) }
    }

    #[inline(always)]
    fn write_mii_addr(&self, val: u32) {
        unsafe { core::ptr::write_volatile((MAC_BASE + GMACMIIADDR_OFFSET) as *mut u32, val) }
    }

    #[inline(always)]
    fn read_mii_data(&self) -> u32 {
        unsafe { core::ptr::read_volatile((MAC_BASE + GMACMIIDATA_OFFSET) as *const u32) }
    }

    #[inline(always)]
    fn write_mii_data(&self, val: u32) {
        unsafe { core::ptr::write_volatile((MAC_BASE + GMACMIIDATA_OFFSET) as *mut u32, val) }
    }
}

impl Default for EspMdio {
    fn default() -> Self {
        Self::new()
    }
}

/// MdioBus trait implementation — only with "mdio-phy" feature.
/// Delegates to the standalone read()/write() methods.
#[cfg(feature = "mdio-phy")]
impl eth_mdio_phy::MdioBus for EspMdio {
    type Error = EmacError;

    fn read(&mut self, phy_addr: u8, reg_addr: u8) -> Result<u16, EmacError> {
        EspMdio::read(self, phy_addr, reg_addr)
    }

    fn write(&mut self, phy_addr: u8, reg_addr: u8, value: u16) -> Result<(), EmacError> {
        EspMdio::write(self, phy_addr, reg_addr, value)
    }
}

// ── Tests (pure-logic only, no hardware register access) ─────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── MdcClockDivider ──────────────────────────────────────────────────

    #[test]
    fn mdc_divider_from_sys_clock() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(20_000_000),
            MdcClockDivider::Div16
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(40_000_000),
            MdcClockDivider::Div26
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(80_000_000),
            MdcClockDivider::Div42
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(120_000_000),
            MdcClockDivider::Div62
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(160_000_000),
            MdcClockDivider::Div102
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(280_000_000),
            MdcClockDivider::Div124
        );
    }

    #[test]
    fn mdc_divider_default_is_div42() {
        assert_eq!(MdcClockDivider::default(), MdcClockDivider::Div42);
    }

    #[test]
    fn mdc_divider_reg_values() {
        assert_eq!(MdcClockDivider::Div42.reg_value(), 0);
        assert_eq!(MdcClockDivider::Div62.reg_value(), 1);
        assert_eq!(MdcClockDivider::Div16.reg_value(), 2);
        assert_eq!(MdcClockDivider::Div26.reg_value(), 3);
        assert_eq!(MdcClockDivider::Div102.reg_value(), 4);
        assert_eq!(MdcClockDivider::Div124.reg_value(), 5);
    }

    #[test]
    fn mdc_divider_boundary_35mhz() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(34_999_999),
            MdcClockDivider::Div16
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(35_000_000),
            MdcClockDivider::Div26
        );
    }

    #[test]
    fn mdc_divider_boundary_60mhz() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(59_999_999),
            MdcClockDivider::Div26
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(60_000_000),
            MdcClockDivider::Div42
        );
    }

    #[test]
    fn mdc_divider_boundary_100mhz() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(99_999_999),
            MdcClockDivider::Div42
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(100_000_000),
            MdcClockDivider::Div62
        );
    }

    #[test]
    fn mdc_divider_boundary_150mhz() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(149_999_999),
            MdcClockDivider::Div62
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(150_000_000),
            MdcClockDivider::Div102
        );
    }

    #[test]
    fn mdc_divider_boundary_250mhz() {
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(249_999_999),
            MdcClockDivider::Div102
        );
        assert_eq!(
            MdcClockDivider::from_sys_clock_hz(250_000_000),
            MdcClockDivider::Div124
        );
    }

    // ── build_mii_addr ───────────────────────────────────────────────────

    #[test]
    fn build_mii_addr_read() {
        let mdio = EspMdio::new(); // Div42 = 0
        let addr = mdio.build_mii_addr(1, 0, false);
        // PHY addr 1 → bits[15:11] = 1<<11 = 0x0800
        // Reg addr 0 → bits[10:6] = 0
        // CR = 0 (Div42) → bits[5:2] = 0
        // GW = 0 (read)
        // GB = 1 (trigger)
        assert_eq!(addr, 0x0800 | GMACMIIADDR_GB);
    }

    #[test]
    fn build_mii_addr_write() {
        let mdio = EspMdio::new();
        let addr = mdio.build_mii_addr(1, 0, true);
        // Same as read + GW bit
        assert_eq!(addr, 0x0800 | GMACMIIADDR_GW | GMACMIIADDR_GB);
    }

    #[test]
    fn build_mii_addr_phy_and_reg() {
        let mdio = EspMdio::new();
        let addr = mdio.build_mii_addr(31, 31, false);
        // PHY addr 31 → 31<<11 = 0xF800
        // Reg addr 31 → 31<<6 = 0x07C0
        assert_eq!(addr & GMACMIIADDR_PA_MASK, 31 << 11);
        assert_eq!(addr & GMACMIIADDR_GR_MASK, 31 << 6);
    }

    #[test]
    fn build_mii_addr_clock_divider() {
        let mdio = EspMdio::with_clock_divider(MdcClockDivider::Div102);
        let addr = mdio.build_mii_addr(0, 0, false);
        // Div102 reg_value = 4 → bits[5:2] = 4<<2 = 0x10
        let cr_field = (addr & GMACMIIADDR_CR_MASK) >> GMACMIIADDR_CR_SHIFT;
        assert_eq!(cr_field, 4);
    }

    // ── Validation ───────────────────────────────────────────────────────
    // NOTE: read()/write() can't be tested on host (hardware registers).
    // We test validation logic that runs before hardware access.

    #[test]
    fn max_phy_addr_constant() {
        assert_eq!(MAX_PHY_ADDR, 31);
    }

    #[test]
    fn max_reg_addr_constant() {
        assert_eq!(MAX_REG_ADDR, 31);
    }
}
