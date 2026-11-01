# App Blinky

Application image booted by [`app-bootloader`](../app-bootloader) on a normal
(non-upgrade) start. It toggles **PA5** every 600 ms and prints one line over
UART0, so it is unambiguous whether the boot path worked.

## Build

The board presets in [`boards/`](../../../boards/README.md) describe the image;
from the workspace root, one command builds every package it needs and packs it:

```bash
# D13x / SPI-NOR
cargo run -p aicfwc -- --board d13x-nor --app app-blinky --build

# D21x / SPI-NAND
cargo run -p aicfwc -- --board d21x-nand --app app-blinky --build
```

`--app app-blinky` packages the boot chain plus this application; `--build` runs
one `cargo build` per package the image needs (`pbp-common` always gets its own
invocation).

Output (the image plus each piece it is made of):

```text
target/riscv32imafc-unknown-none-elf/release/app-blinky-out/app-blinky.img
```

## How it is booted

`boards/d13x-nor.toml` (and `boards/d21x-nand.toml`) declares the app in the
`app` partition — its AIC name, file name and link base — and `[image] app = ""`
tells the loader to boot the first valid AIC image after its own partition
(setting `app = "blinky"` pins it by name instead).

On D13x the app is linked at `0x4070_0000` (PSRAM), clear of the PSRAM pattern
test, the updater RAM slot and the bootloader; on D21x it is linked into DRAM at
`0x4200_0000`. Both addresses are declared in the board presets, and this crate's
`build.rs` passes the matching one to the linker. No PBP runs before an app: the
loader has already brought PSRAM/DDR up, so the app itself is a plain load +
jump. `uart_logger_init` runs again here because loggers do not carry across
images.
