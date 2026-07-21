//! Pre-Boot Program runtime.
use core::arch::naked_asm;

/// Pre-Boot Program header structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PbpHeader {
    /// Magic number, should be ASCII "PBP ".
    pub magic: [u8; 4],
    /// PBP checksum.
    pub checksum: u32,
}

/// Static-linked Pre-Boot Program header.
#[unsafe(link_section = ".head.pbp")]
#[used]
pub static PBP_HEADER: PbpHeader = PbpHeader {
    magic: *b"PBP ",
    checksum: 0x0, // <- Real checksum filled by PBP tools.
};

#[unsafe(link_section = ".bss.uninit")]
static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];
const STACK_SIZE: usize = 1024; // 1 KiB

const MXSTATUS: u16 = 0x7c0;
const MEXSTATUS: u16 = 0x7e1;

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    naked_asm!(
        // 1. Disable interrupt
        "   csrw    mie, zero",

        // 2. Hart specific initialization
        // - Enable T-Head instruction sets (THEADISAEE) and
        // misaligned access (MM) in `mxstatus` register.
        // Cache (MHCR/MHINT) enabled later via enable_cache().
        // TODO SPUSHEN and SPSWAPEN in `mexstatus` once we have trap handler
        "   li      t2, 0x408000
            csrs    {mxstatus}, t2",

        // 3. Initialize float point unit
        "   li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",

        // 4. Clear `.bss` section
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",

        // 5. Prepare programming language stack
        "2: la      sp, {stack} + {stack_size}",

        // 6. Init vector table and enable caches before main
        "   call    {init_vector_table}",
        "   call    {enable_cache}",
        "   fence.i",

        // 7. Start Rust main function
        "   j       {main}",

        // 8. Platform halt (by loop-wfi) if main function returns
        // Set T-Head wfi behavior to deep-sleep, disable interrupt then
        // loop-wfi. Clears LPMD=0 and WFEEN=0 in `mexstatus`.
        "   li      t0, 0x1c
            csrc    {mexstatus}, t0
            csrci   mstatus, 0x8
        3:  wfi
            j       3b",

        stack_size       = const STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym pbp_main,
        init_vector_table =  sym _init_vector_table,
        enable_cache     =   sym _enable_cache,
        mxstatus         =   const MXSTATUS,
        mexstatus        =   const MEXSTATUS,
    )
}

unsafe extern "C" {
    unsafe fn pbp_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
    unsafe fn _init_vector_table();
    unsafe fn _enable_cache();
}
