use std::mem;

#[link_section = ".text"]
static mut EXECUTABLE_SHELLCODE: [u8; 42_000] = [0x90; 42_000];

fn main() {
    #[link_section = ".text"]
    let shellcode = include_bytes!("../../shellcode.bin");
    let shellcode_len = shellcode.len();
    println!("shellcode {:x?}", &shellcode);

    unsafe {
        println!("before {:x?}", &EXECUTABLE_SHELLCODE[..shellcode_len]);
        // segfault because .text is not writable
        EXECUTABLE_SHELLCODE[..shellcode_len].copy_from_slice(shellcode);
        println!("executing {:x?}", &EXECUTABLE_SHELLCODE[..shellcode_len]);
    }

    let exec_shellcode: extern "C" fn() -> ! =
        unsafe { mem::transmute(&EXECUTABLE_SHELLCODE as *const _ as *const ()) };
    exec_shellcode();
}