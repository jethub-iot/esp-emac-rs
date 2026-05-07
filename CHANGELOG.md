# Changelog

All notable changes to `esp-emac` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-06

### Breaking

- `Speed` and `Duplex` are now re-exports of `eth_mdio_phy::{Speed,
  Duplex}` (gated by feature `mdio-phy`) rather than locally
  defined enums with `From<eth_mdio_phy::*>` conversions. Call
  sites that used to write `emac.set_speed(status.speed.into())`
  drop the `.into()` — the types are literally the same now.
  Without the `mdio-phy` feature the symbols are no longer exposed
  from the crate; drop down to
  `crate::regs::mac::set_speed_100mbps` /
  `set_duplex_full` for raw register control.
- `Emac::stop()` now returns `Err(EmacError::TxFlushTimeout)` when
  the FTF poll exhausts `TX_FIFO_FLUSH_TIMEOUT_US` instead of
  silently returning `Ok(())`. The rest of the teardown still runs
  unconditionally, so the driver state still ends up at
  `Initialized` and `start()` is safe to retry — the new error is
  a recoverable warning, not a state-machine corruption signal.
  Callers that pattern-match on `Ok(())` need a wildcard or
  explicit handling of the new variant.
- MSRV bumped from 1.75 to 1.88, matching what `esp-hal = "1.1"`
  declares in its own manifest (`esp-hal 1.0` carried the same
  1.88 pin). The previous declaration was mis-advertised.
- Bump `eth-mdio-phy` dependency pin from `^0.1.1` to `^0.2.0`.
  Trait crate's `Speed`/`Duplex` became `#[non_exhaustive]` in
  that release.
- Bump `embassy-sync` from `^0.7` to `^0.8` (cascade from
  `esp-hal 1.1` requirement).
- Bump `esp-hal` from `^1.0.0` to `^1.1.0`.

### Added

- `EmacError::TxFlushTimeout` variant (lands as a non-breaking
  variant addition because `EmacError` is `#[non_exhaustive]`).
- `Emac::set_speed` / `set_duplex` now match each `Speed`/`Duplex`
  variant explicitly. The trait-crate types became `#[non_exhaustive]`
  in `eth-mdio-phy 0.2`, so a future variant (e.g. a hypothetical
  `Speed::Mbps1000`) compiles transparently but has no register
  encoding on ESP32 EMAC. Such inputs are clamped to 100 Mbps
  (highest mode the peripheral physically supports) / Full duplex,
  with a `defmt::warn!` under the `defmt` feature flagging the
  mismatch.

### Fixed

- Two clippy errors under `cargo +esp clippy --target
  xtensa-esp32-none-elf -D warnings`:
  `redundant_guards` at `emac.rs:208` (collapsed
  `RmiiClockConfig::InternalApll { gpio, .. } if matches!(gpio, ClkGpio::Gpio0)`
  into `RmiiClockConfig::InternalApll { gpio: ClkGpio::Gpio0, .. }`)
  and `let_unit_value` at `emac.rs:474`
  (`esp_hal::interrupt::enable` returns `()`, the prior
  `let _ =` was dead).

### Documentation

- New "Recovery from task respawn" section in the embassy-net
  module rustdoc covering the `static mut EMAC` re-borrow path:
  `init()` is one-shot, so a respawned task must call `stop()` +
  `start()` to bring the engine back up rather than silently
  ignoring `EmacError::AlreadyInitialized`.
- `[package.metadata.docs.rs]` now sets `default-target =
  "riscv32imc-unknown-none-elf"` and drops the `esp-hal` feature.
  docs.rs cannot satisfy `xtensa-esp32-none-elf` (rustc upstream
  has no xtensa target, and docs.rs does not carry the Microchip
  fork), so the previous metadata produced a "Documentation:
  failed" badge. Matches the convention every other esp-rs crate
  (esp-hal, esp-println, esp-backtrace, esp-storage, esp-radio)
  already uses.
