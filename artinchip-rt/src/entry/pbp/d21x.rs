//! D21x Pre-Boot Program startup assembly and entry points.

/// Emit D21x PBP header, version words, `.bss.uninit` state, and `_start`.
#[doc(hidden)]
#[macro_export]
macro_rules! __pbp_startup_d21x {
    () => {
        #[unsafe(link_section = ".head.pbp")]
        #[used]
        pub static PBP_HEADER: $crate::entry::pbp::PbpHeader = $crate::entry::pbp::PbpHeader::new();

        #[unsafe(link_section = ".head.pbp.version")]
        #[used]
        static PBP_HEADER_VERSION: [u32; 6] = [0x0001_0001; 6];

        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_PBP_STACK: $crate::sync::SingleCore<$crate::entry::pbp::PbpStack> =
            $crate::sync::SingleCore::new($crate::entry::pbp::PbpStack::new());

        #[unsafe(link_section = ".bss.uninit")]
        static __AIC_PBP_RETURN_CTX: $crate::sync::SingleCore<$crate::entry::pbp::ReturnContext> =
            $crate::sync::SingleCore::new($crate::entry::pbp::ReturnContext::new());

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".text.entry")]
        pub extern "C" fn _start() {
            const MXSTATUS: u16 = 0x7c0;
            const MHCR: u16 = 0x7c1;
            unsafe extern "C" {
                fn pbp_main(boot_param: u32, priv_addr: *const u8, priv_len: u32);
                fn _init_vector_table();
                fn _enable_cache();
                fn _flush_cache();
            }

            ::core::arch::naked_asm!(
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
                    sw      a2, {o_a2}(t0)
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
                "   csrw    mie, zero
                    csrw    mip, zero
                    csrci   mstatus, 0x8",
                "   li      t0, 0x408000
                    csrs    {mxstatus}, t0
                    li      t0, 0x4000
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
                "2: la      sp, {stack} + {stack_size}
                    call    {init_vector_table}
                    call    {enable_cache}
                    fence.i",
                "   la      t0, {ctx}
                    lw      a0, {o_a0}(t0)
                    lw      a1, {o_a1}(t0)
                    lw      a2, {o_a2}(t0)
                    call    {main}",
                "   la      t0, {ctx}
                    lw      t1, {o_a0}(t0)
                    andi    t1, t1, 0xf
                    li      t2, 7
                    bne     t1, t2, 3f",
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
                    lw      a0, {o_a0}(t0)
                    lw      a1, {o_a1}(t0)
                    lw      a2, {o_a2}(t0)
                    lw      ra, {o_ra}(t0)
                    lw      sp, {o_sp}(t0)
                    lw      gp, {o_gp}(t0)
                    lw      tp, {o_tp}(t0)
                    lw      t1, {o_mstatus}(t0)
                    csrw    mstatus, t1
                    ret",
                "3:  csrci   mstatus, 0x8
                4:  wfi
                    j       4b",
                stack = sym __AIC_PBP_STACK,
                stack_size = const $crate::entry::pbp::STACK_SIZE,
                ctx = sym __AIC_PBP_RETURN_CTX,
                main = sym pbp_main,
                init_vector_table = sym _init_vector_table,
                enable_cache = sym _enable_cache,
                flush_cache = sym _flush_cache,
                mhcr = const MHCR,
                mxstatus = const MXSTATUS,
                o_ra = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, ra),
                o_sp = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, sp),
                o_gp = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, gp),
                o_tp = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, tp),
                o_s0 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s0),
                o_s1 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s1),
                o_s2 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s2),
                o_s3 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s3),
                o_s4 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s4),
                o_s5 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s5),
                o_s6 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s6),
                o_s7 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s7),
                o_s8 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s8),
                o_s9 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s9),
                o_s10 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s10),
                o_s11 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, s11),
                o_a0 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, a0),
                o_a1 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, a1),
                o_a2 = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, a2),
                o_mstatus = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mstatus),
                o_mie = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mie),
                o_mip = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mip),
                o_mtvec = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mtvec),
                o_mhcr = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mhcr),
                o_mxstatus = const ::core::mem::offset_of!($crate::entry::pbp::ReturnContext, mxstatus),
            )
        }
    };
}
