//! Pre-Boot Program runtime.
#![cfg(not(feature = "app"))]
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
///
/// The only D21x entry: the return-capable trampoline is E907-specific (the C906
/// ABI differs), so the C906 variant stays terminal and must not return to the
/// BROM's downloader.
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

// The return trampoline is E907-specific; the C906 ABI differs.

/// Caller state parked by the D13x entry.
///
/// The D13x PBP is a *service* call: the caller (BootROM or host tool) resumes at
/// `ra` with its own `sp`/callee-saved registers once the image `ret`s, so all of
/// that plus the machine CSRs this image touches is saved before any of it is
/// clobbered. The block lives in `.bss.uninit`, which the `.bss` clear loop
/// (`sbss..ebss`) does not cover.
#[cfg(not(feature = "d21x"))]
#[allow(dead_code)]
#[repr(C, align(16))]
struct ReturnContext {
    ra: u32,
    sp: u32,
    gp: u32,
    tp: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    a0: u32,
    a1: u32,
    mstatus: u32,
    mie: u32,
    mip: u32,
    mtvec: u32,
    mhcr: u32,
    mxstatus: u32,
}

#[cfg(not(feature = "d21x"))]
#[allow(dead_code)]
#[unsafe(link_section = ".bss.uninit")]
static mut RETURN_CONTEXT: ReturnContext = ReturnContext {
    ra: 0,
    sp: 0,
    gp: 0,
    tp: 0,
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
    s4: 0,
    s5: 0,
    s6: 0,
    s7: 0,
    s8: 0,
    s9: 0,
    s10: 0,
    s11: 0,
    a0: 0,
    a1: 0,
    mstatus: 0,
    mie: 0,
    mip: 0,
    mtvec: 0,
    mhcr: 0,
    mxstatus: 0,
};

/// T-Head cache control register (`mhcr`).
#[cfg(not(feature = "d21x"))]
const MHCR: u16 = 0x7c1;

