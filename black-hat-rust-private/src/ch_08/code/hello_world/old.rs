#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const SYS_OPEN: u64 = 2;
const SYS_LSEEK: u64 = 8;
const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;
const O_RDWR: u64 = 2;
const SEEK_SET: u64 = 0;
const STDOUT: u64 = 1;

unsafe fn syscall1(scnum: u64, arg1: u64) -> u64 {
    let ret: u64;
    asm!(
        "syscall",
        in("rax") scnum,
        in("rdi") arg1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

unsafe fn syscall2(scnum: u64, arg1: u64, arg2: u64) -> u64 {
    let ret: u64;
    asm!(
        "syscall",
        in("rax") scnum,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

unsafe fn syscall3(scnum: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let ret: u64;
    asm!(
        "syscall",
        in("rax") scnum,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

// fn my_itoa(mut n: u32, dst: &mut [u8]) {
//     // No bounds checks. But a 10-element buffer is sufficient for all cases.
//     let mut i = dst.len() - 1;

//     loop {
//         dst[i] = (n % 10) as u8 + b'0';
//         n /= 10;
//         if n == 0 {
//             break;
//         }
//         i -= 1;
//     }
// }

#[no_mangle]
// #[link_section = ".text.prologue"]
fn _start() -> ! {
    // fn main() {
    let message = "hello world\n";

    unsafe {
        syscall3(
            SYS_WRITE,
            STDOUT,
            message.as_ptr() as u64,
            message.len() as u64,
        );

        syscall1(SYS_EXIT, 0)
    };

    loop {}
}
