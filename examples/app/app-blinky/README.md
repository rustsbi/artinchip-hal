# App Blinky

Application image booted by [`app-bootloader`](../app-bootloader) on a normal
(non-upgrade) start. It toggles **PE17** every 500 ms and prints one line over
UART0, so it is unambiguous whether the boot path worked.

## Build

From this directory, one command builds every package the manifest names and
packs the image:

```bash
cd examples/app/app-blinky
cargo run -p aicfwc -- --build --toml .
```

Or from the workspace root, naming the directory instead of using `.`:

```bash
cargo run -p aicfwc -- --build --toml examples/app/app-blinky
```

`--build` runs one `cargo build` per component `build_package`; `pbp-common` has
to be built on its own invocation, which the per-package loop guarantees.

Output:

```text
target/riscv32imafc-unknown-none-elf/release/app-blinky-out/app-blinky.img
```

## How it is booted

`app-blinky.toml` declares the app in the `app` partition, and `[image] app = ""`
tells the loader to boot the first valid AIC image after its own partition
(setting `app = "blinky"` pins it by name instead).

The app is linked at `0x4070_0000`, clear of the PSRAM pattern test, the updater
RAM slot and the bootloader. No PBP runs before an app: the loader has already
brought PSRAM up, so the app itself is a plain load + jump. `uart_logger_init`
runs again here because loggers do not carry across images.
