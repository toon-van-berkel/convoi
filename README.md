# convoi - OS

Rust-based Operating System. Focus: RAM optimization, privacy, customizability.

---

## 🎯 Milestones

- [ ] **Phase 1: Bootloader & Bare Metal**
  - [ ] Setup `#![no_std]` and `#![no_main]`
  - [ ] Implement custom `panic_handler`
  - [ ] Configure custom target json (`x86_64-convoi_os.json`)
  - [ ] Integrate UEFI bootloader crate

- [ ] **Phase 2: Basic Kernel & I/O**
  - [ ] Map VGA text buffer (`0xb8000`)
  - [ ] Create a minimal VGA text driver (with formatting & colors)
  - [ ] Implement `println!` macro wrapper
  - [ ] Setup Exception Handling (IDT - Interrupt Descriptor Table)

- [ ] **Phase 3: Advanced Graphics & Memory**
  - [ ] Switch to Graphics Framebuffer (GOP)
  - [ ] Implement primitive pixel drawing (`draw_pixel`, `draw_rect`)
  - [ ] Setup Physical Memory Management (Page tables)
  - [ ] Initialize Heap Allocator (Enable `alloc` crate for `Vec`, `String`)

---

## 🏗️ Architecture
┌─────────────────────────────────────────────────────────┐
│               User Space (Apps / UI)                    │
└───────────────────────────┬─────────────────────────────┘
▼
┌─────────────────────────────────────────────────────────┐
│             System Services (Drivers)                   │
└───────────────────────────┬─────────────────────────────┘
▼
┌─────────────────────────────────────────────────────────┐
│             convoi - KERNEL (Memory / Interrupts)       │
└───────────────────────────┬─────────────────────────────┘
▼
┌─────────────────────────────────────────────────────────┐
│               Hardware / QEMU Emulator                  │
└─────────────────────────────────────────────────────────┘
---

## 📖 Documentation & Resources
- Core Guide: os.phil-opp.com
- Rust Embedded: rust-embedded.github.io/book
