//! Bootloader entry startup (`#[boot_entry]`).

/// Bootloader stack size (64 KiB in `.bss.uninit`).
pub const BOOT_STACK_SIZE: usize = 64 * 1024;

/// 16-byte aligned bootloader stack storage.
#[doc(hidden)]
#[repr(align(16))]
pub struct BootStack(pub [u8; BOOT_STACK_SIZE]);

impl BootStack {
    pub const fn new() -> Self {
        Self([0; BOOT_STACK_SIZE])
    }
}

impl Default for BootStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit D21x (C906) application `_start` and `.bss.uninit` stack.
#[doc(hidden)]
#[macro_export]
macro_rules! __boot_startup_d21x {
    () => {
        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_BOOT_STACK: $crate::sync::SingleCore<$crate::entry::bootloader::BootStack> =
            $crate::sync::SingleCore::new($crate::entry::bootloader::BootStack::new());

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
                "   csrw    mie, zero
                    csrw    mip, zero
                    csrci   mstatus, 0x8
                    mv      s0, a0
                    mv      s1, a1
                    mv      s2, a2",
                "   li      t0, 0x408000
                    csrs    {mxstatus}, t0",
                "   li      t0, 0x4000
                    li      t1, 0x2000
                    csrc    mstatus, t0
                    csrs    mstatus, t1
                    csrw    fcsr, zero",
                "   la      t0, sbss
                    la      t1, ebss
                1:  bgeu    t0, t1, 2f
                    sw      zero, 0(t0)
                    addi    t0, t0, 4
                    j       1b",
                "2: la      sp, {stack} + {stack_size}",
                "   call    {init_vector_table}
                    call    {enable_cache}
                    fence.i",
                "   mv      a0, s0
                    mv      a1, s1
                    mv      a2, s2
                    j       {main}",
                "   csrci   mstatus, 0x8
                3:  wfi
                    j       3b",
                stack_size = const $crate::entry::bootloader::BOOT_STACK_SIZE,
                stack = sym __AIC_BOOT_STACK,
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
macro_rules! __boot_startup_d13x {
    () => {
        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_BOOT_STACK: $crate::sync::SingleCore<$crate::entry::bootloader::BootStack> =
            $crate::sync::SingleCore::new($crate::entry::bootloader::BootStack::new());

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
                "   csrw    mie, zero
                    mv      s0, a0
                    mv      s1, a1
                    mv      s2, a2",
                "   li      t2, 0x408000
                    csrs    {mxstatus}, t2",
                "   li      t0, 0x4000
                    li      t1, 0x2000
                    csrc    mstatus, t0
                    csrs    mstatus, t1
                    csrw    fcsr, zero",
                "   la      t0, sbss
                    la      t1, ebss
                1:  bgeu    t0, t1, 2f
                    sw      zero, 0(t0)
                    addi    t0, t0, 4
                    j       1b",
                "2: la      sp, {stack} + {stack_size}",
                "   call    {init_vector_table}
                    call    {enable_cache}
                    fence.i",
                "   mv      a0, s0
                    mv      a1, s1
                    mv      a2, s2
                    j       {main}",
                "   li      t0, 0x1c
                    csrc    {mexstatus}, t0
                    csrci   mstatus, 0x8
                3:  wfi
                    j       3b",
                stack_size = const $crate::entry::bootloader::BOOT_STACK_SIZE,
                stack = sym __AIC_BOOT_STACK,
                main = sym app_main,
                init_vector_table = sym _init_vector_table,
                enable_cache = sym _enable_cache,
                mxstatus = const MXSTATUS,
                mexstatus = const MEXSTATUS,
            )
        }
    };
}
