# PBP Hello World

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-hello-world --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-hello-world target/riscv32imafc-unknown-none-elf/release/pbp-hello-world.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-hello-world.bin --spi-nor --raw-img
```

### D21x (C906 series)

```bash
cargo build -p pbp-hello-world --no-default-features --features d21x --target riscv64gc-unknown-none-elf --release
rust-objcopy -O binary target/riscv64gc-unknown-none-elf/release/pbp-hello-world target/riscv64gc-unknown-none-elf/release/pbp-hello-world.bin
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-hello-world.bin --raw-img --spi-nand
```

Without `--raw-img`, the converter only validates and repairs the PBP checksum:

```bash
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-hello-world.bin
```

The converter writes `pbp-hello-world.pbp` and, when `--raw-img` is set,
`pbp-hello-world.img` under `target/{arch}/release/pbp-hello-world-out/`.
