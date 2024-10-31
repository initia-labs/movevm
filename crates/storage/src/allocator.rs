use std::{
    alloc::{GlobalAlloc, Layout, System}, cell::Cell
};

thread_local! {
    static SIZE: Cell<usize> = Cell::new(0);
}

struct SizeCounterAllocator;

unsafe impl GlobalAlloc for SizeCounterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SIZE.with(|size| size.set(size.get() + layout.size()));
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: SizeCounterAllocator = SizeCounterAllocator;

pub(crate) fn initialize_size() {
    SIZE.with(|size| size.set(0));
}

pub(crate) fn get_size() -> usize {
    SIZE.with(|size| size.get())
}