# PBP Hello World

## Build

```
cargo build -p pbp-hello-world --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-hello-world target/riscv32imac-unknown-none-elf/release/pbp-hello-world.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-hello-world.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-hello-world.pbp
```
