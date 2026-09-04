# PBP I2C Master

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-i2c-master --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-i2c-master target/riscv32imafc-unknown-none-elf/release/pbp-i2c-master.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-i2c-master.bin --spi-nor --raw-img
```

The converter writes `pbp-i2c-master.pbp` and `pbp-i2c-master.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-i2c-master-out/`.
