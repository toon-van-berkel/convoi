# Convoi OS

**Convoi OS** is an experimental x86_64 operating system written in Rust.

The project explores operating-system development from the ground up, with a long-term focus on **memory efficiency**, **privacy**, and **deep customizability**.

Convoi is currently in early development. The project boots into a bare-metal Rust kernel and is gradually expanding toward memory management, hardware abstraction, graphics, system services, and eventually user-space applications.

> **Status:** Early development / experimental

---

## About the Project

Convoi started as an experiment in understanding what actually happens underneath conventional operating systems.

Rather than building on top of an existing desktop environment, the project works directly with the hardware-facing parts of the system: booting, memory, interrupts, graphics, drivers, and kernel architecture.

The long-term goal is to develop an operating system that prioritizes:

* **Memory efficiency** — keeping system overhead and RAM usage under control.
* **Privacy** — minimizing unnecessary data collection and background communication.
* **Customizability** — giving users control over how the operating system behaves and looks.
* **Performance** — avoiding unnecessary layers and services where possible.
* **Learning** — using the project to explore low-level systems programming and OS architecture.

---

## Current State

The current kernel implements the first pieces required to run Rust without a conventional operating system underneath it.

### Implemented

* Bare-metal Rust environment using `#![no_std]`
* Custom entry point using `#![no_main]`
* Custom panic handler
* Custom x86_64 target specification
* Rust `bootloader` integration
* `bootimage` build/run configuration
* Direct VGA text-buffer access
* Basic kernel output:

  * `Hello World from Convoi`

Development progress is tracked in [ROADMAP.md](ROADMAP.md).

---

## Architecture

The intended high-level architecture of Convoi is:

```text
┌─────────────────────────────────────────────────────────┐
│                    User Space                           │
│                   Apps / UI                             │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                 System Services                         │
│              Drivers / Core Services                    │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                   Convoi Kernel                         │
│        Memory / Interrupts / Scheduling / I/O           │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                     Hardware                            │
│                  x86_64 / QEMU                          │
└─────────────────────────────────────────────────────────┘
```

The majority of this architecture is still planned and will be implemented incrementally.

---

## Project Structure

```text
convoi/
├── .cargo/
│   └── config.toml
├── src/
│   └── main.rs
├── Cargo.lock
├── Cargo.toml
├── x86_64-kernel.json
├── README.md
├── ROADMAP.md
└── LICENSE
```

### Important files

**`src/main.rs`**
Contains the current kernel entry point, panic handler, and low-level VGA output.

**`x86_64-kernel.json`**
Defines the custom bare-metal x86_64 compilation target.

**`.cargo/config.toml`**
Configures Cargo to build the Rust core libraries for the custom target and use the `bootimage` runner.

**`Cargo.toml`**
Contains the package configuration, bootloader dependency, and bootimage configuration.

---

## Building Convoi

Convoi requires a Rust development environment capable of building the Rust standard components for a custom bare-metal target.

### Requirements

* Rust / Cargo
* Rust nightly toolchain
* `rust-src`
* `llvm-tools-preview`
* `bootimage`
* QEMU x86_64

A typical Rust setup is:

```bash
rustup toolchain install nightly

rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly

cargo install bootimage
```

QEMU must also be installed and available from your command line.

### Run

With the required tools installed:

```bash
cargo +nightly run
```

The repository's Cargo configuration uses the custom x86_64 target and `bootimage` runner.

During the current development stage, a successful boot should display:

```text
Hello World from Convoi
```

---

## Roadmap

Convoi is being developed incrementally.

Major areas include:

* Boot and bare-metal infrastructure
* Kernel I/O
* Interrupt and exception handling
* Memory management
* Heap allocation
* Graphics and framebuffer rendering
* Hardware drivers
* Task/process management
* System services
* User-space applications
* Privacy and security architecture
* Customization

See the full development checklist in **[ROADMAP.md](ROADMAP.md)**.

---

## Development

Convoi is currently developed by:

* [Toon van Berkel](https://github.com/toon-van-berkel)
* [Casper van Gameren](https://github.com/CaspervGameren)

The project is experimental and under active development. Interfaces, architecture, tooling, and implementation details may change significantly as development progresses.

---

## Documentation & Resources

Resources used while researching and developing Convoi include:

* [Writing an OS in Rust](https://os.phil-opp.com/)
* [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

---

## License

Copyright © 2026 Toon van Berkel and Casper van Gameren.

All rights reserved.

This repository is publicly available for portfolio, educational, and reference purposes. It is **not open-source software** and no general permission is granted to copy, modify, redistribute, republish, or incorporate the project into other software.

See [LICENSE](LICENSE) for the full terms.
