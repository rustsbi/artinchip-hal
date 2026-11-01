# PBP Dram

D21x early-stage DRAM bring-up example. It configures the controller and PHY
through `artinchip_hal::dram`, checks the PHY handshake, and sweeps the decoded
DRAM window with a diagnostic pattern.

## Build

```bash
cargo build -p pbp-dram --target riscv64gc-unknown-none-elf --release
rust-objcopy -O binary target/riscv64gc-unknown-none-elf/release/pbp-dram target/riscv64gc-unknown-none-elf/release/pbp-dram.bin
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-dram.bin --raw-img --spi-nand
```

The converter writes the raw PBP image under
`target/riscv64gc-unknown-none-elf/release/pbp-dram-out/`.
