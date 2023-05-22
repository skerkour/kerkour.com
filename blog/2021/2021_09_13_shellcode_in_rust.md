+++
date = 2021-09-13T13:00:00Z
title = "How to Write and Compile a Shellcode in Rust"
type = "post"
tags = ["hacking", "rust", "programming", "tutorial", "security"]
authors = ["Sylvain Kerkour"]
url = "/shellcode-in-rust"


[extra]
lang = "en"

comment ="""
"""
+++

A few months ago, we saw [how to execute a shellcode from memory in Rust](https://kerkour.com/rust-execute-from-memory/). What if we could write the actual shellcode in Rust?

Writing shellcodes is usually done directly in assembly. It gives you absolute control over what you are crafting, however, it comes with many, **many** drawbacks:
- It requires a lot of deep knowledge that is not transferable
- Shellcodes are not portable across different architectures
- Assembly is like regexps: (barely) easy to write, impossible to read
- Assembly code is a nightmare to compose and reuse
- It's extremely easy to introduce bugs that are hard to debug
- It's a nightmare to maintain over time and across teams of many developers

<!-- TODO: trouver d'autres drawbacks -->

What if instead, we could write our shellcodes in a language that is high-level and thanks to a highly advanced compiler, gives us precise, low-level control. A language that would make our shellcodes portables across architectures and easy to reuse.

Sounds too good to be true?

Without further ado, here is how to write shellcodes in Rust, so you will be able to judge by yourself.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Here is the assembly equivalent of the "Hello world" shellcode that we are about to craft in Rust:
```assembly
_start:
    jmp short string

code:
    pop rsi
    xor rax, rax
    mov al, 1
    mov rdi, rax
    mov rdx, rdi
    add rdx, 12
    syscall

    xor rax, rax
    add rax, 60
    xor rdi, rdi
    syscall

string:
    call code
    db  'hello world',0x0A
```


## Rust shellcode

First, we need to configure the linker to produce a bloat-free binary:

**shellcode.ld**
```ld
ENTRY(_start);

SECTIONS
{
	. = ALIGN(16);
	.text :
	{
		*(.text.prologue)
		*(.text)
		*(.rodata)
	}
	.data :
	{
		*(.data)
	}

	/DISCARD/ :
	{
		*(.interp)
		*(.comment)
		*(.debug_frame)
	}
}
```

And tell `cargo` to use this file:

**shellcode/.cargo/config.toml**
```toml
[build]
rustflags = ["-C", "link-arg=-nostdlib", "-C", "link-arg=-static", "-C", "link-arg=-Wl,-T./shellcode.ld,--build-id=none", "-C", "relocation-model=pic"]
```

Then, we need to configure Rust to optimize the final binary for size:
**shellcode/Cargo.toml**
```toml
[package]
name = "shellcode"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]


[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "z"
lto = true
codegen-units = 1
```

Now the configuration is done, we can start crafting the shellcode.

First, the boilerplate:

**shellcode/main.rs**
```rust
#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const SYS_WRITE: usize = 1;
const SYS_EXIT: usize = 60;
const STDOUT: usize = 1;
static MESSAGE: &str = "hello world\n";
```

Then, we implement the syscalls. It's the only part that requires assembly and is not architecture-agnostic:
```rust
unsafe fn syscall1(syscall: usize, arg1: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

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
```

And finally, we can write the entry point of the shellcode:
```rust
#[no_mangle]
fn _start() {
    unsafe {
        syscall3(
            SYS_WRITE,
            STDOUT,
            MESSAGE.as_ptr() as usize,
            MESSAGE.len(),
        );

        syscall1(SYS_EXIT, 0)
    };
}
```

It can be compiled with:
```makefile
.PHONY: build_shellcode
build_shellcode:
	cd shellcode && cargo +nightly build --release
	strip -s shellcode/target/release/shellcode
	objcopy -O binary shellcode/target/release/shellcode shellcode.bin
```

and examined with:
```makefile
.PHONY: dump_shellcode
dump_shellcode: build_shellcode
	objdump -D -b binary -mi386 -Mx86-64 -Mintel -z shellcode.bin
```

```shell
$ make dump_shellcode
Disassembly of section .data:

00000000 <.data>:
   0:   48 8d 35 13 00 00 00    lea    rsi,[rip+0x13]  # 0x1a
   7:   6a 01                   push   0x1
   9:   58                      pop    rax
   a:   6a 0c                   push   0xc
   c:   5a                      pop    rdx
   d:   48 89 c7                mov    rdi,rax
  10:   0f 05                   syscall
  12:   6a 3c                   push   0x3c
  14:   58                      pop    rax
  15:   31 ff                   xor    edi,edi
  17:   0f 05                   syscall
  19:   c3                      ret
  1a:   68 65 6c 6c 6f          push   0x6f6c6c65    # "hello world\n"
  1f:   20 77 6f                and    BYTE PTR [rdi+0x6f],dh
  22:   72 6c                   jb     0x90
  24:   64                      fs
  25:   0a                      .byte 0xa
```

**38 bytes! It's even better than our hand-crafted shellcode!**

The only imperfection is the useless `ret` instruction.

```shell
$ make execute_shellcode
hello world
```

## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_shellcode) (please don't forget to star the repo 🙏).



**Want to learn how to craft more advanced shellcodes in Rust (such as a TCP reverse shell)?** Get my book: **[Black Hat Rust](https://kerkour.com/black-hat-rust)**. All early-access supporters get a special discount and awesome bonuses: [https://kerkour.com/black-hat-rust](https://kerkour.com/black-hat-rust).

**Warning**: this offer is limited in time!
