# PBP I2C Master

## Build

```
cargo build -p pbp-i2c-master --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-i2c-master target/riscv32imac-unknown-none-elf/release/pbp-i2c-master.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-i2c-master.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-i2c-master.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-i2c-master.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
