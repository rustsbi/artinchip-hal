#![no_std]
#![no_main]

use artinchip_hal::dma::*;
use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[repr(C, align(8))]
struct MemBuf([u32; 2000]);

static mut MEM_SRC: MemBuf = MemBuf([0u32; 2000]);
static mut MEM_DST: MemBuf = MemBuf([0xDEAD_BEEFu32; 2000]);

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let mut pa5 = p.gpioa.pa5.into_pull_up_output();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let _uart0 =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();

    let dma_channels = p.dma.split(&mut p.cmu);

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

    info!("=== MEM2MEM DMA Task ===");
    info!("Task addr: 0x{:08X}:", task as *const _ as u32);
    info!("Task src addr: 0x{:08X}", task.src);
    info!("Task dst addr: 0x{:08X}", task.dst);
    info!("src addr % 8 = {}", task.src % 8);
    info!("dst addr % 8 = {}", task.dst % 8);
    info!("task.len = {}", task.len);

    unsafe {
        // Clean the task descriptor itself as well as the source buffer.
        dcache_clean_invalidate_range(task as *const _ as usize, core::mem::size_of::<DmaTask>());
        dcache_clean_invalidate_range(task.src as usize, task.len as usize);
    }

    info!("Starting DMA transfer...");

    dma_ch0.start(task);

    while !dma_ch0.is_all_finish_pending() {
        core::hint::spin_loop();
    }

    // Invalidate the destination cache line(s) so CPU reads fresh data.
    unsafe {
        // Use the range-based invalidate for the destination buffer.
        dcache_invalidate_range(task.dst as usize, task.len as usize);
    }

    info!("Transfer completed!");
    info!("Verify data");

    info!("Destination data:");
    let mut test_ok = true;
    for i in 0..2000 {
        let got = unsafe { MEM_DST.0[i] };
        let expect = unsafe { MEM_SRC.0[i] };
        if got != expect {
            test_ok = false;
            info!("0x{:08X} (expected: 0x{:08X})", got, expect);
            continue;
        }
        if i % 200 == 0 {
            info!("0x{:08X} (expected: 0x{:08X})", got, expect);
        }
    }

    info!("Test {}", if test_ok { "PASSED" } else { "FAILED" });

    loop {
        pa5.toggle();
        delay.delay_ms(500);
    }
}
