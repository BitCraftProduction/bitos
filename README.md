
# VedOS – A New Era of Operating Systems

**VedOS** is a next-gen experimental operating system built in **Rust**, focused on security, simplicity, and exploration. This is not just an OS; it's an idea — to build a new digital world rooted in freedom, speed, and innovation.

> Current Level: 0  
> Status: Learning, prototyping, and simulating kernel logic


## Vision

VedOS is not a Linux clone. It's a fresh take on what a personal OS can be — secure, technical, and deeply personal. Built with:
- **Rust** for memory-safety and performance
- Custom kernel architecture (no `std`)
- Focus on AI integration, custom UI, and minimalist design


## Roadmap

* [x] Level 0: Basic kernel simulation with enums and handlers
* [ ] Level 1: Bare-metal setup (`#![no_std]`, `bootimage`, etc.)
* [ ] Level 2: Memory management and interrupt handling
* [ ] Level 3: Custom shell, UI, and file system
* [ ] Level ∞: BitAI, BitNote, BitLearn, BitDraw, etc. ecosystem

## Project Structure

### Current

```
VedOS/
├── bootloader/                         # Minimal Rust crate that boots and loads the kernel binary
│   ├── Cargo.toml                      # Bootloader crate configuration and dependencies
│   └── src/
│       └── main.rs                     # Entry point for bootloader logic (mostly handled by `bootloader` crate)
│
├── kernel/                             # Core kernel logic and modules for VedOS
│   ├── Cargo.toml                      # Kernel crate configuration and dependencies
│   ├── src/
│   │   ├── main.rs                     # Kernel's main entry point (e.g. init, test setup)
│   │   ├── memory.rs                   # Memory management setup (paging, heap, etc.)
│   │   ├── gdt.rs                      # Global Descriptor Table setup (CPU privilege and segment settings)
│   │   ├── serial.rs                   # Serial port (UART) setup for early debugging/logging
│   │   ├── interrupts.rs               # Interrupt Descriptor Table (IDT) and interrupt handler setup
│   │   └── logger.rs                   # Logging infrastructure for printing kernel messages
│   ├── target/                         # Auto-generated build output (ignored by Git)
│   └── .cargo/
│       └── config.toml                 # Custom target triple and runner setup (e.g., QEMU)
│
├── .gitignore                          # Ignore build files, editor configs, and logs
├── Cargo.toml                          # Root Cargo workspace configuration including kernel and bootloader

```

### Complete OS

```
VedOS/
├── bootloader/
├── kernel/
│   ├── arch/          # Architecture-specific code (x86_64, etc.)
│   ├── drivers/       # Keyboard, VGA, mouse, storage
│   ├── fs/            # Filesystem layer
│   ├── gui/           # GUI logic and drawing primitives
│   ├── net/           # Network stack
│   ├── scheduler/     # Task scheduler
│   ├── syscalls/      # System call interface
│   ├── userlib/       # Lib for user apps (libc-lite)
│   └── apps/          # Built-in apps or examples
├── user/              # User programs or GUI apps
├── tools/             # Build tools, ISO creator, etc.
├── Cargo.toml
├── .cargo/config.toml
└── README.md

```

## Author

Made with love by Ganesh Kumar & Ghost at [BitCraft Production](https://www.bitcraftproduction.com)


## License

This project is licensed under the **MIT License** – see [LICENSE](./LICENSE) for details.

> You are free to use, modify, share — as long as you include the original license and credit the author.

