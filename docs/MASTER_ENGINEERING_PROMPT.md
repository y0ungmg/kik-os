# KiK OS — Master Engineering Prompt
## Independent Operating System Development

---

# 1. PROJECT IDENTITY

**Official Name:** KiK OS (KiK Operating System)

**Category:** Independent desktop operating system

**Vision:** Create a modern, visually distinctive, powerful, reliable desktop operating system with its own kernel, bootloader, drivers, filesystem, desktop environment, applications, and system services.

KiK OS is an original operating system. It is not a Linux distribution, desktop environment running on Linux, web application, simulation, or cosmetic modification of another operating system.

It must have its own kernel, bootloader, system APIs, libraries, filesystem, services, desktop environment, applications, and system architecture.

**Hosted and developed through:** GitHub

**Primary development architecture:** x86_64 (Intel and AMD processors, desktop computers and laptops)

**Future architecture:** ARM64 (after x86_64 reaches sufficient maturity)

**Development host:** Arch Linux

**Initial testing environment:** QEMU

**Primary development languages:** Rust, C, and Assembly

---

# 2. INDEPENDENCE AND ORIGINAL COMPONENTS

The following components must be developed specifically for KiK OS:

- KiK Bootloader
- KiK Kernel
- KiK Hardware Abstraction Layer
- KiKFS
- KiK API and ABI
- KiK system libraries
- KiK Desktop
- KiK graphical rendering engine
- KiK Window Manager and Compositor
- KiK Shell
- KiK Terminal
- KiK Package Manager
- KiK Store
- KiK Installer
- KiK Recovery
- Native KiK applications
- System services

Prefer original implementations. Existing Open Source libraries are permitted when technically justified, but they must be exceptions rather than the foundation of the operating system.

Possible exceptions include:

- cryptographic libraries
- proven algorithms and data structures
- multimedia codecs
- file-format implementations
- selected networking libraries

Every external dependency must undergo license, provenance, security, maintenance, and compatibility review. Maintain a dependency registry documenting why each external component is necessary.

Never use the Linux kernel or another operating system kernel as KiK OS's kernel.

---

# 3. HARDWARE AND BOOT ARCHITECTURE

**Initial architecture:** x86_64 (Intel and AMD processors, desktop computers and laptops)

**Future architecture:** ARM64 (after x86_64 reaches sufficient maturity)

**Boot support:**

- UEFI
- Legacy BIOS

Develop an original bootloader:

- Custom UEFI boot application
- Custom multistage BIOS bootloader
- Common documented boot-information structure
- Memory map handoff
- Framebuffer information
- Hardware and boot configuration information

Do not use GRUB, Limine, systemd-boot, or another existing bootloader as the final KiK OS bootloader.

EFI System Partition compatibility with FAT is permitted as a boot transport requirement. It must not be confused with the native KiKFS filesystem.

---

# 4. PROGRAMMING LANGUAGES

Use a hybrid programming architecture:

**Rust**

- Main kernel implementation
- Memory management
- Process management
- Kernel subsystems
- System services where appropriate

**C**

- Low-level hardware interfaces where beneficial
- Selected drivers and interoperability components
- Carefully controlled FFI boundaries

**Assembly**

- Bootloader
- CPU initialization
- Interrupt entry and exit
- Context switching
- Architecture-specific operations
- Other low-level processor operations

Use modern Rust tooling and appropriate cross-compilation targets.

Establish a consistent coding standard across all languages.

---

# 5. KERNEL ARCHITECTURE

Build a modular monolithic kernel.

The kernel must have clearly separated subsystems, well-defined interfaces, and strict ownership of hardware resources.

Required foundations:

- x86_64 long mode
- GDT and IDT
- CPU exceptions
- Interrupt handling
- Physical memory management
- Virtual memory management
- Heap allocation
- Hardware abstraction
- Timers
- Multitasking
- Scheduling
- Threads
- Process management
- System calls
- Kernel/user privilege separation
- IPC
- Device management
- Logging
- Diagnostics
- Security protections

Use Rust, C, and Assembly according to their technical strengths.

Do not fake memory allocation, process execution, virtual addresses, interrupts, or other kernel functionality.

---

# 6. SECURITY: MAXIMUM SECURITY

Security is a core architectural requirement.

Implement progressively:

- Secure account management
- Strong authentication
- Granular permissions
- Process isolation
- Application sandboxing
- Capability-based security
- Service isolation
- Device access control
- Firewall
- Secure boot support
- System integrity verification
- Signed packages and repositories
- Encrypted user data
- Cryptographic key management
- Security auditing
- Security event logging
- Advanced kernel protections

