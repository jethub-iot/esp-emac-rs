// SPDX-License-Identifier: GPL-2.0-or-later OR Apache-2.0
// Copyright (c) Viacheslav Bocharov <v@baodeep.com> and JetHome (r)

//! DMA controller register definitions.
//!
//! The EMAC DMA controller manages data transfers between the MAC
//! and system memory using descriptor-based scatter-gather DMA.
//! Base address: `0x3FF6_9000`.

#![allow(dead_code)]

// =============================================================================
// Base Address
// =============================================================================

/// DMA register block base address (ESP32).
pub const BASE: usize = 0x3FF6_9000;

// =============================================================================
// Register Offsets
// =============================================================================

/// Bus Mode Register.
pub const DMABUSMODE: usize = 0x00;
/// TX Poll Demand Register.
pub const DMATXPOLLDEMAND: usize = 0x04;
/// RX Poll Demand Register.
pub const DMARXPOLLDEMAND: usize = 0x08;
/// RX Descriptor List Address Register.
pub const DMARXBASEADDR: usize = 0x0C;
/// TX Descriptor List Address Register.
pub const DMATXBASEADDR: usize = 0x10;
/// Status Register.
pub const DMASTATUS: usize = 0x14;
/// Operation Mode Register.
pub const DMAOPERATION: usize = 0x18;
/// Interrupt Enable Register.
pub const DMAINTENABLE: usize = 0x1C;
/// Missed Frame and Buffer Overflow Counter Register.
pub const DMAMISSEDFR: usize = 0x20;
/// Receive Interrupt Watchdog Timer Register.
pub const DMARXWATCHDOG: usize = 0x24;
/// Current Host TX Descriptor Register (read-only).
pub const DMACURTXDESC: usize = 0x48;
/// Current Host RX Descriptor Register (read-only).
pub const DMACURRXDESC: usize = 0x4C;
/// Current Host TX Buffer Address Register (read-only).
pub const DMACURTXBUFADDR: usize = 0x50;
/// Current Host RX Buffer Address Register (read-only).
pub const DMACURRXBUFADDR: usize = 0x54;

// =============================================================================
// DMABUSMODE bits
// =============================================================================

/// Bit-field constants for the DMA Bus Mode Register.
pub mod bus_mode {
    /// Software Reset — resets all EMAC logic, cleared automatically.
    pub const SW_RESET: u32 = 1 << 0;
    /// DMA Arbitration Scheme: 0 = round-robin, 1 = fixed priority.
    pub const DMA_ARB: u32 = 1 << 1;
    /// Descriptor Skip Length shift (number of dwords to skip).
    pub const DSL_SHIFT: u32 = 2;
    /// Descriptor Skip Length mask.
    pub const DSL_MASK: u32 = 0x1F << 2;
    /// Alternate Descriptor Size (8 dwords instead of 4).
    pub const ATDS: u32 = 1 << 7;
    /// Programmable Burst Length shift (max beats per DMA transaction).
    pub const PBL_SHIFT: u32 = 8;
    /// Programmable Burst Length mask.
    pub const PBL_MASK: u32 = 0x3F << 8;
    /// Fixed Burst: 0 = variable length, 1 = fixed length.
    pub const FIXED_BURST: u32 = 1 << 16;
    /// RX DMA Programmable Burst Length shift (when USP=1).
    pub const RPBL_SHIFT: u32 = 17;
    /// RX DMA Programmable Burst Length mask.
    pub const RPBL_MASK: u32 = 0x3F << 17;
    /// Use Separate PBL: 1 = use RPBL for RX, PBL for TX.
    pub const USP: u32 = 1 << 23;
    /// PBL x8 Mode: multiplies PBL/RPBL by 8.
    pub const PBL_X8: u32 = 1 << 24;
    /// Address Aligned Beats.
    pub const AAL: u32 = 1 << 25;
    /// Mixed Burst.
    pub const MIXED_BURST: u32 = 1 << 26;
    /// Transmit Priority: 0 = round-robin, 1 = TX has priority.
    pub const TX_PRIORITY: u32 = 1 << 27;
}

// =============================================================================
// DMASTATUS bits
// =============================================================================

