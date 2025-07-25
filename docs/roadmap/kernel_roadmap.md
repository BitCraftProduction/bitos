# BitOS Kernel Dev Roadmap

## Phase 0: Project Setup & Toolchain

* [ ] Setup Rust nightly toolchain
* [ ] Install required tools: `bootimage`, `cargo-xbuild`, `qemu`, `llvm`, `nasm`
* [ ] Initialize kernel crate (`bitos_kernel`)
* [ ] Add `.cargo/config.toml` with target
* [ ] Create a custom target JSON (e.g. `x86_64-bitos.json`)
* [ ] Add `bootloader` dependency (or use multiboot with GRUB)
* [ ] Configure linker script
* [ ] Setup build script (`build.rs`)
* [ ] Test `cargo run` with QEMU boot (black screen or OK)



## Phase 1: Kernel Skeleton & Entry Point

* [ ] Define `entry_point!(main)` macro
* [ ] Define a basic panic handler
* [ ] Create `lib.rs` and `main.rs` files
* [ ] Setup `no_std` and `no_main`
* [ ] Map VGA buffer for printing
* [ ] Implement basic `println!()` macro
* [ ] Boot into “Hello BitOS!” using QEMU

 

## Phase 2: Memory Management (Paging)

* [ ] Detect total physical memory
* [ ] Map kernel code/data in higher half (KERNEL\_BASE)
* [ ] Setup page tables manually (x86\_64)
* [ ] Implement frame allocator (buddy system or bitmap)
* [ ] Implement heap allocator (`linked_list_allocator` or `buddy`)
* [ ] Support dynamic allocation with `alloc` crate
* [ ] Map VGA/text/framebuffer memory
* [ ] Map identity-mapped physical memory

 

## Phase 3: Interrupts & Exceptions

* [ ] Setup IDT (Interrupt Descriptor Table)
* [ ] Implement handlers for:

  * [ ] Breakpoint
  * [ ] Page fault
  * [ ] Double fault
  * [ ] General protection fault
  * [ ] Timer interrupt
  * [ ] Keyboard interrupt
* [ ] Enable PIC (Programmable Interrupt Controller)
* [ ] Setup APIC / xAPIC / IOAPIC
* [ ] Implement interrupt masking/unmasking

 

## Phase 4: Timer & Timekeeping

* [ ] Setup PIT or HPET for tick timer
* [ ] Implement global timer (uptime in ms)
* [ ] Add delay/sleep functions
* [ ] Track system boot time

 

## Phase 5: Input Devices

* [ ] Implement PS/2 keyboard input
* [ ] Add keyboard scancode to ASCII mapping
* [ ] Implement `read_line()` and `get_key()` for shell
* [ ] Add mouse input (PS/2 or USB in future)

 

## Phase 6: Multitasking & Scheduling

* [ ] Setup context switching
* [ ] Implement basic task struct
* [ ] Implement cooperative multitasking (yield)
* [ ] Implement preemptive multitasking (timer interrupts)
* [ ] Setup process states (Ready, Running, Blocked, etc.)
* [ ] Add process ID, priority
* [ ] Add task queues
* [ ] Implement basic `fork()` and task cloning
* [ ] Add syscall for process exit

 

## Phase 7: System Calls (Syscalls)

* [ ] Setup syscall interrupt (e.g. `int 0x80`)
* [ ] Implement syscall table
* [ ] Add syscalls:

  * [ ] `write()`
  * [ ] `read()`
  * [ ] `exit()`
  * [ ] `getpid()`
  * [ ] `fork()`
  * [ ] `sleep()`

 

## Phase 8: Filesystem & Disk I/O

* [ ] Implement block device abstraction
* [ ] Implement ATA / IDE / SATA driver
* [ ] Read MBR / GPT partitions
* [ ] Mount FAT12/16/32 or ext2/ext4
* [ ] Implement basic FS operations:

  * [ ] `open()`
  * [ ] `read()`
  * [ ] `write()`
  * [ ] `mkdir()`
  * [ ] `ls()`
* [ ] Create virtual file system (VFS) layer

 

## Phase 9: Drivers

* [ ] VGA text mode driver
* [ ] Framebuffer graphics driver (VBE/VGA/VESA)
* [ ] PCI Bus scanning
* [ ] USB base stack
* [ ] AHCI SATA driver
* [ ] Sound card (optional later)

 

## Phase 10: Shell / Userland Interface

* [ ] Create BitShell CLI
* [ ] Add built-in commands:

  * [ ] `help`
  * [ ] `clear`
  * [ ] `ls`
  * [ ] `cd`
  * [ ] `run`
  * [ ] `shutdown`
* [ ] Support launching basic ELF binaries
* [ ] Implement memory isolation for processes

 

## Phase 11: GUI (Optional, Future)

* [ ] Set graphics mode (VESA)
* [ ] Basic 2D graphics library
* [ ] Event-driven GUI loop
* [ ] Window manager and mouse cursor
* [ ] GUI apps like terminal, file explorer

 

## Phase 12: Packaging & Booting

* [ ] Generate bootable ISO
* [ ] Test on real hardware (USB)
* [ ] Support for GRUB bootloader
* [ ] Add logo and boot splash screen
* [ ] Add boot config parser

 

## Phase 13: Networking (Advanced)

* [ ] Add network card driver (e.g. RTL8139)
* [ ] Implement Ethernet stack
* [ ] Add ARP, ICMP (ping)
* [ ] Add IPv4
* [ ] Add TCP/UDP basic
* [ ] Implement HTTP client in kernel

 

## Phase 14: Security & Isolation

* [ ] Implement user/supervisor mode separation
* [ ] Prevent user code from accessing kernel memory
* [ ] Add syscall validation
* [ ] Setup basic access control for FS
* [ ] Sandboxed processes


## Phase 15: Testing & Automation

* [ ] Add unit tests using `cargo test` where applicable
* [ ] Add integration tests using QEMU snapshots
* [ ] Setup CI pipeline (e.g. GitHub Actions)
* [ ] Auto-generate documentation
* [ ] Kernel benchmarking framework


## BONUS – Experimental Features

* [ ] Rust async support in kernel
* [ ] WebAssembly runtime support
* [ ] BitOS scripting language
* [ ] AI/LLM integration in kernel space
* [ ] Bootloader password protection
* [ ] BIOS/UEFI dual support

