# App Bootloader

A minimal AIC USB upgrade bootloader written from scratch. It enumerates as the
ArtInChip upgrade device (VID `0x33c3` / PID `0x6677`, vendor-specific bulk) and
speaks the `UPGC`/`UPGR` protocol, so the official host tool can burn flash
through it. The same image also has a normal-boot path that jumps to the app in
the `app` partition.

The goal is compatibility with the host flasher, not a full clone of the vendor
bootloader. Protocol and reverse-engineering notes live in
`docs/PBP-BOOTLOADER-REPORT.md`.

## Roles

The first stage (`pbp-common`) publishes the role it detected, and the loader
picks its path from it:

* **Upgrade** — the ROM's USB downloader started us, so serve the upgrade loop.
* **Boot** — a cold start, so listen briefly for a host and then boot the app.

## Build

The board presets in [`boards/`](../../../boards/README.md) describe the image;
from the workspace root, one command builds every package it needs and packs it:

```bash
# D13x / SPI-NOR
cargo run -p aicfwc -- --board d13x-nor --bootloader --build

# D21x / SPI-NAND
cargo run -p aicfwc -- --board d21x-nand --bootloader --build
```

`--bootloader` packages the boot chain alone — the PBP updaters plus this
bootloader, no application; use `--app <package>` for a product image.
`--build` runs one `cargo build` per package the image needs, in package order;
`pbp-common` has to be built on its own invocation, which the per-package loop
guarantees.

Output (the image plus each piece it is made of):

```text
target/riscv32imafc-unknown-none-elf/release/app-bootloader-out/app-bootloader.img
```

The loader is linked as an AIC **loader** image: `load_address` / `entry_offset`
and the on-flash layout come from `boards/d13x-nor.toml` (or
`boards/d21x-nand.toml`), and this crate's `build.rs` reads the same file, so
nothing depends on a shell environment variable. Burning the result with the host
tool is the same as any vendor image.

## Notes

* Every PBP slot carries our own `pbp-common`; there is no vendor PBP and no
  `pbp_cfg.bin`.
* `[image] app` empty boots the first valid AIC image after the `bootloader`
  partition; set it to a name to pin one. It lives in the board preset.
