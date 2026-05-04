# Changelog

All notable changes to `esp-emac` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-05-04

### Added

- `embassy::EmacDefaultDriver<'d>` and `embassy::EmacSmallDriver<'d>`
  type aliases for the `Emac<10, 10, 1600>` and `Emac<4, 4, 1600>`
  ring sizings — pair with the existing `EmacDefault` / `EmacSmall`
  aliases so users only spell out the const generics in one place.
  In particular `embassy_executor::task` signatures collapse from
  `Runner<'static, EmacDriver<'static, 10, 10, 1600>>` to
  `Runner<'static, EmacDefaultDriver<'static>>`.

### Documentation

- README quick-start, `src/lib.rs` crate-level rustdoc, `src/embassy.rs`
  module-level rustdoc, and `examples/embassy_net_lan8720a.rs` all
  switched from `static mut EMAC: Emac<...> = Emac::new(...)` +
  `unsafe { &mut *core::ptr::addr_of_mut!(EMAC) }` to the safer
  `static EMAC: StaticCell<EmacDefault> = StaticCell::new();` +
  `EMAC.init(EmacDefault::new(...))` pattern. No `static mut` and no
  `unsafe` block in the user-facing example.
- New "Singleton design" section in `src/embassy.rs` explaining why
  both `Emac` and `EmacDriverState` are designed as per-process
  singletons (one EMAC peripheral on the chip, ISR symbol points at a
  single `static`), with a stronger doc-comment on `EmacDriver`
  capturing the same constraint.

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

[0.1.2]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jethub-iot/esp-emac-rs/releases/tag/v0.1.0
