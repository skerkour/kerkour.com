+++
date = 2021-09-21T06:00:00Z
title = "2021_09_21_how_to_build_a_cross_platform_shellcode_in_rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/2021_09_21_how_to_build_a_cross_platform_shellcode_in_rust"
draft = true

[extra]
lang = "en"

comment ="""

https://stackoverflow.com/questions/41906688/what-are-the-semantics-of-adrp-and-adrl-instructions-in-arm-assembly
"""
+++


```
Disassembly of section .data:

0000000000000000 <.data>:
   0:   90000001        adrp    x1, 0x0
   4:   52800020        mov     w0, #0x1                        // #1
   8:   9100a021        add     x1, x1, #0x28
   c:   52800808        mov     w8, #0x40                       // #64
  10:   52800182        mov     w2, #0xc                        // #12
  14:   d4000001        svc     #0x0
  18:   aa1f03e0        mov     x0, xzr
  1c:   52800ba8        mov     w8, #0x5d                       // #93
  20:   d4000001        svc     #0x0
  24:   d65f03c0        ret
  28:   6c6c6568        ldnp    d8, d25, [x11, #-320]
  2c:   6f77206f        umlal2  v15.4s, v3.8h, v7.h[3]
  30:   0a646c72        bic     w18, w3, w4, lsr #27
```


```

Breakpoint 1, 0x0000aaaaaaab0a24 in executor::main ()
(gdb) disassemble /r
Dump of assembler code for function executor::main:
   0x0000aaaaaaab0a1c <+0>:	ff c3 00 d1	sub	sp, sp, #0x30
   0x0000aaaaaaab0a20 <+4>:	fe 13 00 f9	str	x30, [sp, #32]
=> 0x0000aaaaaaab0a24 <+8>:	28 02 00 f0	adrp	x8, 0xaaaaaaaf7000
   0x0000aaaaaaab0a28 <+12>:	08 bd 47 f9	ldr	x8, [x8, #3960]
   0x0000aaaaaaab0a2c <+16>:	e8 0b 00 f9	str	x8, [sp, #16]
   0x0000aaaaaaab0a30 <+20>:	e8 0b 40 f9	ldr	x8, [sp, #16]
   0x0000aaaaaaab0a34 <+24>:	e8 07 00 f9	str	x8, [sp, #8]
   0x0000aaaaaaab0a38 <+28>:	e8 0f 00 f9	str	x8, [sp, #24]
   0x0000aaaaaaab0a3c <+32>:	e8 07 40 f9	ldr	x8, [sp, #8]
   0x0000aaaaaaab0a40 <+36>:	00 01 3f d6	blr	x8
End of assembler dump.
(gdb) disassemble /r SHELLCODE
Dump of assembler code for function SHELLCODE:
   0x0000aaaaaaab09e8 <+0>:	01 00 00 90	adrp	x1, 0xaaaaaaab0000 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h17ec7e33d97bb71eE+28>
   0x0000aaaaaaab09ec <+4>:	20 00 80 52	mov	w0, #0x1                   	// #1
   0x0000aaaaaaab09f0 <+8>:	21 a0 00 91	add	x1, x1, #0x28
   0x0000aaaaaaab09f4 <+12>:	08 08 80 52	mov	w8, #0x40                  	// #64
   0x0000aaaaaaab09f8 <+16>:	82 01 80 52	mov	w2, #0xc                   	// #12
   0x0000aaaaaaab09fc <+20>:	01 00 00 d4	svc	#0x0
   0x0000aaaaaaab0a00 <+24>:	e0 03 1f aa	mov	x0, xzr
   0x0000aaaaaaab0a04 <+28>:	a8 0b 80 52	mov	w8, #0x5d                  	// #93
   0x0000aaaaaaab0a08 <+32>:	01 00 00 d4	svc	#0x0
   0x0000aaaaaaab0a0c <+36>:	c0 03 5f d6	ret
   0x0000aaaaaaab0a10 <+40>:	68 65 6c 6c	ldnp	d8, d25, [x11, #-320]
   0x0000aaaaaaab0a14 <+44>:	6f 20 77 6f	umlal2	v15.4s, v3.8h, v7.h[3]
   0x0000aaaaaaab0a18 <+48>:	72 6c 64 0a	bic	w18, w3, w4, lsr #27
End of assembler dump.



raison pour laquelle ca ne semble pas marcher:
en assembleur, 01 00 00 90	adrp	x1, 0xaaaaaaab0000

est different de l'interpretaiton dans le shellcode:
   0:   90000001        adrp    x1, 0x0
   4:   52800020        mov     w0, #0x1                        // #1
   8:   9100a021        add     x1, x1, #0x28
```
