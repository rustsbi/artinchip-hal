# PBP Async Uart

## Build

```
cargo build -p pbp-async-uart --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-async-uart target/riscv32imac-unknown-none-elf/release/pbp-async-uart.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-async-uart.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-async-uart.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-async-uart.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
