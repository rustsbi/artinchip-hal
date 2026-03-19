#![no_std]
#![no_main]

use artinchip_hal::dma::*;
use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, core::cache::*, pbp_entry};
use embedded_io::Write;
use panic_halt as _;

#[repr(C, align(8))]
struct MemBuf([u32; 2000]);

static mut MEM_SRC: MemBuf = MemBuf([0u32; 2000]);
static mut MEM_DST: MemBuf = MemBuf([0xDEAD_BEEFu32; 2000]);

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();

    let tx = p.gpioa.pa0.into_function::<5>();
    let rx = p.gpioa.pa1.into_function::<5>();
    let mut pa5 = p.gpioa.pa5.into_pull_up_output();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &p.cmu);

    let mut uart0 = p.uart0.new_blocking(tx, rx, UartConfig::default(), &p.cmu);

    let dma_channels = p.dma.split(&p.cmu);

    let mut dma_ch0 = dma_channels.ch0;

    // Initialize MEM_SRC value.
    for i in 0..2000u32 {
        unsafe {
            MEM_SRC.0[i as usize] = i;
        }
    }

    let cfg = ChConfig::zeroed()
        .set_src_dev(0) // Sram for d13x series.
        .set_src_data_width(DataWidth::Bits32)
        .set_src_burst(BurstSize::Burst8)
        .enable_src_addr_inc()
        .set_snk_dev(0) // Sram for d13x series.
        .set_snk_data_width(DataWidth::Bits32)
        .set_snk_burst(BurstSize::Burst8)
        .enable_snk_addr_inc();

    let mode = ChMode::zeroed()
        .set_src_mode(HandshakeMode::Wait)
        .set_snk_mode(HandshakeMode::Wait);

    let len = 2000 * 4u32;

    let src_addr_u32 = unsafe { (core::ptr::addr_of!(MEM_SRC.0) as *const _) as u32 };
    let dst_addr_u32 = unsafe { (core::ptr::addr_of!(MEM_DST.0) as *const _) as u32 };

    let task = &mut DmaTask {
        cfg,
        src: src_addr_u32,
        dst: dst_addr_u32,
        len: len as u32,
        delay: DmaTask::DEFAULT_DELAY,
        p_next: DmaTask::TASK_END,
        mode,
        v_next: None,
    };

    writeln!(uart0, "=== MEM2MEM DMA Task ===").ok();
    writeln!(uart0, "Task addr: 0x{:08X}:", task as *const _ as u32).ok();
    writeln!(uart0, "Task src addr: 0x{:08X}", task.src).ok();
    writeln!(uart0, "Task dst addr: 0x{:08X}", task.dst).ok();
    writeln!(uart0, "src addr % 8 = {}", task.src % 8).ok();
    writeln!(uart0, "dst addr % 8 = {}", task.dst % 8).ok();
    writeln!(uart0, "task.len = {}", task.len).ok();

    unsafe {
        // Clean the task descriptor itself as well as the source buffer.
        dcache_clean_invalidate_range(task as *const _ as usize, core::mem::size_of::<DmaTask>());
        dcache_clean_invalidate_range(task.src as usize, task.len as usize);
    }

    writeln!(uart0, "Starting DMA transfer...").ok();

    dma_ch0.start(task);

    while !dma_ch0.is_all_finish_pending() {
        core::hint::spin_loop();
    }

    // Invalidate the destination cache line(s) so CPU reads fresh data.
    unsafe {
        // Use the range-based invalidate for the destination buffer.
        dcache_invalidate_range(task.dst as usize, task.len as usize);
    }

    writeln!(uart0, "Transfer completed!").ok();
    writeln!(uart0, "Verify data").ok();

    writeln!(uart0, "Destination data:").ok();
    let mut test_ok = true;
    for i in 0..2000 {
        let got = unsafe { MEM_DST.0[i] };
        let expect = unsafe { MEM_SRC.0[i] };
        if got != expect {
            test_ok = false;
            writeln!(uart0, "0x{:08X} (expected: 0x{:08X})", got, expect).ok();
            continue;
        }
        if i % 200 == 0 {
            writeln!(uart0, "0x{:08X} (expected: 0x{:08X})", got, expect).ok();
        }
    }

    writeln!(uart0, "Test {}", if test_ok { "PASSED" } else { "FAILED" }).ok();

    loop {
        pa5.toggle();
        delay.delay_ms(500);
    }
}
