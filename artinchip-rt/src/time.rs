//! Async runtime clock: an `embassy-time` driver backed by the machine timer.

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use core::task::Waker;

#[cfg(feature = "d13x")]
use artinchip_hal::clint::ClintV1 as CoreTimer;
#[cfg(feature = "d21x")]
use artinchip_hal::clint::ClintV2 as CoreTimer;
use artinchip_hal::interrupt::typelevel::{self, Handler, Interrupt as _};
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

#[cfg(feature = "d13x")]
use crate::soc::d13x::{CLINT, MACHINE_TIMER_HZ};
#[cfg(feature = "d21x")]
use crate::soc::d21x::{CLINT, MACHINE_TIMER_HZ};

const _: () = assert!(
    embassy_time_driver::TICK_HZ == MACHINE_TIMER_HZ as u64,
    "embassy-time's tick rate must match the machine timer frequency"
);

static TIMER: MachineTimer = MachineTimer(CoreTimer::__new(CLINT::ptr()));

struct MachineTimer(CoreTimer);

unsafe impl Sync for MachineTimer {}

impl MachineTimer {
    #[inline]
    fn now(&self) -> u64 {
        self.0.mtime()
    }

    #[inline]
    fn set_alarm(&self, at: u64) -> bool {
        if at <= self.now() {
            return false;
        }
        self.0.set_mtimecmp(at);
        true
    }
}

struct TimeDriver {
    queue: Mutex<RefCell<Queue>>,
}

impl TimeDriver {
    /// Wake expired timers, arm `mtimecmp` for the next deadline, and return that tick.
    fn service(&self, cs: CriticalSection) -> u64 {
        let mut queue = self.queue.borrow(cs).borrow_mut();

        let mut next = queue.next_expiration(TIMER.now());
        // Re-check if a deadline elapsed while walking the queue before `mtimecmp` was armed.
        while !TIMER.set_alarm(next) {
            next = queue.next_expiration(TIMER.now());
        }
        next
    }
}

impl Driver for TimeDriver {
    fn now(&self) -> u64 {
        TIMER.now()
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            // `true` indicates `at` became the earliest deadline and `mtimecmp` needs updating.
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(TIMER.now());
                while !TIMER.set_alarm(next) {
                    next = queue.next_expiration(TIMER.now());
                }
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: TimeDriver = TimeDriver {
    queue: Mutex::new(RefCell::new(Queue::new())),
});

/// Handler for the core's machine timer interrupt (`CPU_TIMER`).
pub struct TimerInterrupt;

impl Handler<typelevel::CPU_TIMER> for TimerInterrupt {
    unsafe fn on_interrupt() {
        ALARM_SEEN.store(true, Ordering::Relaxed);
        critical_section::with(|cs| {
            let _ = DRIVER.service(cs);
        });
    }
}

/// Bind `CPU_TIMER` to [`TimerInterrupt`] and arm the machine timer as the runtime clock.
pub fn init() {
    critical_section::with(|_| {
        TIMER.0.set_mtimecmp(u64::MAX);
        typelevel::CPU_TIMER::clear_pending();
        unsafe {
            artinchip_hal::interrupt::register::<typelevel::CPU_TIMER, TimerInterrupt>();
        }
        typelevel::CPU_TIMER::enable();
    });
}

/// Wake expired timers, arm `mtimecmp` for the next deadline, and return that tick.
pub(crate) fn service() -> u64 {
    critical_section::with(|cs| DRIVER.service(cs))
}

/// Current machine timer tick count.
#[inline]
pub(crate) fn now() -> u64 {
    TIMER.now()
}

/// Probe and log machine-timer health once after the first executor poll.
pub(crate) fn report() {
    if REPORTED.swap(true, Ordering::Relaxed) {
        return;
    }

    check();
    // Restore `mtimecmp` from the timer queue after `check()` overwrote it.
    critical_section::with(|cs| {
        DRIVER.service(cs);
    });

    match CLOCK.load(Ordering::Relaxed) {
        CHECKED_OK => log::info!(
            "time: Machine timer at {MACHINE_TIMER_HZ} Hz, mtime={:#x}, alarm interrupt ok",
            TIMER.now()
        ),
        CHECKED_NO_ALARM => log::warn!(
            "time: Machine timer advances but its alarm interrupt never reached the core; \
             deadlines are met by polling the counter"
        ),
        _ => log::error!(
            "time: Machine timer does not advance (mtime={:#x}); nothing time-based can work",
            TIMER.now()
        ),
    }
}

const CHECKED_OK: u8 = 1;
const CHECKED_NO_ALARM: u8 = 2;
const CHECKED_NO_COUNTER: u8 = 3;

static CLOCK: AtomicU8 = AtomicU8::new(0);
static ALARM_SEEN: AtomicBool = AtomicBool::new(false);
static REPORTED: AtomicBool = AtomicBool::new(false);

fn check() {
    const PROBE_TICKS: u64 = MACHINE_TIMER_HZ as u64 / 10_000;
    const SPIN_LIMIT: u32 = 1_000_000;

    let start = TIMER.now();
    ALARM_SEEN.store(false, Ordering::Relaxed);
    TIMER.0.set_mtimecmp(start + PROBE_TICKS);

    let mut spins = 0u32;
    while TIMER.now() < start + 2 * PROBE_TICKS {
        spins += 1;
        if spins >= SPIN_LIMIT {
            CLOCK.store(CHECKED_NO_COUNTER, Ordering::Relaxed);
            return;
        }
        core::hint::spin_loop();
    }

    let state = if ALARM_SEEN.load(Ordering::Relaxed) {
        CHECKED_OK
    } else {
        CHECKED_NO_ALARM
    };
    CLOCK.store(state, Ordering::Relaxed);

    if state != CHECKED_OK {
        report_state();
    }
}

fn report_state() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    let (mtvec, mstatus, mie, mip) = {
        let mut mtvec: usize = 0;
        let mut mstatus: usize = 0;
        let mut mie: usize = 0;
        let mut mip: usize = 0;
        unsafe {
            core::arch::asm!("csrr {}, mtvec", out(reg) mtvec);
            core::arch::asm!("csrr {}, mstatus", out(reg) mstatus);
            core::arch::asm!("csrr {}, mie", out(reg) mie);
            core::arch::asm!("csrr {}, mip", out(reg) mip);
        }
        (mtvec, mstatus, mie, mip)
    };
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    let (mtvec, mstatus, mie, mip) = (0usize, 0usize, 0usize, 0usize);