/// Bit-field constants for the DMA Status Register.
pub mod status {
    /// Transmit Interrupt — frame transmission complete.
    pub const TI: u32 = 1 << 0;
    /// Transmit Process Stopped.
    pub const TPS: u32 = 1 << 1;
    /// Transmit Buffer Unavailable.
    pub const TU: u32 = 1 << 2;
    /// Transmit Jabber Timeout.
    pub const TJT: u32 = 1 << 3;
    /// Receive Overflow.
    pub const OVF: u32 = 1 << 4;
    /// Transmit Underflow.
    pub const UNF: u32 = 1 << 5;
    /// Receive Interrupt — frame reception complete.
    pub const RI: u32 = 1 << 6;
    /// Receive Buffer Unavailable.
    pub const RU: u32 = 1 << 7;
    /// Receive Process Stopped.
    pub const RPS: u32 = 1 << 8;
    /// Receive Watchdog Timeout.
    pub const RWT: u32 = 1 << 9;
    /// Early Transmit Interrupt.
    pub const ETI: u32 = 1 << 10;
    /// Fatal Bus Error Interrupt.
    pub const FBI: u32 = 1 << 13;
    /// Early Receive Interrupt.
    pub const ERI: u32 = 1 << 14;
    /// Abnormal Interrupt Summary.
    pub const AIS: u32 = 1 << 15;
    /// Normal Interrupt Summary.
    pub const NIS: u32 = 1 << 16;
    /// Receive Process State shift (3-bit field at bits 19:17).
    pub const RS_SHIFT: u32 = 17;
    /// Receive Process State mask.
    pub const RS_MASK: u32 = 0x07 << 17;
    /// Transmit Process State shift (3-bit field at bits 22:20).
    pub const TS_SHIFT: u32 = 20;
    /// Transmit Process State mask.
    pub const TS_MASK: u32 = 0x07 << 20;
    /// Error Bits shift (3-bit field at bits 25:23).
    pub const EB_SHIFT: u32 = 23;
    /// Error Bits mask.
    pub const EB_MASK: u32 = 0x07 << 23;

    /// All interrupt status bits (for clearing).
    pub const ALL_INTERRUPTS: u32 =
        TI | TPS | TU | TJT | OVF | UNF | RI | RU | RPS | RWT | ETI | FBI | ERI | AIS | NIS;
}

// =============================================================================
// DMAOPERATION bits
// =============================================================================

/// Bit-field constants for the DMA Operation Mode Register.
pub mod operation {
    /// Start/Stop Receive: 1 = start DMA receive.
    pub const SR: u32 = 1 << 1;
    /// Operate on Second Frame.
    pub const OSF: u32 = 1 << 2;
    /// Receive Threshold Control shift (2-bit field at bits 4:3).
    pub const RTC_SHIFT: u32 = 3;
    /// Receive Threshold Control mask.
    pub const RTC_MASK: u32 = 0x03 << 3;
    /// Forward Undersized Good Frames.
    pub const FUF: u32 = 1 << 6;
    /// Forward Error Frames.
    pub const FEF: u32 = 1 << 7;
    /// Start/Stop Transmission: 1 = start DMA transmit.
    pub const ST: u32 = 1 << 13;
    /// Transmit Threshold Control shift (3-bit field at bits 16:14).
    pub const TTC_SHIFT: u32 = 14;
    /// Transmit Threshold Control mask.
    pub const TTC_MASK: u32 = 0x07 << 14;
    /// Flush Transmit FIFO.
    pub const FTF: u32 = 1 << 20;
    /// Transmit Store and Forward.
    pub const TSF: u32 = 1 << 21;
    /// Disable Flushing of Received Frames.
    pub const DFF: u32 = 1 << 24;
    /// Receive Store and Forward.
    pub const RSF: u32 = 1 << 25;
    /// Disable Dropping of TCP/IP Checksum Error Frames.
    pub const DT: u32 = 1 << 26;
}

// =============================================================================
// DMAINTENABLE bits
// =============================================================================

/// Bit-field constants for the DMA Interrupt Enable Register.
pub mod int_enable {
    /// Transmit Interrupt Enable.
    pub const TIE: u32 = 1 << 0;
    /// Transmit Stopped Enable.
    pub const TSE: u32 = 1 << 1;
    /// Transmit Buffer Unavailable Enable.
    pub const TUE: u32 = 1 << 2;
    /// Transmit Jabber Timeout Enable.
    pub const TJE: u32 = 1 << 3;
    /// Overflow Interrupt Enable.
    pub const OVE: u32 = 1 << 4;
    /// Underflow Interrupt Enable.
    pub const UNE: u32 = 1 << 5;
    /// Receive Interrupt Enable.
    pub const RIE: u32 = 1 << 6;
    /// Receive Buffer Unavailable Enable.
    pub const RUE: u32 = 1 << 7;
    /// Receive Stopped Enable.
    pub const RSE: u32 = 1 << 8;
    /// Receive Watchdog Timeout Enable.
    pub const RWE: u32 = 1 << 9;
    /// Early Transmit Interrupt Enable.
    pub const ETE: u32 = 1 << 10;
    /// Fatal Bus Error Enable.
    pub const FBE: u32 = 1 << 13;
    /// Early Receive Interrupt Enable.
    pub const ERE: u32 = 1 << 14;
    /// Abnormal Interrupt Summary Enable.
    pub const AIE: u32 = 1 << 15;
    /// Normal Interrupt Summary Enable.
    pub const NIE: u32 = 1 << 16;

