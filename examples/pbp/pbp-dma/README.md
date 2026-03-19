# PBP Dma

## Build

```
cargo build -p pbp-dma --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-dma target/riscv32imac-unknown-none-elf/release/pbp-dma.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-dma.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-dma.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-dma.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
