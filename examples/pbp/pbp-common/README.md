# PBP Common

The single PBP used by every PBP slot of the image: the `image.updater.psram`
and `image.updater.bootloader` updaters, and the flash-resident bootloader. It
brings PSRAM up only when it is not already up, then either jumps to the
bootloader or, in the `image.updater.psram` role, returns to its caller (the D13x
PBP entry is return-capable).

## Build

The image flow points the manifest at this crate's **ELF** and lets `aicfwc`
flatten and wrap it, so nothing has to be copied. From the manifest's directory:

```bash
cd examples/app/app-bootloader
cargo run -p aicfwc -- --build --toml .
```

Or from the workspace root, naming that directory:

```bash
cargo run -p aicfwc -- --build --toml examples/app/app-bootloader
```

Build this package **on its own**: `artinchip-rt`'s PBP entry and its `app` entry
are mutually exclusive crate-level features, which `aicfwc --build` guarantees by
building one package per invocation.

To inspect the PBP on its own (not needed for an image):

```bash
cargo build -p pbp-common --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-common target/riscv32imafc-unknown-none-elf/release/pbp-common.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-common.bin --raw-img --spi-nor
```

## Notes

* Role detection is hardware-based, not timer-based: PSRAM already decoding XIP
  means the `updater.psram` call preceded us, so this is the `updater.bootloader`
  run - boot the RAM container and do not bring PSRAM up again. Otherwise the
  ROM's downloader is either running (the `updater.psram` service call - return
  to it) or not (a cold boot - bring PSRAM up, then boot the flash container).
* A cold boot also honours the BOOT key, handing the chip to the ROM's upgrade
  entry - the only path that can still replace a corrupt first-stage image.
* The next stage is told its `Role` through `system::{set_role, role}`; the
  identify/copy/jump logic itself lives in `artinchip_hal::system`.
