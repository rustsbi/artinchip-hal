# PBP Boot Info

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-boot-info --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-boot-info target/riscv32imafc-unknown-none-elf/release/pbp-boot-info.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-boot-info.bin --spi-nor --raw-img
```

The converter writes `pbp-boot-info.pbp` and `pbp-boot-info.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-boot-info-out/`.
