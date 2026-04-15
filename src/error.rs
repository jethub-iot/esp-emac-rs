// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! EMAC error types.

/// EMAC driver error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EmacError {
    /// MDIO operation timed out.
    Timeout,
    /// DMA descriptor or buffer error.
    DmaError,
    /// Invalid configuration (bad pin, clock, etc.).
    InvalidConfig,
    /// PHY address out of range (must be 0-31).
    InvalidPhyAddress,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate alloc;

    #[test]
    fn emac_error_debug() {
        let err = EmacError::Timeout;
        let dbg = alloc::format!("{:?}", err);
        assert!(dbg.contains("Timeout"));
    }

    #[test]
    fn emac_error_equality() {
        assert_eq!(EmacError::Timeout, EmacError::Timeout);
        assert_ne!(EmacError::Timeout, EmacError::DmaError);
        assert_ne!(EmacError::InvalidConfig, EmacError::InvalidPhyAddress);
    }

    #[test]
    fn emac_error_clone() {
        let err = EmacError::DmaError;
        let cloned = err;
        assert_eq!(err, cloned);
    }
}
