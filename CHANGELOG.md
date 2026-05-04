# Changelog

All notable changes to `esp-emac` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.1]: https://github.com/jethub-iot/esp-emac-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jethub-iot/esp-emac-rs/releases/tag/v0.1.0
