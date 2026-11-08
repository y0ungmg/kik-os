# KiK OS Build System

# KiK OS Build System

NIGHTLY_CARGO := /home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/cargo
NIGHTLY_RUSTC := /home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/rustc

.PHONY: all kernel uefi bios clean test-uefi test-bios

all: kernel uefi bios

kernel:
	$(NIGHTLY_CARGO) build --manifest-path kernel/Cargo.toml -Z build-std=core --target x86_64-unknown-none

uefi:
	$(NIGHTLY_CARGO) build --manifest-path boot/uefi/Cargo.toml -Z build-std=core,alloc --target x86_64-unknown-uefi
	mkdir -p build/uefi/EFI/BOOT
	cp boot/uefi/target/x86_64-unknown-uefi/debug/kik-uefi-boot.efi build/uefi/EFI/BOOT/BOOTX64.EFI

bios:
	nasm -f bin boot/bios/stage1.asm -o build/bios_boot.bin

clean:
	rm -rf build/* kernel/target boot/uefi/target

test-uefi: uefi
	timeout 5 qemu-system-x86_64 -bios /usr/share/edk2/x64/OVMF.4m.fd -drive file=fat:rw:build/uefi,format=raw -serial stdio -display none -no-reboot

test-bios: bios
	timeout 5 qemu-system-x86_64 -drive file=build/bios_boot.bin,format=raw -serial stdio -display none -no-reboot