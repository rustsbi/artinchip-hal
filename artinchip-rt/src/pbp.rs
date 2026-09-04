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

/// D21x PBP header version words.
///
/// The D21x BROM expects the PBP code entry at file offset `0x20`:
///
/// ```text
/// [0x00..0x04]  "PBP " magic
/// [0x04..0x08]  checksum (filled by aicfwc)
/// [0x08..0x20]  reserved version words (0x00010001 x 6)
/// [0x20..]      executable entry
/// ```
///
/// D13x/D12x/G73x use an 8-byte PBP header only, so this section is
/// emitted for D21x exclusively.
#[cfg(feature = "d21x")]
#[unsafe(link_section = ".head.pbp.version")]
#[used]
static PBP_HEADER_VERSION: [u32; 6] = [0x0001_0001; 6];

#[cfg(feature = "d21x")]
const STACK_SIZE: usize = 4096; // 4 KiB
#[cfg(not(feature = "d21x"))]
const STACK_SIZE: usize = 2048; // 2 KiB

/// Stack backing storage. Only the buffer's address is consumed by the
/// assembly entry points through the `sym STACK` operand, so the wrapped field
/// is never read from Rust code and is flagged as dead code.
#[repr(align(16))]
#[allow(dead_code)]
struct Stack([u8; STACK_SIZE]);

#[unsafe(link_section = ".bss.uninit")]
static mut STACK: Stack = Stack([0u8; STACK_SIZE]);

const MXSTATUS: u16 = 0x7c0;

/// D21x compatible PBP entry point.
#[cfg(feature = "d21x")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    naked_asm!(
        // 1. Disable interrupt. D21x PBP has no vector table yet, so do not
        // inherit any interrupt state from BROM.
        "   csrw    mie, zero
            csrw    mip, zero
            csrci   mstatus, 0x8
            mv      s0, a0
            mv      s1, a1",

        // 2. Hart specific initialization.
        // - Enable T-Head instruction sets (THEADISAEE) and
        // misaligned access (MM) in `mxstatus` register.
        "   li      t0, 0x408000
            csrs    {mxstatus}, t0",

        // 3. Initialize floating point unit.
        // C906 implements the standard mstatus.FS field and fcsr. Start with
        // FS=01 (initial/clean), which permits FPU instructions without
        // claiming a dirty context before Rust uses it.
        "   li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",

        // 4. Clear `.bss` section. `.bss.uninit` contains the stack and is
        // excluded, matching the D13x startup layout.
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",

        // 5. Prepare programming language stack.
        "2: la      sp, {stack} + {stack_size}",

        // 6. C906 uses ordinary mtvec direct mode for PBP traps.  Install the
        // local trap entry before touching runtime peripherals, then enable
        // caches before entering Rust.
        "   call    {init_vector_table}
            call    {enable_cache}
            fence.i",

        // 7. Start Rust main function. This is a terminal PBP and must not
        // return to BROM's downloader.
        "   mv      a0, s0
            mv      a1, s1
            mv      a2, zero
            j       {main}",

        // 8. Platform halt if main function returns. D21x has no MEXSTATUS
        // register in this early PBP ABI, so only disable MSTATUS.MIE.
        "   csrci   mstatus, 0x8
        3:  wfi
            j       3b",

        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym pbp_main,
        init_vector_table = sym _init_vector_table,
        enable_cache = sym _enable_cache,
        mxstatus   =   const MXSTATUS,
    )
}

/// D13x compatible PBP entry point.
#[cfg(not(feature = "d21x"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    const MEXSTATUS: u16 = 0x7e1;

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
