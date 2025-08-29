#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::string::ToString;

mod allocator;
mod api;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use windows_sys::Win32::System::Threading::ExitProcess;
    use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;

    unsafe {
        MessageBoxA(
            core::ptr::null_mut(),
            info.to_string().as_ptr(),
            c"wtf".as_ptr() as *const _,
            0,
        );

        ExitProcess(101)
    }
}
