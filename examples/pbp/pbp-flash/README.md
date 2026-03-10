# PBP Flash

## Build

```
cargo build -p pbp-flash --target riscv32imac-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imac-unknown-none-elf/release/pbp-flash target/riscv32imac-unknown-none-elf/release/pbp-flash.bin
cargo run -p aicfwc -- target/riscv32imac-unknown-none-elf/release/pbp-flash.bin --pbp -o target/riscv32imac-unknown-none-elf/release/pbp-flash.pbp
```

Your PBP file will be ready at `target/riscv32imac-unknown-none-elf/release/pbp-flash.pbp`.

Packed PBP image will be ready at the same path but with `.pk_pbp` extension.