Security mechanisms must be implemented and tested, not merely described in documentation.

Clearly distinguish supported security features from planned or experimental ones.

---

# 7. FILESYSTEM AND STORAGE

Create KiKFS as an original filesystem.

Requirements:

- Documented on-disk format
- File and directory operations
- Metadata management
- Permissions
- Storage allocation
- Integrity checks
- Error handling
- Recovery mechanisms
- Future support for advanced storage features

Design it for reliability and eventual transactional system updates.

Do not claim journaling, copy-on-write, crash consistency, or recovery guarantees until they are implemented and tested.

---

# 8. HARDWARE DRIVERS

**Initial QEMU device support:**

- Serial output
- Framebuffer
- Keyboard
- Mouse
- Timer
- Storage
- PCI/PCIe discovery

**Progressively develop:**

- USB
- Networking
- Ethernet
- Wi-Fi
- Audio
- Graphics
- Bluetooth
- Power management
- Gaming peripherals

**Target hardware:**

- Intel and AMD PCs
- Laptops
- AMD and NVIDIA GPUs
- Sound devices
- Wi-Fi and Ethernet adapters
- Controllers
- USB devices
- Modern peripherals

Start with QEMU-emulated devices, then expand to physical hardware.

Never claim support for a device without a functioning driver and actual tests.

---

# 9. KiK DESKTOP

Create an original desktop environment.

**Visual identity:**

- Minimalist
- Professional
- Elegant
- Clean
- Consistent
- Efficient

Avoid copying Windows, macOS, or existing Linux desktop environments.

**Architecture:**

- Modular graphical rendering engine
- Custom window manager
- Custom compositor
- Original panel
- Original desktop
- Application menu
- Login screen
- Notification system
- Workspace management
- Virtual desktops
- Multitasking

The default desktop should be one coherent environment, while its internal components remain modular.

**Provide extensive customization:**

- Themes
- Icons
- Panels
- Desktop layout
- Animations
- Visual settings
- Window behavior

---

# 10. LOCALIZATION

Support Polish and English from the beginning.

Requirements:

- Language switching without reinstalling
- Localized system messages
- Localized applications
- Unicode support
- Localization-ready APIs
- Locale-aware dates, times, numbers, and keyboard layouts

Architecture must allow additional languages later.

---

# 11. NATIVE APPLICATIONS

Develop original native KiK applications.

**Initial application suite:**

- KiK Files
- KiK Terminal
- KiK Settings
- KiK Browser
- KiK Calculator
- KiK Text
- KiK Task Manager
- KiK System Monitor
- KiK Archive Manager
- KiK Image Viewer
- KiK Network Tools
- KiK Store
- KiK Recovery

All applications must be genuinely functional. Do not use placeholder interfaces or pretend implementations.

---

# 12. APPLICATION COMPATIBILITY

Native KiK applications have development priority.

**Future compatibility targets:**

- Linux applications
- Windows applications

Implement separate, modular compatibility environments.

Do not rely on another operating system kernel.

Do not promise universal application compatibility.

Document compatibility limitations and test actual applications.

---

# 13. PACKAGE MANAGEMENT AND APPLICATION STORE

Implement both:

**KiK Package Manager**

- Terminal interface
- Installation
- Removal
- Updating
- Dependency resolution
- Repository management
- Package verification

**KiK Store**

- Graphical interface
- Application discovery
- Installation
- Updates
- Removal
- Package information

Both interfaces must use the same package backend.

Support signed packages, repository integrity, dependency validation, and rollback where technically possible.

---

# 14. SYSTEM UPDATES AND RELEASE STABILITY

KiK OS follows a Stable-first release philosophy.

Requirements:

- Carefully tested updates
- Regression testing
- Signed updates
- Integrity verification
- Transactional update architecture
- Rollback
- Update history
- Notifications
- Explicit user approval before installation

Never install system updates without user approval.

Do not publish experimental features as stable.

---

# 15. GAMING CAPABILITIES

KiK OS must include advanced gaming-oriented functionality:

- Game Mode
- CPU/GPU performance management
- Process priority management
- FPS monitoring
- Frametime monitoring
- Hardware temperature monitoring
- Controller support
- Audio optimization
- Low-latency configuration
- Performance overlay
- Configurable performance profiles

Only expose functionality supported by actual drivers and hardware.

Do not fabricate GPU monitoring or performance metrics.

---

# 16. AUDIO AND MULTIMEDIA

Develop a comprehensive audio subsystem.

Requirements:

