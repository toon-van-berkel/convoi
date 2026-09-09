# Convoi OS — Development Roadmap

This document tracks the planned development of Convoi OS.

The roadmap will evolve as the architecture becomes more mature and new requirements are discovered.

---

## Phase 1 — Boot & Bare Metal

Foundation required to execute Rust code directly on x86_64 hardware.

* [x] Configure `#![no_std]`
* [x] Configure `#![no_main]`
* [x] Implement custom kernel entry point
* [x] Implement custom panic handler
* [x] Create custom x86_64 target specification
* [x] Disable red zone
* [x] Configure panic strategy
* [x] Configure `build-std`
* [x] Integrate Rust `bootloader`
* [x] Configure `bootimage`
* [x] Boot kernel in QEMU
* [ ] Verify reproducible build on a clean development environment
* [ ] Investigate / define future UEFI boot strategy

---

## Phase 2 — Kernel I/O & Interrupts

Establish basic communication between the kernel and hardware.

### VGA

* [x] Map VGA text buffer at `0xb8000`
* [x] Write basic text directly to VGA memory
* [ ] Create reusable VGA text writer
* [ ] Add text formatting
* [ ] Add VGA colors
* [ ] Implement `print!`
* [ ] Implement `println!`

### CPU & Interrupts

* [ ] Create Global Descriptor Table (GDT)
* [ ] Create Interrupt Descriptor Table (IDT)
* [ ] Implement breakpoint exception handler
* [ ] Implement double-fault handler
* [ ] Configure hardware interrupts
* [ ] Configure timer interrupts
* [ ] Add keyboard interrupt handling

---

## Phase 3 — Memory Management

Build the memory-management infrastructure required by a more capable kernel.

### Paging

* [ ] Access physical memory information
* [ ] Read bootloader memory map
* [ ] Access active page tables
* [ ] Implement page-table abstractions
* [ ] Implement virtual memory mapping

### Physical Memory

* [ ] Create physical frame allocator
* [ ] Track available and reserved memory
* [ ] Prevent overlapping allocations

### Heap

* [ ] Initialize kernel heap
* [ ] Implement heap allocator
* [ ] Enable Rust `alloc`
* [ ] Support `Box`
* [ ] Support `Vec`
* [ ] Support `String`
* [ ] Add allocator error handler
* [ ] Add memory allocation tests

---

## Phase 4 — Graphics & Boot Experience

Move beyond VGA text mode and begin building Convoi's visual environment.

### Framebuffer

* [ ] Access graphics framebuffer
* [ ] Initialize graphics output
* [ ] Implement `draw_pixel`
* [ ] Implement `draw_rect`
* [ ] Implement basic shapes
* [ ] Implement bitmap/font rendering
* [ ] Add screen clearing
* [ ] Add basic double buffering

### Boot Experience

* [ ] Create graphical boot screen
* [ ] Implement boot progress reporting
* [ ] Create Convoi boot animation
* [ ] Implement binary-code stroke animation

See [Issue #1](../../issues/1) for the original boot-animation concept.

---

## Phase 5 — Hardware & Drivers

Introduce abstractions for interacting with physical and emulated hardware.

* [ ] Keyboard driver
* [ ] Mouse support
* [ ] Timer / clock support
* [ ] Storage-device detection
* [ ] Basic disk I/O
* [ ] PCI device enumeration
* [ ] Hardware abstraction interfaces
* [ ] Basic audio research
* [ ] Network-device research

---

## Phase 6 — Kernel Services

Expand Convoi from a bootable kernel into a functioning operating-system environment.

* [ ] Task abstraction
* [ ] Cooperative task execution
* [ ] Preemptive scheduling research
* [ ] Process model
* [ ] System calls
* [ ] Kernel/user privilege separation
* [ ] User-space memory management
* [ ] Inter-process communication
* [ ] Basic filesystem
* [ ] Executable loading

---

## Phase 7 — User Space

Create the first software running outside the kernel.

* [ ] Define application format
* [ ] Create initial user-space runtime
* [ ] Basic shell
* [ ] Command execution
* [ ] File browser
* [ ] Configuration system
* [ ] Application lifecycle
* [ ] User interface foundation

---

## Phase 8 — Privacy & Security

Develop the features that support Convoi's privacy-first goals.

* [ ] Permission model
* [ ] Application isolation
* [ ] Filesystem permissions
* [ ] User accounts
* [ ] Privilege management
* [ ] Secure configuration storage
* [ ] Network permission controls
* [ ] Background-service controls
* [ ] Telemetry-free default architecture
* [ ] Security audit strategy

---

## Phase 9 — Memory & Performance

Measure and improve the resource usage of the operating system.

* [ ] Kernel memory profiler
* [ ] Boot-time measurements
* [ ] Baseline idle RAM usage
* [ ] Memory leak detection
* [ ] Allocation statistics
* [ ] Reduce unnecessary allocations
* [ ] Optimize core data structures
* [ ] CPU usage monitoring
* [ ] Performance benchmarks

---

## Phase 10 — Customizability

Build the customization layer that will become one of Convoi's defining features.

* [ ] Central configuration system
* [ ] Theme system
* [ ] UI customization
* [ ] Configurable system services
* [ ] Startup application configuration
* [ ] Keyboard shortcuts
* [ ] User profiles
* [ ] Export/import settings
* [ ] Developer customization API

---

# Long-Term Goals

These are broader project goals rather than individual implementation tasks.

* [ ] Boot reliably on real x86_64 hardware
* [ ] Provide a usable graphical interface
* [ ] Maintain low idle memory usage
* [ ] Provide strong privacy defaults
* [ ] Minimize unnecessary background processes
* [ ] Give users extensive control over the system
* [ ] Run native Convoi applications
* [ ] Develop a stable hardware abstraction layer
* [ ] Establish automated kernel testing
* [ ] Establish a release process
* [ ] Produce installable Convoi OS images

---

## Project Status

Convoi OS is an experimental project.

Features marked as complete indicate that an initial implementation exists. They should not automatically be considered production-ready or final.
