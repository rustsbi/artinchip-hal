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

From this directory, one command builds every package the manifest names and
packs the image:

```bash
cd examples/app/app-bootloader
cargo run -p aicfwc -- --build --toml .
```

Or from the workspace root, naming the directory instead of using `.`:

```bash
cargo run -p aicfwc -- --build --toml examples/app/app-bootloader
```

`--toml` takes a directory and picks its single manifest. `--build` runs one
`cargo build` per component `build_package`, in manifest order; `pbp-common` has
to be built on its own invocation, which the per-package loop guarantees.

Output:

```text
target/riscv32imafc-unknown-none-elf/release/app-bootloader-out/app-bootloader.img
```

The loader is linked as an AIC **loader** image: `load_address` / `entry_offset`
and the on-flash layout come from `app-bootloader.toml`, so nothing depends on a
shell environment variable. Burning the result with the host tool is the same as
any vendor image.

## Notes

* Every PBP slot carries our own `pbp-common`; there is no vendor PBP and no
  `pbp_cfg.bin`.
* `[image] app` empty boots the first valid AIC image after the `bootloader`
  partition; set it to a name to pin one.