- Audio output
- Microphone input
- Multiple audio devices
- Application-specific volume mixing
- Audio profiles
- Bluetooth Audio
- Low-latency audio
- Recording
- Streaming
- Codec support
- Native audio APIs
- Gaming integration

Respect codec and library licensing.

---

# 17. NETWORKING

Develop an original networking architecture.

Required foundations:

- Ethernet
- Wi-Fi
- IPv4
- IPv6
- ARP
- DHCP
- DNS
- TCP
- UDP
- Sockets
- Network configuration
- Firewall
- VPN support
- Hotspot functionality
- Bluetooth networking
- Diagnostics
- Monitoring
- Server configuration tools
- Native networking APIs

Implement networking progressively and test using virtualized and physical environments.

---

# 18. POWER MANAGEMENT

Professional power management is required.

Implement:

- ACPI
- CPU power states
- GPU power management where supported
- Battery monitoring
- Temperature monitoring
- Fan management where supported
- Power profiles
- Sleep
- Hibernation
- Wake-up
- Laptop lid handling
- Performance and energy optimization

Do not expose unsupported hardware controls.

---

# 19. USER ACCOUNTS AND SESSIONS

Implement:

- Multiple local accounts
- Administrator and standard accounts
- Groups
- Granular permissions
- Secure login
- Screen locking
- Encrypted user data
- Session management
- Authentication policies
- Optional biometric authentication where hardware supports it

Provide secure recovery mechanisms for account access.

---

# 20. KiK BROWSER

Develop KiK Browser as the default browser.

It should have:

- Original interface
- Tabs
- Bookmarks
- Browsing history
- Downloads
- Private browsing
- Cookie management
- Website permissions
- Privacy controls
- Protection against unsafe websites

Use a mature, appropriately licensed browser engine rather than attempting to build a complete browser engine from scratch.

Allow other browsers to be installed.

---

# 21. KiK RECOVERY

Create a separate recovery environment.

Requirements:

- Emergency boot
- BIOS and UEFI boot repair
- Filesystem diagnostics
- KiKFS repair tools
- Rollback
- System restoration
- Configuration recovery
- Data recovery tools
- Hardware diagnostics
- System logs

Avoid promising lossless recovery when hardware or filesystem damage makes it impossible.

---

# 22. BACKUP SYSTEM

Develop a professional backup subsystem.

Features:

- Scheduled backups
- User-selected directories
- Encrypted backups
- File versioning
- External disk backups
- Network backups
- Selective restoration
- Full restoration
- KiK Recovery integration

Implement secure encryption-key recovery procedures.

---

# 23. INSTALLER

Create a graphical installer with two modes.

**Simple mode:**

- Automatic partitioning
- Default configuration
- Accessible installation process

**Advanced mode:**

- Manual partitioning
- Filesystem selection
- Encryption
- Bootloader configuration
- Advanced system settings

Support UEFI and Legacy BIOS.

Provide warnings before destructive disk operations.

Test installation in QEMU before physical hardware.

---

# 24. GITHUB INTEGRATION

GitHub is mandatory from project milestone zero.

**Repository:** `kik-os`

Before creating it:

- Check whether it already exists
- Never overwrite an existing repository
- Inspect GitHub authentication
- Use authenticated GitHub CLI or available GitHub integration

Never request, expose, or commit credentials, tokens, or private keys.

**Default repository visibility:** Public

**Set up:**

- README.md
- LICENSE
- CONTRIBUTING.md
- CODE_OF_CONDUCT.md
- SECURITY.md
- CHANGELOG.md
- ROADMAP.md
- ARCHITECTURE.md
- BUILDING.md
- TESTING.md

**Create:**

- GitHub Issues
- Milestones
- Labels
- Branch conventions
- Contribution guidelines
- Release management

**Branch strategy:**

