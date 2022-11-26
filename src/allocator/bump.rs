use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    //caller must ensure that the given memory range is unused and must not call this function twice
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_end;
        self.next = heap_start;
    }
}

//Locked<> is a Mutex wrapper
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(), //integer overflow
        };

        if bump.heap_end > alloc_end {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        } else {
            return null_mut(); //out of memory
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
