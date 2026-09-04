# PBP Async Uart

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-async-uart --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-async-uart target/riscv32imafc-unknown-none-elf/release/pbp-async-uart.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-async-uart.bin --spi-nor --raw-img
```

The converter writes `pbp-async-uart.pbp` and `pbp-async-uart.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-async-uart-out/`.