    /// Default interrupt enable mask (normal operation).
    pub const DEFAULT: u32 = TIE | RIE | FBE | AIE | NIE;
}

// =============================================================================
// Register access helpers
// =============================================================================

/// Read a DMA register at `offset` from BASE.
///
/// # Safety
/// Caller must ensure the EMAC peripheral clock is enabled and
/// `offset` is a valid register offset within this block.
#[inline(always)]
pub unsafe fn read(offset: usize) -> u32 {
    // SAFETY: caller guarantees address validity.
    core::ptr::read_volatile((BASE + offset) as *const u32)
}

/// Write a DMA register at `offset` from BASE.
///
/// # Safety
/// Caller must ensure the EMAC peripheral clock is enabled and
/// `offset` is a valid register offset within this block.
#[inline(always)]
pub unsafe fn write(offset: usize, val: u32) {
    // SAFETY: caller guarantees address validity.
    core::ptr::write_volatile((BASE + offset) as *mut u32, val);
}

/// Read-modify-write: set bits in a DMA register.
///
/// # Safety
/// Same requirements as [`read`] and [`write`].
#[inline(always)]
pub unsafe fn set_bits(offset: usize, bits: u32) {
    let val = read(offset);
    write(offset, val | bits);
}

/// Read-modify-write: clear bits in a DMA register.
///
/// # Safety
/// Same requirements as [`read`] and [`write`].
#[inline(always)]
pub unsafe fn clear_bits(offset: usize, bits: u32) {
    let val = read(offset);
    write(offset, val & !bits);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_address() {
        assert_eq!(BASE, 0x3FF6_9000);
    }

    #[test]
    fn register_offsets_within_block() {
        // DMA register block is 0x800 (2 KiB, up to EXT at 0x9800)
        let offsets = [
            DMABUSMODE,
            DMATXPOLLDEMAND,
            DMARXPOLLDEMAND,
            DMARXBASEADDR,
            DMATXBASEADDR,
            DMASTATUS,
            DMAOPERATION,
            DMAINTENABLE,
            DMAMISSEDFR,
            DMARXWATCHDOG,
            DMACURTXDESC,
            DMACURRXDESC,
            DMACURTXBUFADDR,
            DMACURRXBUFADDR,
        ];
        for off in offsets {
            assert!(off < 0x800, "offset {:#x} exceeds DMA block size", off);
        }
    }

    #[test]
    fn status_bits_no_overlap() {
        let bits = [
            status::TI,
            status::TPS,
            status::TU,
            status::TJT,
            status::OVF,
            status::UNF,
            status::RI,
            status::RU,
            status::RPS,
            status::RWT,
            status::ETI,
            status::FBI,
            status::ERI,
            status::AIS,
            status::NIS,
        ];
        for i in 0..bits.len() {
            for j in (i + 1)..bits.len() {
                assert_eq!(
                    bits[i] & bits[j],
                    0,
                    "status bits {:#x} and {:#x} overlap",
                    bits[i],
                    bits[j]
                );
            }
        }
    }

    #[test]
    fn all_interrupts_covers_every_status_bit() {
        let manual = status::TI
            | status::TPS
            | status::TU
            | status::TJT
            | status::OVF
            | status::UNF
            | status::RI
            | status::RU
            | status::RPS
            | status::RWT
            | status::ETI
            | status::FBI
            | status::ERI
            | status::AIS
            | status::NIS;
        assert_eq!(status::ALL_INTERRUPTS, manual);
    }

    #[test]
    fn int_enable_bits_no_overlap() {
        let bits = [
            int_enable::TIE,
            int_enable::TSE,
            int_enable::TUE,
            int_enable::TJE,
            int_enable::OVE,
            int_enable::UNE,
            int_enable::RIE,
            int_enable::RUE,
            int_enable::RSE,
            int_enable::RWE,
            int_enable::ETE,
            int_enable::FBE,
            int_enable::ERE,
            int_enable::AIE,
            int_enable::NIE,
        ];
        for i in 0..bits.len() {
            for j in (i + 1)..bits.len() {
                assert_eq!(
                    bits[i] & bits[j],
                    0,
                    "int_enable bits {:#x} and {:#x} overlap",
                    bits[i],
                    bits[j]
                );
            }
        }
    }

    #[test]
    fn operation_start_stop_bits_distinct() {
        assert_eq!(operation::SR & operation::ST, 0);
    }

    #[test]
    fn bus_mode_pbl_field_position() {
        // PBL = 1 should produce 1 << 8
        let pbl1 = 1u32 << bus_mode::PBL_SHIFT;
        assert_eq!(pbl1, 0x100);
        assert_eq!(pbl1 & bus_mode::PBL_MASK, pbl1);
    }

    #[test]
    fn dma_base_before_ext_base() {
        // DMA: 0x3FF6_9000, EXT: 0x3FF6_9800
        assert!(BASE < 0x3FF6_9800);
    }
}
