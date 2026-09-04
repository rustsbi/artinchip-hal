# PBP PWM

## Build

### D13x (E907 series)

```bash
cargo build -p pbp-pwm --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-pwm target/riscv32imafc-unknown-none-elf/release/pbp-pwm.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-pwm.bin --spi-nor --raw-img
```

The converter writes `pbp-pwm.pbp` and `pbp-pwm.img` under
`target/riscv32imafc-unknown-none-elf/release/pbp-pwm-out/`.