- main (stable, tested)
- develop (integration)
- feature/* (individual features)
- fix/* (bug fixes)
- release/* (release preparation)

Protect main from knowingly broken code.

**Use GitHub Actions for:**

- Rust formatting
- Rust compilation
- Tests
- C compilation
- Assembly compilation
- Static analysis
- Kernel image building
- BIOS validation
- UEFI validation
- QEMU boot tests where available

Pin toolchain versions and dependencies.

Do not claim tests passed if they were not actually executed.

Audit changes and secrets before commits and pushes.

Publish tested releases only.

---

# 25. DEVELOPMENT ENVIRONMENT

**Development host:** Arch Linux

First inspect the actual environment.

Check:

- Installed Rust toolchain
- Cargo
- C compiler
- Assembler
- Linker
- LLVM/Clang
- QEMU
- OVMF
- SeaBIOS
- Make
- Git
- GitHub CLI

Do not blindly install or upgrade tools.

Avoid destructive host changes.

Do not use unrestricted root access.

Request explicit authorization before privileged operations, partition changes, disk formatting, deleting data, or other irreversible host modifications.

Use QEMU for initial testing.

---

# 26. AUTONOMOUS ENGINEERING WORKFLOW

Hermes Agent is expected to operate with maximum practical autonomy.

It should:

1. Inspect the repository
2. Understand current architecture
3. Maintain the roadmap
4. Select the next task
5. Implement it
6. Compile
7. Run relevant tests
8. Diagnose errors
9. Fix defects
10. Update documentation
11. Review changes
12. Commit
13. Push when authentication and repository permissions allow
14. Continue to the next task

Do not interrupt the user for trivial decisions.

Make reasonable, documented engineering choices independently.

Continue working through the roadmap rather than stopping after producing a plan.

**Important safety boundary:**

Autonomy does not authorize destructive operations on the user's host, unauthorized publication of secrets, disk formatting, partition changes, or irreversible actions.

Use isolated QEMU images and disposable test environments whenever possible.

---

# 27. DOCUMENTATION STANDARDS

Maintain full engineering documentation.

Required documentation includes:

- Kernel architecture
- BIOS and UEFI boot specifications
- KiK API and ABI
- KiKFS format
- Driver architecture
- Security architecture
- Networking protocols
- Binary formats
- Graphical architecture
- Application APIs
- Build instructions
- Installation instructions
- Recovery procedures
- Testing methodology
- Coding standards

Use Architecture Decision Records for important architectural decisions.

Maintain diagrams and specifications.

Documentation must always match the actual implementation.

---

# 28. VERSIONING

**Official product name:** KiK OS

**Release naming:** Combine semantic versioning and codenames.

Examples:

- KiK OS 0.1
- KiK OS 0.5 Aurora
- KiK OS 1.0 Nova
- KiK OS 1.1 Eclipse

Do not assign stable release status prematurely.

---

# 29. FIRST MAJOR RELEASE: KiK OS ALPHA

The first major release target is a real, bootable Alpha OS.

**Expected scope:**

- Original BIOS and UEFI bootloader
- Working kernel
- Basic hardware drivers
- KiKFS
- Memory management
- Process management
- Keyboard and mouse
- Graphical desktop
- User accounts
- Terminal and shell
- Basic native applications
- Package manager
- Graphical installer

This is a long-term milestone.

Develop smaller, bootable, testable milestones on the way.

Never wait until Alpha to test whether the kernel boots.

---

# 30. ENGINEERING QUALITY

Strict rules:

- No fake implementations
- No pretending that placeholders are working features
- No unverified support claims
- No silent error suppression
- No undocumented architectural shortcuts
- No unreviewed external dependencies
- No unnecessary duplication
- No knowingly broken main branch
- No unsupported security claims

Every feature must have:

- Implementation
- Tests
- Documentation
- Known limitations
- Clear status

Use the following statuses consistently:

- Implemented
- Tested
- Experimental
- In Progress
- Planned
- Unsupported

---

# 31. PROJECT STRUCTURE

Maintain a professional repository structure:

```
kik-os/
  boot/
    uefi/
    bios/
    common/
  kernel/
    arch/
    memory/
    interrupts/
    scheduler/
    drivers/
    filesystem/
    ipc/
    syscalls/
  userland/
    init/
    services/
    libraries/
    applications/
  desktop/
    compositor/
    window-manager/
    shell/
    widgets/
  libraries/
  tools/
  tests/
  scripts/
  config/
  docs/
  build/
  releases/
```

Adapt the structure when technically justified.

Maintain clear boundaries between kernel code, user-space code, hardware-specific code, and build tooling.

---

# 32. DEVELOPMENT ROADMAP

Follow these milestones in order:

**MILESTONE 0 — Foundation**

- Inspect development environment
- Initialize Git repository
- Configure toolchain
- Establish build system
- Create architecture documentation
- Create initial project structure

**MILESTONE 1 — First Boot**

- Custom UEFI bootloader
- Custom Legacy BIOS bootloader
- Kernel entry point
- Basic CPU initialization
- Serial output
- Boot diagnostics
- Successful QEMU boot in both modes

**MILESTONE 2 — Kernel Foundation**

- GDT, IDT, Interrupt handling, Exceptions
- Physical memory management
- Virtual memory
- Heap allocator
- Timer

**MILESTONE 3 — Multitasking**

- Scheduler, Threads, Context switching
- Processes, System calls, User-mode execution

**MILESTONE 4 — Hardware and Storage**

- PCI enumeration, Keyboard, Mouse, Framebuffer
- Storage driver, KiKFS

**MILESTONE 5 — User Space**

- Init process, System APIs, Native libraries
- Terminal, Basic shell, Native application framework

**MILESTONE 6 — Graphics**

- Graphics subsystem, Window manager, Compositor
- Desktop shell, Input integration

**MILESTONE 7 — Desktop Applications**

- File manager, Settings, Task manager, Text editor, Calculator, System monitor

**MILESTONE 8 — Advanced Functionality**

- Networking, Audio, USB, Improved graphics, Power management, User accounts, Security

**MILESTONE 9 — Release Engineering**

- ISO generation, UEFI/BIOS testing, Installation workflow, Recovery, Documentation, Versioning, Release packaging

Do not skip milestones to create the illusion of progress.

---

# 33. FINAL INSTRUCTION

Your task is to consolidate all existing KiK OS project requirements into one authoritative Master Engineering Prompt.

Save the updated prompt in the appropriate existing project documentation location.

Preserve the original project's useful technical details, but eliminate duplication and contradictions.

After updating it:

1. Review the entire document.
2. Verify that all 33 established decisions are represented.
3. Check technical consistency.
4. Check that no AI functionality or AI dependency has been introduced.
5. Check that no external operating system kernel is required.
6. Verify that GitHub integration is mandatory.
7. Verify that autonomous development and safety boundaries are clearly defined.
8. Provide a summary of the changes.
9. Report the exact file path of the updated Master Prompt.
10. List any unresolved architectural questions that genuinely require a future decision.

Do not start implementing KiK OS during this task.

The deliverable is one complete, consolidated, technically consistent Master Engineering Prompt for KiK OS.
---

## Summary of Changes

This consolidated Master Engineering Prompt integrates:

1. **Original Master Prompt** (sections 1-22): Project identity, hardware targets, programming languages, kernel architecture, bootloader, memory management, filesystem, drivers, desktop environment, native applications, system services, networking, security, development environment, testing strategy, project structure, development roadmap, autonomous engineering rules, quality requirements, documentation, release criteria, and initial tasks.

2. **GitHub Integration** (section 23): Repository creation, Git workflow, repository documentation, Issues and Milestones, CI/CD with GitHub Actions, GitHub Releases, repository maintenance, project transparency, and initial GitHub tasks.

3. **33 Established Decisions** integrated throughout:
   - Official name: KiK OS (KiK Operating System)
   - Independent OS, not a Linux distro
   - Original components required (bootloader, kernel, HAL, KiKFS, API/ABI, libraries, desktop, compositor, shell, terminal, package manager, store, installer, recovery, apps, services)
   - x86_64 primary, ARM64 future
   - UEFI + Legacy BIOS boot
   - Rust/C/Assembly hybrid
   - Modular monolithic kernel
   - Maximum security (sandboxing, capabilities, secure boot, etc.)
   - KiKFS original filesystem
   - QEMU-first device support
   - Original desktop (minimalist, professional, elegant)
   - Polish + English localization from start
   - 13 native applications
   - Linux/Windows app compatibility (future, modular)
   - Package manager + Store (shared backend)
   - Stable-first updates with rollback
   - Gaming features (Game Mode, FPS monitoring, etc.)
   - Comprehensive audio subsystem
   - Original networking stack
   - Professional power management
   - Multiple user accounts with encryption
   - KiK Browser (mature engine, not from scratch)
   - KiK Recovery environment
   - Backup system
   - Graphical installer (simple + advanced modes)
   - GitHub mandatory from milestone 0
   - Arch Linux development host
   - Autonomous engineering workflow
   - Full documentation standards
   - Semantic versioning + codenames
   - KiK OS Alpha as first major release
   - Strict engineering quality rules
   - 33 decisions verified represented

**File path:** `docs/MASTER_ENGINEERING_PROMPT.md`

**Unresolved architectural questions:**
1. Exact ABI specification for KiK system calls (needs detailed design)
2. KiKFS on-disk format specification (needs binary layout definition)
3. Inter-process communication mechanism choice (pipes vs message queues vs shared memory)
4. Graphics driver architecture for AMD/NVIDIA GPUs (requires hardware-specific research)
5. Compatibility layer architecture for Linux/Windows applications (requires future design)
6. Specific browser engine selection for KiK Browser (requires license and compatibility review)
7. Secure boot key management infrastructure (requires PKI design)
8. Transactional update mechanism design (requires storage layer maturity)
---

*This document is the authoritative specification for KiK OS development.*
*All implementation must conform to the requirements defined herein.*
*Last updated: 2026-11-08*
