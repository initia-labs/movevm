use std::{
    alloc::{GlobalAlloc, Layout, System}, sync::Arc, thread::ThreadId
};

static mut SIZE_COUNTER: usize = 0;
static mut REQUEST_THREAD_ID: Option<ThreadId> = None;

struct SizeCounterAllocator;

unsafe impl GlobalAlloc for SizeCounterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        unsafe {
            if let Some(tid) = REQUEST_THREAD_ID {
                if tid == std::thread::current().id() {
                    SIZE_COUNTER += layout.size();
                }
            }
        }

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: SizeCounterAllocator = SizeCounterAllocator;

static SIZE_COUNTER_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
pub(crate) fn get_size_of<T: Clone>(t: Arc<T>) -> usize {
    let lock = SIZE_COUNTER_LOCK.lock().unwrap();

    let msize: usize;
    unsafe {
        REQUEST_THREAD_ID = Some(std::thread::current().id());
        let _ = (*t).clone();
        msize = SIZE_COUNTER + size_of::<T>();
        // println!("SIZE_COUNTER {} {}", SIZE_COUNTER, size_of::<T>());
        SIZE_COUNTER = 0;
        REQUEST_THREAD_ID = None;
    }

    drop(lock);

    msize
}
