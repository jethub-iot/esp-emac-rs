// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! DMA software reset state machine.
//!
//! Wraps the `DMABUSMODE.SWR` bit-poll loop in a small struct that takes
//! a [`DelayNs`] implementation, so the same routine works equally well
//! from a blocking or async context.

use embedded_hal::delay::DelayNs;

use crate::regs::dma::{self, bus_mode, DMABUSMODE};

/// Default soft-reset timeout (matches ESP-IDF / ph-esp32-mac).
pub const SOFT_RESET_TIMEOUT_MS: u32 = 100;
/// Polling interval while waiting for `DMABUSMODE.SWR` to self-clear.
pub const RESET_POLL_INTERVAL_US: u32 = 100;

/// Reset failure cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetError {
    /// `DMABUSMODE.SWR` did not self-clear within the configured timeout.
    Timeout,
}

/// Owns a [`DelayNs`] implementation and exposes the soft-reset routine.
pub struct ResetController<D: DelayNs> {
    delay: D,
    timeout_ms: u32,
}

impl<D: DelayNs> ResetController<D> {
    /// Build a controller with the [`SOFT_RESET_TIMEOUT_MS`] default.
    pub fn new(delay: D) -> Self {
        Self {
            delay,
            timeout_ms: SOFT_RESET_TIMEOUT_MS,
        }
    }

    /// Build a controller with a caller-chosen timeout.
    pub fn with_timeout(delay: D, timeout_ms: u32) -> Self {
        Self { delay, timeout_ms }
    }

    /// Issue the DMA software reset and wait for `DMABUSMODE.SWR` to
    /// self-clear. Returns [`ResetError::Timeout`] if it does not happen
    /// within `timeout_ms`. The reset clears the entire DMA + MAC core to
    /// its hardware-default state.
    pub fn soft_reset(&mut self) -> Result<(), ResetError> {
        // Setting the SWR bit triggers the reset; the bit auto-clears when
        // the controller is back to a known state.
        // SAFETY: DMABUSMODE is a known-valid 32-bit register.
        unsafe { dma::set_bits(DMABUSMODE, bus_mode::SW_RESET) };

        // Compute in u64 then clamp to u32 so `timeout_ms` values past
        // ~4.3 s (where `timeout_ms * 1000` would wrap a u32) still
        // produce a sane upper bound on the polling loop.
        let max_iters = (u64::from(self.timeout_ms) * 1000 / u64::from(RESET_POLL_INTERVAL_US))
            .min(u64::from(u32::MAX)) as u32;
        for _ in 0..max_iters {
            // SAFETY: same address, read-only volatile.
            let still_in_progress = unsafe { dma::read(DMABUSMODE) } & bus_mode::SW_RESET != 0;
            if !still_in_progress {
                return Ok(());
            }
            self.delay.delay_us(RESET_POLL_INTERVAL_US);
        }
        Err(ResetError::Timeout)
    }

    /// Configured timeout in milliseconds.
    pub fn timeout_ms(&self) -> u32 {
        self.timeout_ms
    }
}
