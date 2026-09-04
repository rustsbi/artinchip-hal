# PBP Blinky

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-blinky --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-blinky target/riscv32imafc-unknown-none-elf/release/pbp-blinky.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-blinky.bin --raw-img --spi-nor
```

### D21x (C906 series)

```bash
cargo build -p pbp-blinky --no-default-features --features d21x --target riscv64gc-unknown-none-elf --release
rust-objcopy -O binary target/riscv64gc-unknown-none-elf/release/pbp-blinky target/riscv64gc-unknown-none-elf/release/pbp-blinky.bin
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-blinky.bin --raw-img --spi-nand
```

Without `--raw-img`, the converter only validates and repairs the PBP checksum:

```bash
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-blinky.bin
```

The converter writes `pbp-blinky.pbp` and, when `--raw-img` is set,
`pbp-blinky.img` under `target/{arch}/release/pbp-blinky-out/`.
