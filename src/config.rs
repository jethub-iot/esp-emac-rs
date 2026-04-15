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
/// The 50 MHz RMII clock can be generated internally (APLL) or supplied
/// externally from the PHY's crystal oscillator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiClockConfig {
    /// ESP32 APLL generates 50 MHz, output on GPIO.
    ///
    /// Cannot coexist with WiFi/BT: ESP32 errata CLK-3.22 causes
    /// REF_CLK instability on the GPIO pin during RF transmission.
    /// APLL itself is a separate PLL from BBPLL (used by WiFi),
    /// but the clock OUTPUT signal is corrupted by on-chip RF noise.
    InternalApll {
        /// GPIO for clock output (only 0, 16, or 17).
        gpio: ClkGpio,
    },
    /// External 50 MHz clock from PHY crystal.
    ///
    /// Required for Ethernet + WiFi coexistence (immune to on-chip noise).
    External {
        /// GPIO for clock input (only 0, 16, or 17).
        gpio: ClkGpio,
    },
}

/// GPIO pins that can carry the EMAC RMII reference clock.
///
/// Only GPIO0, GPIO16, and GPIO17 support EMAC clock I/O on ESP32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpio {
    /// GPIO0 — also used for boot strapping, use with caution.
    Gpio0,
    /// GPIO16 — EMAC_CLK_OUT (0° phase).
    Gpio16,
    /// GPIO17 — EMAC_CLK_OUT_180 (180° phase, most common for LAN8720A).
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
