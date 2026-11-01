# PBP Flash

Boot-flash configuration report and partition read/write test.

It prints the selected configuration, the JEDEC id and the boot-chain partition
headers, then exercises the selected read/write path on the last sector/block of
the flash (a scratch area, no partition owns it):

- `nor` / `nand` choose the medium;
- `quad` selects the four-line data mode (QE is enabled and left enabled; it
  does not affect single-line operation);
- `dma` moves bulk transfers through the SoC GPDMA instead of the QSPI FIFO.

The PBP runs from the BootROM with no RAM brought up, so it is also the cheapest
way to check a pin assignment: wrong wiring shows up as an id-read error.

## D13x (E907) - SPI-NOR on QSPI0

| Signal | Pin |
|:--|:--|
| SCK | PB4 |
| MOSI | PB5 |
| MISO | PB1 |
| CS | PB2 (GPIO output) |
| WP / IO2 | PB0 (quad only) |
| HOLD / IO3 | PB3 (quad only) |

Console: UART0 (PA0 TX, PA1 RX). Status LED: PA5.

```bash
cargo build -p pbp-flash --target riscv32imafc-unknown-none-elf --release
rust-objcopy -O binary target/riscv32imafc-unknown-none-elf/release/pbp-flash target/riscv32imafc-unknown-none-elf/release/pbp-flash.bin
cargo run -p aicfwc -- target/riscv32imafc-unknown-none-elf/release/pbp-flash.bin --spi-nor --raw-img
```

Feature combinations:

```bash
--features "d13x nor quad"
--features "d13x nor dma"
--features "d13x nor quad dma"
```

Expected output (quad + dma):

```
[INFO] Welcome to pbp flash example by artinchip-hal🦀!
[INFO] Config: NOR flash, QUAD data, GPDMA mode
[INFO] JEDEC ID=EF 40 18, 16 MiB, sck=100000000 Hz
[INFO] Partition headers:
[INFO]   0x000000: [41, 49, 43, 20] (AIC )
[INFO]   0x000100: [50, 42, 50, 20] (PBP )
[INFO] Write test (256 B @ 0x00FFF000): OK
```

The write test runs once and uses every enabled feature together (quad data
phase and/or GPDMA). Its scratch offset is derived from the JEDEC capacity, so a
different flash size gets a different last sector. Without `quad`/`dma` the
write line is absent. The app partition offset is not fixed, so it is not
probed here.

## D21x (C906, RV64) - SPI-NAND on QSPI0

| Signal | Pin |
|:--|:--|
| SCK | PB5 |
| MOSI | PB4 |
| MISO | PB3 |
| CS | PB2 (GPIO output) |
| WP / IO2 | PB1 (quad only) |
| HOLD / IO3 | PB0 (quad only) |

```bash
cargo build -p pbp-flash --target riscv64gc-unknown-none-elf --release --no-default-features --features "d21x nand"
rust-objcopy -O binary target/riscv64gc-unknown-none-elf/release/pbp-flash target/riscv64gc-unknown-none-elf/release/pbp-flash.bin
cargo run -p aicfwc -- target/riscv64gc-unknown-none-elf/release/pbp-flash.bin --spi-nand --raw-img
```

Expected output:

```
[INFO] Welcome to pbp flash example by artinchip-hal🦀!
[INFO] Config: NAND flash, SINGLE data, FIFO mode
[INFO] JEDEC ID=0xEFAA21, sck=100000000 Hz
[INFO] Partition headers:
[INFO]   page table     @ 0x00000000: [41, 49, 43, 50] (AICP)
[INFO]   boot image     @ 0x00000800: [41, 49, 43, 20] (AIC )
```

`quad` / `dma` add one write/verify test on the last block; the block index is
derived from the part geometry, not hard-coded.