    #[cfg(feature = "d13x")]
    {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        let mtvt: usize = unsafe {
            let mut mtvt: usize = 0;
            core::arch::asm!("csrr {}, 0x307", out(reg) mtvt);
            mtvt
        };
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        let mtvt: usize = 0;

        log::debug!(
            "time: No alarm; mtvec={mtvec:#010x} mtvt={mtvt:#010x} mstatus={mstatus:#010x} \
             mie={mie:#010x} mip={mip:#010x}"
        );

        let clic = unsafe { &*crate::soc::d13x::CLIC::ptr() };
        let cfg = clic.cfg.read();
        let info = clic.info.read();
        log::debug!(
            "time: Clic nlbits={} nmbits={} vector={} thresh={:#04x} ctlbits={} sources={}",
            cfg.nlbits(),
            cfg.nmbits(),
            cfg.is_vector_mode_enabled(),
            clic.mint_thresh.read().mint_thresh(),
            info.clicctlbits(),
            info.num_interrupt()
        );

        let irq = <typelevel::CPU_TIMER as typelevel::Interrupt>::IRQ as usize;
        let source = &clic.interrupts[irq];
        let attr = source.int_attr.read();
        let vector = if mtvt == 0 {
            0
        } else {
            unsafe { (mtvt as *const u32).add(irq).read_volatile() }
        };
        log::debug!(
            "time: Source {irq}: pending={} enabled={} shv={} trig={} mode={} ctl={:#04x} \
             vector={vector:#010x}",
            source.int_ip.read().is_pending(),
            source.int_ie.read().is_enabled(),
            attr.hardware_vector(),
            attr.trig(),
            attr.mode(),
            source.int_ctl.read(),
        );
    }

    #[cfg(feature = "d21x")]
    {
        log::debug!(
            "time: No alarm; mtvec={mtvec:#010x} mstatus={mstatus:#010x} mie={mie:#010x} \
             mip={mip:#010x}"
        );
        log::debug!("time: MTIE={} MTIP={}", (mie >> 7) & 1, (mip >> 7) & 1);
    }
}
