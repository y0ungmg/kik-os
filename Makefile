# KiK OS Build System

TARGET := x86_64-unknown-none
KERNEL := kernel/target/$(TARGET)/debug/libkik_kernel.a

.PHONY: all kernel clean qemu-uefi qemu-bios

all: kernel

kernel:
	cargo build --manifest-path kernel/Cargo.toml --target $(TARGET)

clean:
	cargo clean --manifest-path kernel/Cargo.toml
	rm -f build/*

qemu-uefi: kernel
	@echo "UEFI boot not yet implemented"

qemu-bios: kernel
	@echo "BIOS boot not yet implemented"