# PBP PSRAM

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-psram --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-psram target/riscv32imafc-unknown-none-elf/release/pbp-psram.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-psram.bin --spi-nor --raw-img
```

The converter writes `pbp-psram.pbp` and `pbp-psram.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-psram-out/`.