/// D13x PBP entry point.
///
/// The caller's context is parked first and restored afterwards, so the image can
/// either run to completion and never return (`pbp_main` looping or jumping) or
/// `return` as a *service* call - the `image.updater.psram` role - and give the
/// machine back as it was found. The vector table and caches are brought up for
/// `pbp_main` as the old terminal entry did, and both are restored before `ret`
/// (`mtvec`/`mhcr` are part of the parked context; the caches are flushed first).
#[cfg(not(feature = "d21x"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    naked_asm!(
        // 1. Park the caller's context before touching a single register it
        // expects to survive. `t0`/`t1` are the only scratch used here.
        "   la      t0, {ctx}
            sw      ra, {o_ra}(t0)
            sw      sp, {o_sp}(t0)
            sw      gp, {o_gp}(t0)
            sw      tp, {o_tp}(t0)
            sw      s0, {o_s0}(t0)
            sw      s1, {o_s1}(t0)
            sw      s2, {o_s2}(t0)
            sw      s3, {o_s3}(t0)
            sw      s4, {o_s4}(t0)
            sw      s5, {o_s5}(t0)
            sw      s6, {o_s6}(t0)
            sw      s7, {o_s7}(t0)
            sw      s8, {o_s8}(t0)
            sw      s9, {o_s9}(t0)
            sw      s10, {o_s10}(t0)
            sw      s11, {o_s11}(t0)
            sw      a0, {o_a0}(t0)
            sw      a1, {o_a1}(t0)
            csrr    t1, mstatus
            sw      t1, {o_mstatus}(t0)
            csrr    t1, mie
            sw      t1, {o_mie}(t0)
            csrr    t1, mip
            sw      t1, {o_mip}(t0)
            csrr    t1, mtvec
            sw      t1, {o_mtvec}(t0)
            csrr    t1, {mhcr}
            sw      t1, {o_mhcr}(t0)
            csrr    t1, {mxstatus}
            sw      t1, {o_mxstatus}(t0)",

        // 2. Mask interrupts while we run. `mtvec`/CLIC are left as the caller
        // had them: installing this image's vector table would leave a stale
        // pointer behind the moment we `ret`.
        "   csrw    mie, zero
            csrw    mip, zero
            csrci   mstatus, 0x8",

        // 3. Hart specific initialization, as in the terminal entry.
        "   li      t2, 0x408000
            csrs    {mxstatus}, t2
            li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",

        // 4. Clear `.bss`; the context block lives in `.bss.uninit`, outside
        // the `sbss..ebss` range, and survives.
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",

        // 5. Own stack, then the vector table and caches - the same bring-up the
        // old terminal entry did. Both are restored in step 7, because `mtvec` and
        // `mhcr` are part of the parked context.
        "2: la      sp, {stack} + {stack_size}
            call    {init_vector_table}
            call    {enable_cache}
            fence.i",

        // 6. PBP arguments, then the program itself. `a2` is not part of the PBP
        // ABI, so `private_data` stays empty.
        "   la      t0, {ctx}
            lw      a0, {o_a0}(t0)
            lw      a1, {o_a1}(t0)
            mv      a2, zero
            call    {main}",

        // 7. Clean + invalidate the caches this run may have dirtied, then put the
        // caller's machine state back and return. `mstatus` goes last, so no
        // interrupt can be taken with only half of it restored.
        "   call    {flush_cache}
            fence.i
            la      t0, {ctx}
            lw      t1, {o_mtvec}(t0)
            csrw    mtvec, t1
            lw      t1, {o_mie}(t0)
            csrw    mie, t1
            lw      t1, {o_mip}(t0)
            csrw    mip, t1
            lw      t1, {o_mxstatus}(t0)
            csrw    {mxstatus}, t1
            lw      t1, {o_mhcr}(t0)
            csrw    {mhcr}, t1
            lw      s0, {o_s0}(t0)
            lw      s1, {o_s1}(t0)
            lw      s2, {o_s2}(t0)
            lw      s3, {o_s3}(t0)
            lw      s4, {o_s4}(t0)
            lw      s5, {o_s5}(t0)
            lw      s6, {o_s6}(t0)
            lw      s7, {o_s7}(t0)
            lw      s8, {o_s8}(t0)
            lw      s9, {o_s9}(t0)
            lw      s10, {o_s10}(t0)
            lw      s11, {o_s11}(t0)
            lw      ra, {o_ra}(t0)
            lw      sp, {o_sp}(t0)
            lw      gp, {o_gp}(t0)
            lw      tp, {o_tp}(t0)
            lw      t1, {o_mstatus}(t0)
            csrw    mstatus, t1
            ret",

        stack = sym STACK,
        stack_size = const STACK_SIZE,
        ctx = sym RETURN_CONTEXT,
        main = sym pbp_main,
        init_vector_table = sym _init_vector_table,
        enable_cache = sym _enable_cache,
        flush_cache = sym _flush_cache,
        mhcr = const MHCR,
        mxstatus = const MXSTATUS,
        o_ra = const core::mem::offset_of!(ReturnContext, ra),
        o_sp = const core::mem::offset_of!(ReturnContext, sp),
        o_gp = const core::mem::offset_of!(ReturnContext, gp),
        o_tp = const core::mem::offset_of!(ReturnContext, tp),
        o_s0 = const core::mem::offset_of!(ReturnContext, s0),
        o_s1 = const core::mem::offset_of!(ReturnContext, s1),
        o_s2 = const core::mem::offset_of!(ReturnContext, s2),
        o_s3 = const core::mem::offset_of!(ReturnContext, s3),
        o_s4 = const core::mem::offset_of!(ReturnContext, s4),
        o_s5 = const core::mem::offset_of!(ReturnContext, s5),
        o_s6 = const core::mem::offset_of!(ReturnContext, s6),
        o_s7 = const core::mem::offset_of!(ReturnContext, s7),
        o_s8 = const core::mem::offset_of!(ReturnContext, s8),
        o_s9 = const core::mem::offset_of!(ReturnContext, s9),
        o_s10 = const core::mem::offset_of!(ReturnContext, s10),
        o_s11 = const core::mem::offset_of!(ReturnContext, s11),
        o_a0 = const core::mem::offset_of!(ReturnContext, a0),
        o_a1 = const core::mem::offset_of!(ReturnContext, a1),
        o_mstatus = const core::mem::offset_of!(ReturnContext, mstatus),
        o_mie = const core::mem::offset_of!(ReturnContext, mie),
        o_mip = const core::mem::offset_of!(ReturnContext, mip),
        o_mtvec = const core::mem::offset_of!(ReturnContext, mtvec),
        o_mhcr = const core::mem::offset_of!(ReturnContext, mhcr),
        o_mxstatus = const core::mem::offset_of!(ReturnContext, mxstatus),
    )
}

unsafe extern "C" {
    unsafe fn pbp_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
    unsafe fn _init_vector_table();
    unsafe fn _enable_cache();
    unsafe fn _flush_cache();
}
