# Writing shellcodes in Rust

## Summary
Shellcode development is an ungrateful task. Writing assembly by hand is definitely not sexy. Fortunately for us, Rust, one more time, got our back! In this chapter we will learn how to write shellcodes in plain Rust with no_std.


Writing in assembly is so 1990, so why hackers and pentesters still have to write their shellcode by hand in assembly,
and adapt for each different COU instruction set?

enjeux:
- developper des shellcodes en Rust
- developper

hook:

plan:

* https://github.com/zerosum0x0/SassyKitdi
* https://github.com/lf-/ctf
* https://lfcode.ca/blog/writeonly-in-rust
* https://www.reddit.com/r/rust/comments/4a92qk/is_it_possible_to_execute_shellcode_directly_from/
* https://www.reddit.com/r/rust/comments/c2zkj8/rust_for_exploit_development/
* https://zerosum0x0.blogspot.com/2020/08/sassykitdi-kernel-mode-tcp-sockets.html


## Resources


### Assembly

* http://embed.rs/articles/2016/arm-inline-assembly-rust/
* https://os.phil-opp.com/freestanding-rust-binary/
* https://doc.rust-lang.org/1.8.0/book/inline-assembly.html
* https://filippo.io/linux-syscall-table/


### Shellcode

* https://www.reddit.com/r/rust/comments/4a92qk/is_it_possible_to_execute_shellcode_directly_from/
* https://stackoverflow.com/questions/55856247/how-to-execute-raw-instructions-from-a-memory-buffer-in-rust
* https://users.rust-lang.org/t/how-can-i-execute-hex-opcodes-directly-in-rust/31123/13
* https://lfcode.ca/blog/writeonly-in-rust
* https://github.com/lf-/ctf/blob/main/writeonly.rs/src/main.rs

* https://docs.rust-embedded.org/embedonomicon/memory-layout.html
* https://github.com/mikesiko/shellcode_launcher
* https://github.com/topics/shellcode-loader
* https://docs.rs/embed_plist/1.2.0/embed_plist/
* https://security.stackexchange.com/questions/176096/how-does-shellcode-really-run
* https://tuttlem.github.io/2017/10/28/executing-shellcode-in-c.html
* https://www.contextis.com/en/blog/a-beginners-guide-to-windows-shellcode-execution-techniques