#![no_std]
#![no_main]
#![feature(lang_items, asm, llvm_asm)]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let sys_message = "Hello world from no_std!\n";
    say_hello(sys_message);
    loop {}
}


#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "stdcall" {
    fn GetStdHandle(nStdHandle: i32) -> i32;
    fn WriteConsoleA(
        hConsoleOutput: i32,
        lpBuffer: *const u8,
        numberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *const core::ffi::c_void,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn say_hello(message: &str) {
    let msg_ptr = message.as_ptr();
    let len = message.len() as u32;

    let mut output: u32 = 0;
    let handle = unsafe { GetStdHandle(-11) };
    if handle == -1 {
        return;
    }
    unsafe {
        WriteConsoleA(handle, msg_ptr, len,
                      &mut output, core::ptr::null())
    };
}


#[cfg(all(not(target_os = "windows"), not(feature = "asm")))]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

#[cfg(all(not(target_os = "windows"), not(feature = "asm")))]
fn say_hello(message: &str) {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let _ = unsafe { write(1, msg_ptr, len) };
}


#[cfg(all(target_os = "macos", feature = "asm"))]
fn say_hello(message: &str) {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        llvm_asm!(
            "
        mov     $$0x2000004, %rax
        mov     $$1, %rdi
        mov     $0, %rsi
        mov     $1, %rdx
        syscall
    "
        :
        : "r"(msg_ptr), "r"(len)
        : "rax", "rdi", "rsi", "rdx"
        )
    };
}


#[cfg(all(target_os = "linux", feature = "asm"))]
#[inline(never)]
fn say_hello(message: &str) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        llvm_asm!("
        mov     $$1, %rax
        mov     $$1, %rdi
        mov     $0, %rsi
        mov     $1, %rdx
        syscall
    "
        :
        : "r"(msg_ptr), "r"(len)
        : "rax", "rdi", "rsi", "rdx"
        )
    }
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
