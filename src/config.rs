// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! EMAC configuration types.

/// EMAC configuration.
#[derive(Debug, Clone)]
pub struct EmacConfig {
    /// RMII clock source.
    pub clock: RmiiClockConfig,
    /// RMII management pins (MDC/MDIO).
    pub pins: RmiiPins,
}

/// RMII reference clock configuration.
///
/// The 50 MHz RMII clock can be generated internally (ESP32 APLL) or
/// supplied externally from a PHY-driven oscillator. Mode selection is
/// hardware-specific and `Emac::init` rejects mismatched GPIO choices
/// with [`crate::EmacError::InvalidConfig`] — see each variant's docs
/// for which pads are valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiClockConfig {
    /// ESP32 APLL generates 50 MHz and drives it out of the chip.
    ///
    /// Valid GPIO choices: [`ClkGpio::Gpio16`] (`EMAC_CLK_OUT`, 0°) or
    /// [`ClkGpio::Gpio17`] (`EMAC_CLK_OUT_180`, 180° — the LAN8720A
    /// reference design preference). [`ClkGpio::Gpio0`] is **invalid**
    /// for this mode: GPIO0 function 5 is `EMAC_TX_CLK`, an input pad.
    ///
    /// Coexistence note: ESP32 errata CLK-3.22 — the APLL clock signal
    /// emitted on the GPIO pad is corrupted by on-chip RF noise during
    /// WiFi/BT transmission. This mode is unsafe with active radio;
    /// boards needing Ethernet + WiFi should use [`Self::External`].
    InternalApll {
        /// GPIO for clock output. Must be `Gpio16` or `Gpio17`.
        gpio: ClkGpio,
    },
    /// External 50 MHz clock fed in from a PHY crystal or oscillator.
    ///
    /// Valid GPIO choice: [`ClkGpio::Gpio0`] only — that is the only
    /// pad whose function 5 (`EMAC_TX_CLK`) is an input. `Gpio16` /
    /// `Gpio17` are **invalid** here.
    ///
    /// Required for Ethernet + WiFi coexistence (immune to the
    /// CLK-3.22 errata since the clock never leaves the PHY domain).
    External {
        /// GPIO for clock input. Must be `Gpio0`.
        gpio: ClkGpio,
    },
}

/// GPIO pins that can carry the EMAC RMII reference clock on ESP32.
///
/// Direction is fixed by the IO_MUX function 5 wiring:
///
/// - [`Self::Gpio0`] — `EMAC_TX_CLK`, input only — pair with
///   [`RmiiClockConfig::External`].
/// - [`Self::Gpio16`] — `EMAC_CLK_OUT` (0°), output only — pair with
///   [`RmiiClockConfig::InternalApll`].
/// - [`Self::Gpio17`] — `EMAC_CLK_OUT_180` (180°), output only — pair
///   with [`RmiiClockConfig::InternalApll`]. Most common on LAN8720A.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpio {
    /// GPIO0 — `EMAC_TX_CLK` input. Also a boot-strapping pin, take
    /// care that the external oscillator does not violate boot timing.
    Gpio0,
    /// GPIO16 — `EMAC_CLK_OUT` (0° phase).
    Gpio16,
    /// GPIO17 — `EMAC_CLK_OUT_180` (180° phase, most common for LAN8720A).
    Gpio17,
}

impl ClkGpio {
    /// Get the GPIO number.
    pub const fn gpio_num(self) -> u8 {
        match self {
            ClkGpio::Gpio0 => 0,
            ClkGpio::Gpio16 => 16,
            ClkGpio::Gpio17 => 17,
        }
    }
}

/// RMII management pin configuration.
///
/// Data pins (TXD0/1, RXD0/1, TX_EN, CRS_DV) are fixed on ESP32 via IO_MUX
/// and cannot be remapped. Only MDC/MDIO are routed via GPIO Matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RmiiPins {
    /// MDC (Management Data Clock) — routed via GPIO Matrix, any GPIO.
    pub mdc: u8,
    /// MDIO (Management Data I/O) — routed via GPIO Matrix, any GPIO.
    pub mdio: u8,
}

impl Default for RmiiPins {
    /// Default: MDC=GPIO23, MDIO=GPIO18 (most common ESP32 Ethernet boards).
    fn default() -> Self {
        Self { mdc: 23, mdio: 18 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clk_gpio_num() {
        assert_eq!(ClkGpio::Gpio0.gpio_num(), 0);
        assert_eq!(ClkGpio::Gpio16.gpio_num(), 16);
        assert_eq!(ClkGpio::Gpio17.gpio_num(), 17);
    }

    #[test]
    fn rmii_pins_default() {
        let pins = RmiiPins::default();
        assert_eq!(pins.mdc, 23);
        assert_eq!(pins.mdio, 18);
    }

    #[test]
    fn rmii_clock_config_equality() {
        let a = RmiiClockConfig::InternalApll {
            gpio: ClkGpio::Gpio17,
        };
        let b = RmiiClockConfig::InternalApll {
            gpio: ClkGpio::Gpio17,
        };
        let c = RmiiClockConfig::External {
            gpio: ClkGpio::Gpio0,
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn emac_config_clone() {
        let config = EmacConfig {
            clock: RmiiClockConfig::InternalApll {
                gpio: ClkGpio::Gpio17,
            },
            pins: RmiiPins::default(),
        };
        let cloned = config.clone();
        assert_eq!(cloned.pins.mdc, 23);
    }
}