- Drop the WIP shields.io badge from README and remove the
  `### Pre-publication` section now that the crate ships on
  crates.io as 0.2.0.

## [0.1.2] - 2026-05-04

### Added

- `pub const DEFAULT_RX = 10`, `DEFAULT_TX = 10`, `DEFAULT_BUF = 1600`,
  `SMALL_RX = 4`, `SMALL_TX = 4` in `src/emac.rs` — single source of
  truth for the canonical ring sizings.
- `EmacDefault` / `EmacSmall` (MAC side) and the new
  `embassy::EmacDefaultDriver<'d>` / `embassy::EmacSmallDriver<'d>`
  driver-side aliases are all expressed in terms of those constants,
  so retuning a sizing updates all four aliases together. The driver
  aliases collapse `embassy_executor::task` signatures from
  `Runner<'static, EmacDriver<'static, 10, 10, 1600>>` to
  `Runner<'static, EmacDefaultDriver<'static>>`.

### Documentation

- Driver construction sites use `EmacDefaultDriver::new(emac, &state)`
  (the type alias's inherent `new`) instead of `EmacDriver::new(...)`,
  so the call site doesn't repeat the const generics.
- New "Lifetime alignment" section in `src/embassy.rs` accurately
  describing the constraints: `Emac` is a hardware-peripheral
  singleton (one EMAC on the ESP32, global MMIO), but
  `EmacDriverState` is **not** strictly singleton — multiple
  instances are fine for tests / sequential re-init; the constraint
  is that the ISR-bound instance must match the one passed to
  `EmacDriver::new`. Replaces the previous, overstated "per-process
  singleton" wording.
- Recommend `static mut EMAC: EmacDefault = EmacDefault::new(...)` +
  `unsafe { &mut *core::ptr::addr_of_mut!(EMAC) }` as the canonical
  storage pattern. `Emac::new` is `const fn`, so the value lives in
  BSS — no runtime stack involvement on boot. A
  `StaticCell::init(EmacDefault::new(...))` wrapper would risk
  materialising the ~32 KiB `EmacDefault` on the calling task's
  stack frame before moving it into the cell, which is enough to
  overflow tight ESP32 task stacks. Documentation in README,
  `src/lib.rs`, `src/embassy.rs`, and `examples/embassy_net_lan8720a.rs`
  all use this pattern with an explanatory comment.

### Notes

- `Emac::start()` and `Emac::stop()` are already idempotent (returning
  `Ok(())` on the matching state) — no changes needed; this changelog
  entry exists to make that contract visible after the migration §5.4
  audit.

## [0.1.1] - 2026-05-03

### Documentation

- Rewrite `README.md` as an integration guide: installation, feature
  matrix, compatibility table, full embassy-net + LAN8720A example,
  ISR binding recipe, troubleshooting checklist.
- Expand crate-level rustdoc in `src/lib.rs` to mirror the README on
  docs.rs.
- Add runnable example `examples/embassy_net_lan8720a.rs` (build with
  `--features esp-hal,mdio-phy,embassy-net --target xtensa-esp32-none-elf`).
- Add `documentation` and `readme` fields to `Cargo.toml`.

### Internal

- Move bare-metal example dependencies under
  `[target.'cfg(target_os = "none")'.dev-dependencies]` so `cargo test`
  on the host stays unaffected.

## [0.1.0] - 2026-04-29

### Added

- Initial public release.
- `Emac<RX, TX, BUF>` driver with native MAC/DMA bring-up.
- `EmacDriver` adaptor for `embassy-net` (feature `embassy-net`).
- `EspMdio` Station Management controller.
- `regs::{mac, dma, ext, gpio}` typed register helpers.
- `reset::ResetController` (blocking) and
  `reset::async_impl::AsyncResetController` (feature `async`).
- `clock` module — APLL 50 MHz programming and GPIO0/16/17 routing.

[0.2.0]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jethub-iot/esp-emac-rs/releases/tag/v0.1.0
