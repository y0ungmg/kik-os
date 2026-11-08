# KiK OS Architecture

## Overview

KiK OS is a modular monolithic kernel operating system for x86_64 processors.

## Subsystems

### Boot
- UEFI boot application (custom)
- Legacy BIOS multi-stage bootloader (custom)
- Boot information structure (handoff to kernel)

### Kernel
- GDT / IDT
- Interrupt handling
- Physical memory manager
- Virtual memory manager
- Kernel heap
- Scheduler (preemptive)
- System calls
- IPC

### Drivers
- Serial output
- Framebuffer
- Keyboard / Mouse
- Timer (PIT/APIC)
- Storage controllers
- PCI/PCIe enumeration

### Filesystem
- KiKFS (native)
- FAT (ESP only, for UEFI boot assets)

### Userland
- Init process
- System services
- Native libraries
- Applications

### Desktop
- Compositor
- Window manager
- Shell
- Widgets

## Design Principles

- Clear separation of concerns
- Well-defined interfaces
- Strict ownership of hardware resources
- Security-first (kernel/user isolation)