all:
	cargo build
	cargo bootimage

run:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-chad_os.bin
