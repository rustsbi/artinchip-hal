//! Application image runtime (`#[app_entry]`).
//!
//! Supports both:
//! - asynchronous applications (`#[app_entry] async fn app_main()`), driven by
//!   the built-in Embassy executor with `CPU_TIMER` bound automatically, and
//! - synchronous loader/boot images (`#[app_entry] fn app_main(boot_param, private_data)`).

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
use core::mem::MaybeUninit;
#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
use embassy_executor::{SpawnToken, Spawner, raw};

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
use crate::sync::SingleCore;

pub use crate::entry::app::{AppStack, STACK_SIZE};

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
static RUNNING: AtomicBool = AtomicBool::new(false);

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
static EXECUTOR: SingleCore<MaybeUninit<raw::Executor>> = SingleCore::new(MaybeUninit::uninit());

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
static PENDER: AtomicBool = AtomicBool::new(false);

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
static SPAWNER: SingleCore<Option<Spawner>> = SingleCore::new(None);

/// Start the built-in Embassy executor, bind `CPU_TIMER`, and poll `token` forever.
#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
pub fn run_task<S>(token: SpawnToken<S>) -> ! {
    assert!(
        !RUNNING.swap(true, Ordering::AcqRel),
        "artinchip-rt executor is already running"
    );

    crate::time::init();

    let executor: &'static raw::Executor = unsafe {
        let slot = &mut *EXECUTOR.as_ptr();
        slot.write(raw::Executor::new(core::ptr::null_mut()))
    };

    publish(executor.spawner());
    executor.spawner().spawn(token);

    loop {
        crate::time::service();

        PENDER.store(false, Ordering::Relaxed);
        unsafe { executor.poll() };

        crate::time::report();

        let next = crate::time::service();
        loop {
            if PENDER.load(Ordering::Relaxed) || next == u64::MAX || crate::time::now() >= next {
                break;
            }
            core::hint::spin_loop();
        }
    }
}

/// Spawn a task on the application's built-in Embassy executor.
///
/// # Panics
/// Panics if called before [`run_task`] has started the executor.
#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
pub fn spawn<S>(token: SpawnToken<S>) {
    critical_section::with(|_| {
        let spawner = unsafe { *SPAWNER.as_ptr() }.expect("the async runtime is not running");
        spawner.spawn(token);
    });
}

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
fn publish(spawner: Spawner) {
    critical_section::with(|_| {
        unsafe { *SPAWNER.as_ptr() = Some(spawner) };
    });
}

#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
#[unsafe(export_name = "__pender")]
fn pender(_context: *mut ()) {
    PENDER.store(true, Ordering::Relaxed);
}
