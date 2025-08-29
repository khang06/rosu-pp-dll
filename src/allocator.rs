extern crate alloc;

use alloc::alloc::handle_alloc_error;
use core::alloc::GlobalAlloc;
use windows_sys::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HeapFree};

#[global_allocator]
static ALLOCATOR: WinAlloc = WinAlloc;

struct WinAlloc;

unsafe impl GlobalAlloc for WinAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let heap = unsafe { GetProcessHeap() };
        assert!(!heap.is_null(), "failed to get process heap");

        let ptr = unsafe { HeapAlloc(heap, 0, layout.size()) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        ptr as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let heap = unsafe { GetProcessHeap() };
        assert!(!heap.is_null(), "failed to get process heap");

        unsafe {
            HeapFree(heap, 0, ptr as _);
        }
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("allocation failed: {:?}", layout);
}
