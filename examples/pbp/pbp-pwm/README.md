# PBP PWM

## Build

```
cargo build -p pbp-pwm --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-pwm target/riscv32imac-unknown-none-elf/release/pbp-pwm.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-pwm.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-pwm.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-pwm.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
