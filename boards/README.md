# Board presets

Each `boards/*.toml` is the single source of truth for one board: chip,
Cargo target, media, AIC.FW metadata, partition table, boot chain and
applications. `aicfwc` and the firmware `build.rs` scripts both read it.

| Preset | Chip | Target | Media |
|:--|:--|:--|:--|
| [`d13x-nor.toml`](d13x-nor.toml) | D13x (RV32 E907) | `riscv32imafc-unknown-none-elf` | SPI-NOR |
| [`d13x-nor-quad-dma.toml`](d13x-nor-quad-dma.toml) | D13x (RV32 E907) | `riscv32imafc-unknown-none-elf` | SPI-NOR, quad + DMA |
| [`d21x-nand.toml`](d21x-nand.toml) | D21x (RV64 C906) | `riscv64gc-unknown-none-elf` | SPI-NAND |
| [`d21x-nand-quad-dma.toml`](d21x-nand-quad-dma.toml) | D21x (RV64 C906) | `riscv64gc-unknown-none-elf` | SPI-NAND, quad + DMA |

## Packaging

```bash
cargo run -p aicfwc -- --board d13x-nor --bootloader --build
cargo run -p aicfwc -- --board d13x-nor --app app-blinky --build
cargo run -p aicfwc -- --board d21x-nand --bootloader --build
cargo run -p aicfwc -- --board d21x-nand --app app-blinky --build
```

`--build` builds the packages the image needs; without it already-built
artifacts are packed. Output goes to `<target-dir>/<triple>/<profile>/<app>-out/`.

Out-of-tree presets: `--toml <file>`, or `AICFW_BOARDS_DIR` for `--board`.
Components use `features` for their own package and optional `pbp_features`
for `pbp_package`; the latter defaults to `features`.

The schema and validation rules are in [`aicfwc/src/preset.rs`](../aicfwc/src/preset.rs)
and `aicfwc/src/preset/`.
