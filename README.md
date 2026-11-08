# KiK OS

**KiK OS** — An independently engineered x86_64 desktop operating system with its own bootloader, kernel, filesystem, drivers, desktop environment and native applications.

## Vision

Create a modern, visually distinctive, powerful, reliable desktop operating system with its own kernel, bootloader, drivers, filesystem, desktop environment, applications, and system services.

KiK OS is built from scratch. It is not a Linux distribution, desktop theme, web application, emulator interface, or simulation.

## Development Status

- **Current Milestone:** MILESTONE 8 — Advanced Functionality (in progress)
- **Status:** Kernel subsystems implemented and building. Userland started. Advanced features in progress.

### Subsystem Status

| Subsystem | Status |
|---|---|
| Bootloader (UEFI) | Implemented and tested in QEMU |
| Bootloader (BIOS) | Implemented and tested in QEMU |
| Kernel Entry | Implemented |
| Serial Output | Implemented |
| GDT | Implemented |
| IDT | Implemented |
| Physical Memory Manager | Implemented |
| Kernel Heap | Implemented |
| Timer (PIT) | Implemented |
| Keyboard (PS/2) | Implemented |
| Framebuffer | Implemented |
| Virtual Memory | Implemented (basic) |
| Scheduler | Implemented (basic) |
| System Calls | Implemented (basic) |
| IPC | Implemented (basic) |
| Logging | Implemented |
| PCI Enumeration | Implemented |
| KiKFS Filesystem | Implemented (structure) |
| Networking | Implemented (basic) |
| Audio | Implemented (basic) |
| Init Process | Implemented |
| Shell | Implemented (basic) |
| Desktop Environment | Planned |
| Native Applications | Planned |

## Architecture

- **Kernel:** Modular monolithic kernel (Rust + C + Assembly)
- **Architecture:** x86_64 (long mode)
- **Boot:** UEFI and Legacy BIOS (custom bootloader — KiK Boot)
- **Filesystem:** Native KiKFS
- **Desktop:** KiK Desktop (custom compositor + window manager)

## Development Environment

- **Host OS:** Arch Linux
- **Testing:** QEMU (UEFI + Legacy BIOS)
- **Target:** x86_64

## Build Instructions

See [BUILDING.md](BUILDING.md) for detailed build instructions.

## QEMU Testing

See [TESTING.md](TESTING.md) for QEMU testing instructions.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full development roadmap.

## License

This project is licensed under the MIT License — see [LICENSE](LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.