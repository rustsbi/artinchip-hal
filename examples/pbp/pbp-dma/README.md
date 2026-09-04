# PBP Dma

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-dma --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-dma target/riscv32imafc-unknown-none-elf/release/pbp-dma.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-dma.bin --spi-nor --raw-img
```

The converter writes `pbp-dma.pbp` and `pbp-dma.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-dma-out/`.
