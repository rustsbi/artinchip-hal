# PBP Flash

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-flash --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-flash target/riscv32imafc-unknown-none-elf/release/pbp-flash.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-flash.bin --spi-nor --raw-img
```

The converter writes `pbp-flash.pbp` and `pbp-flash.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-flash-out/`.
