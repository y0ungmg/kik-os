# KiK OS

**KiK OS** — An independently engineered x86_64 desktop operating system with its own bootloader, kernel, filesystem, drivers, desktop environment and native applications.

## Vision

Create a modern, visually distinctive, powerful, reliable desktop operating system with its own kernel, bootloader, drivers, filesystem, desktop environment, applications, and system services.

KiK OS is built from scratch. It is not a Linux distribution, desktop theme, web application, emulator interface, or simulation.

## Development Status

- **Current Milestone:** MILESTONE 0 — Foundation
- **Status:** Initial project setup in progress

### Subsystem Status

| Subsystem | Status |
|---|---|
| Bootloader (UEFI) | Planned |
| Bootloader (BIOS) | Planned |
| Kernel | Planned |
| Memory Management | Planned |
| Process Scheduler | Planned |
| Filesystem (KiKFS) | Planned |
| Device Drivers | Planned |
| Desktop Environment | Planned |
| Native Applications | Planned |
| Networking | Planned |

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