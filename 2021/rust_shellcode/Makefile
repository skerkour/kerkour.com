.PHONY: build_shellcode
build_shellcode:
	cd shellcode && cargo +nightly build --release
	strip -s shellcode/target/release/shellcode
	objcopy -O binary shellcode/target/release/shellcode shellcode.bin

.PHONY: dump_shellcode
dump_shellcode: build_shellcode
	objdump -D -b binary -mi386 -Mx86-64 -Mintel -z shellcode.bin

.PHONY: run_shellcode
run_shellcode: build_shellcode execute

.PHONY: execute
execute:
	cd executor && cargo run
