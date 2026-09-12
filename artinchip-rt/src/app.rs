//! Application image runtime.
//!
//! This is the loader-style counterpart of [`crate::pbp`]: the image is a normal
//! AIC `loader` resource (256-byte AIC header + payload at `load_address + 0x100`)
//! that the boot chain copies and jumps to. The user entry point is provided by
//! [`artinchip_rt_macros::app_entry`] and exported as `app_main`.

use core::arch::naked_asm;

/// Application stack size.
///
/// This has to be far larger than what a PBP needs: application code keeps
/// large objects *by value* on the stack, e.g. the upgrade protocol engine in
/// `examples/app/app-bootloader`, whose inline request/response buffers alone are
/// about 13 KiB. With a 2 KiB stack that overflows the moment the engine is
/// constructed — silently, since there is no stack guard.
///
/// The stack lives in `.bss.uninit`, which is not part of the loaded image, so
/// this only costs RAM.
const STACK_SIZE: usize = 64 * 1024; // 64 KiB

/// Stack backing storage. Only the buffer's address is consumed by the assembly
/// entry points through the `sym STACK` operand, so the wrapped field is never
/// read from Rust code and is flagged as dead code.
#[repr(align(16))]
#[allow(dead_code)]
struct Stack([u8; STACK_SIZE]);

#[unsafe(link_section = ".bss.uninit")]
static mut STACK: Stack = Stack([0u8; STACK_SIZE]);

const MXSTATUS: u16 = 0x7c0;

/// D21x (C906) application entry point.
#[cfg(feature = "d21x")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    naked_asm!(
        // 1. Disable interrupts and park the BootROM arguments. This entry has no
        // vector table of its own, so no interrupt state is inherited.
        "   csrw    mie, zero
            csrw    mip, zero
            csrci   mstatus, 0x8
            mv      s0, a0
            mv      s1, a1",

        // 2. Hart specific initialization: enable T-Head instruction sets
        // (THEADISAEE) and misaligned access (MM) in `mxstatus`.
        "   li      t0, 0x408000
            csrs    {mxstatus}, t0",

        // 3. Initialize the floating point unit (mstatus.FS = initial, fcsr = 0).
        "   li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",

        // 4. Clear `.bss`; `.bss.uninit` holds the stack and is excluded.
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",

        // 5. Prepare the programming language stack.
        "2: la      sp, {stack} + {stack_size}",

        // 6. Install the local vector table and enable caches before Rust.
        "   call    {init_vector_table}
            call    {enable_cache}
            fence.i",

        // 7. Restore the BootROM arguments and enter Rust. The third argument is
        // not part of the app ABI, so `private_data` stays empty.
        "   mv      a0, s0
            mv      a1, s1
            mv      a2, zero
            j       {main}",

        // 8. Platform halt if the app returns.
        "   csrci   mstatus, 0x8
        3:  wfi
            j       3b",

        stack_size = const STACK_SIZE,
        stack = sym STACK,
        main = sym app_main,
        init_vector_table = sym _init_vector_table,
        enable_cache = sym _enable_cache,
        mxstatus = const MXSTATUS,
    )
}

/// D13x (E907) application entry point.
#[cfg(not(feature = "d21x"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() {
    const MEXSTATUS: u16 = 0x7e1;

    naked_asm!(
        // 1. Disable interrupts.
        "   csrw    mie, zero",

        // 2. Hart specific initialization: enable T-Head instruction sets
        // (THEADISAEE) and misaligned access (MM) in `mxstatus`.
        "   li      t2, 0x408000
            csrs    {mxstatus}, t2",

        // 3. Initialize the floating point unit (mstatus.FS = initial, fcsr = 0).
        "   li      t0, 0x4000
            li      t1, 0x2000
            csrc    mstatus, t0
            csrs    mstatus, t1
            csrw    fcsr, zero",

        // 4. Clear `.bss`; `.bss.uninit` holds the stack and is excluded.
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sw      zero, 0(t0)
            addi    t0, t0, 4
            j       1b",

        // 5. Prepare the programming language stack.
        "2: la      sp, {stack} + {stack_size}",

        // 6. Install the local vector table and enable caches before Rust.
        "   call    {init_vector_table}
            call    {enable_cache}
            fence.i",

        // 7. Enter Rust with the BootROM app arguments still in `a0`/`a1`.
        "   j       {main}",

        // 8. Platform halt if the app returns, clearing MEXSTATUS first.
        "   li      t0, 0x1c
            csrc    {mexstatus}, t0
            csrci   mstatus, 0x8
        3:  wfi
            j       3b",

        stack_size = const STACK_SIZE,
        stack = sym STACK,
        main = sym app_main,
        init_vector_table = sym _init_vector_table,
        enable_cache = sym _enable_cache,
        mxstatus = const MXSTATUS,
        mexstatus = const MEXSTATUS,
    )
}

unsafe extern "C" {
    unsafe fn app_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
    unsafe fn _init_vector_table();
    unsafe fn _enable_cache();
}
