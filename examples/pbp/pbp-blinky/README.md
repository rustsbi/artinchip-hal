# PBP Blinky

Blink LED on PA5. Currently we light up LED on PA5.

## Build

```
cargo build -p pbp-blinky --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-blinky target/riscv32imac-unknown-none-elf/release/pbp-blinky.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-blinky.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-blinky.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-blinky.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
