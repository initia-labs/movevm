use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

use move_binary_format::errors::VMResult;

thread_local! {
    static METERING: Cell<bool> = const { Cell::new(false) };
    static SIZE: Cell<usize> = const { Cell::new(0) };
}

struct SizeCounterAllocator;

unsafe impl GlobalAlloc for SizeCounterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if METERING.with(|metering| metering.get()) {
            SIZE.with(|size| size.set(size.get() + layout.size()));
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: SizeCounterAllocator = SizeCounterAllocator;

#[inline]
fn start_metering() {
    debug_assert!(!METERING.with(|metering| metering.get()));
    SIZE.with(|size| size.set(0));
    METERING.with(|metering| metering.set(true));
}

#[inline]
fn finish_metering() -> usize {
    debug_assert!(METERING.with(|metering| metering.get()));
    METERING.with(|metering| metering.set(false));
    SIZE.with(|size| size.get())
}

/// Turns metering off when dropped, so a panic in the measured closure does not leave it on.
struct MeteringGuard;

impl Drop for MeteringGuard {
    fn drop(&mut self) {
        METERING.with(|metering| metering.set(false));
    }
}

#[inline]
pub(crate) fn get_size<T, O: FnOnce() -> VMResult<T>>(f: O) -> VMResult<(T, usize)> {
    start_metering();
    let _metering = MeteringGuard;
    let ret = f();
    let size = finish_metering();

    Ok((ret?, size + size_of::<T>()))
}

#[cfg(test)]
mod allocator_test {
    use rand::Rng;
    use std::thread;

    use super::*;

    #[test]
    fn test_get_size_turns_metering_off_after_a_panic() {
        let res = std::panic::catch_unwind(|| {
            get_size(|| -> VMResult<()> { panic!("measured closure panicked") })
        });
        assert!(res.is_err());
        assert!(!METERING.with(|metering| metering.get()));
    }

    #[test]
    fn test_get_size() {
        let num_thread = 100;
        for _ in 0..num_thread {
            let handle = thread::spawn(|| {
                let num_bytes = rand::thread_rng().gen_range(0..5120); // < 5KB
                let (_, size) = get_size(|| {
                    for _ in 0..num_bytes {
                        // allocate 1 byte
                        let _ = vec![0u8; 1];
                    }

                    Ok(())
                })
                .unwrap();

                assert_eq!(size, num_bytes);
            });

            handle.join().unwrap();
        }
    }

    #[test]
    fn test_get_size_after_error() {
        let failed: VMResult<((), usize)> = get_size(|| {
            Err(move_binary_format::errors::PartialVMError::new(
                move_core_types::vm_status::StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            )
            .finish(move_binary_format::errors::Location::Undefined))
        });
        assert!(failed.is_err());

        // A failed measurement must not leave metering on for the next one on this thread.
        let (_, size) = get_size(|| {
            let _ = vec![0u8; 1];
            Ok(())
        })
        .unwrap();
        assert_eq!(size, 1);
    }
}
