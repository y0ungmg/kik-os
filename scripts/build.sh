#!/bin/bash
# KiK OS ISO Generation Script

set -e

BUILD_DIR="build"
ISO_DIR="$BUILD_DIR/iso"
OUTPUT="$BUILD_DIR/kik-os.iso"

echo "Building KiK OS..."

# Clean
rm -rf "$ISO_DIR"
mkdir -p "$ISO_DIR/EFI/BOOT"

# Build kernel
echo "Building kernel..."
cd kernel
RUSTC=/home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/rustc \
  /home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/cargo build \
  -Z build-std=core --target x86_64-unknown-none
cd ..

# Build UEFI bootloader
echo "Building UEFI bootloader..."
cd boot/uefi
RUSTC=/home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/rustc \
  /home/y0ungmg/.rustup/toolchains/nightly-2026-07-14-x86_64-unknown-linux-gnu/bin/cargo build \
  -Z build-std=core,alloc --target x86_64-unknown-uefi
cd ../..
cp boot/uefi/target/x86_64-unknown-uefi/debug/kik-uefi-boot.efi "$ISO_DIR/EFI/BOOT/BOOTX64.EFI"

# Build BIOS bootloader
echo "Building BIOS bootloader..."
nasm -f bin boot/bios/stage1.asm -o "$BUILD_DIR/bios_boot.bin"

echo "Build complete!"
echo "UEFI bootloader: $ISO_DIR/EFI/BOOT/BOOTX64.EFI"
echo "BIOS bootloader: $BUILD_DIR/bios_boot.bin"