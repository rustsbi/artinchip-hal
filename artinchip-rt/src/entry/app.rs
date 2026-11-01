//! Application entry startup (`#[app_entry]`).

/// Application stack size (64 KiB in `.bss.uninit`).
///
/// Application code keeps large objects by value on the stack; the upgrade
/// protocol engine alone holds inline request/response buffers in the order of
/// 13 KiB. `.bss.uninit` is not part of the loaded image, so this only costs RAM.
pub const STACK_SIZE: usize = 64 * 1024;

/// Stack backing storage; only its address is consumed by the assembly entry.
#[doc(hidden)]
#[repr(align(16))]
pub struct AppStack(pub [u8; STACK_SIZE]);

impl AppStack {
    pub const fn new() -> Self {
        Self([0; STACK_SIZE])
    }
}

impl Default for AppStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit D21x (C906) application `_start` and `.bss.uninit` stack.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_startup_d21x {
    () => {
        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_APP_STACK: $crate::sync::SingleCore<$crate::entry::app::AppStack> =
            $crate::sync::SingleCore::new($crate::entry::app::AppStack::new());

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".text.entry")]
        pub extern "C" fn _start() {
            const MXSTATUS: u16 = 0x7c0;
            unsafe extern "C" {
                fn app_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
                fn _init_vector_table();
                fn _enable_cache();
            }

            ::core::arch::naked_asm!(
                // 1. Disable interrupts and park the BootROM arguments.
                "   csrw    mie, zero
                    csrw    mip, zero
                    csrci   mstatus, 0x8
                    mv      s0, a0
                    mv      s1, a1
                    mv      s2, a2",
                // 2. Enable T-Head instruction sets and misaligned access.
                "   li      t0, 0x408000
                    csrs    {mxstatus}, t0",
                // 3. Initialize the floating point unit.
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
                // 5. Prepare the Rust stack.
                "2: la      sp, {stack} + {stack_size}",
                // 6. Install the vector table and enable caches.
                "   call    {init_vector_table}
                    call    {enable_cache}
                    fence.i",
                // 7. Restore the BootROM arguments and enter Rust.
                "   mv      a0, s0
                    mv      a1, s1
                    mv      a2, s2
                    j       {main}",
                // 8. Halt if the application returns.
                "   csrci   mstatus, 0x8
                3:  wfi
                    j       3b",
                stack_size = const $crate::entry::app::STACK_SIZE,
                stack = sym __AIC_APP_STACK,
                main = sym app_main,
                init_vector_table = sym _init_vector_table,
                enable_cache = sym _enable_cache,
                mxstatus = const MXSTATUS,
            )
        }
    };
}

/// Emit D13x/E907 application `_start` and `.bss.uninit` stack.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_startup_d13x {
    () => {
        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_APP_STACK: $crate::sync::SingleCore<$crate::entry::app::AppStack> =
            $crate::sync::SingleCore::new($crate::entry::app::AppStack::new());

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".text.entry")]
        pub extern "C" fn _start() {
            const MXSTATUS: u16 = 0x7c0;
            const MEXSTATUS: u16 = 0x7e1;
            unsafe extern "C" {
                fn app_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
                fn _init_vector_table();
                fn _enable_cache();
            }

            ::core::arch::naked_asm!(
                // 1. Disable interrupts and park the BootROM arguments.
                "   csrw    mie, zero
                    mv      s0, a0
                    mv      s1, a1
                    mv      s2, a2",
                // 2. Enable T-Head instruction sets and misaligned access.
                "   li      t2, 0x408000
                    csrs    {mxstatus}, t2",
                // 3. Initialize the floating point unit.
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
                // 5. Prepare the Rust stack.
                "2: la      sp, {stack} + {stack_size}",
                // 6. Install the vector table and enable caches.
                "   call    {init_vector_table}
                    call    {enable_cache}
                    fence.i",
                // 7. Restore the BootROM arguments and enter Rust.
                "   mv      a0, s0
                    mv      a1, s1
                    mv      a2, s2
                    j       {main}",
                // 8. Halt if the application returns, clearing MEXSTATUS first.
                "   li      t0, 0x1c
                    csrc    {mexstatus}, t0
                    csrci   mstatus, 0x8
                3:  wfi
                    j       3b",
                stack_size = const $crate::entry::app::STACK_SIZE,
                stack = sym __AIC_APP_STACK,
                main = sym app_main,
                init_vector_table = sym _init_vector_table,
                enable_cache = sym _enable_cache,
                mxstatus = const MXSTATUS,
                mexstatus = const MEXSTATUS,
            )
        }
    };
}
