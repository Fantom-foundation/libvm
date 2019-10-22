libvm
=====
[![Rust: nightly](https://img.shields.io/badge/Rust-nightly-blue.svg)](https://www.rust-lang.org) [![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE) [![Build Status](https://travis-ci.org/Fantom-foundation/libvm.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libvm)

`libvm` is an abstraction of the concept of a vm.

Example of use:

```rust
struct ExampleCpu {
    program: Vec<ExampleInstruction>,
    pc: usize,
}

struct ExampleInstruction {}

impl Instruction for ExampleInstruction {
    fn size(&self) -> Result<usize, Error> {
        Ok(0)
    }
    fn get_cycles(&self) -> Result<usize, Error> {
        Ok(0)
    }
}

impl Cpu for ExampleCpu {
    fn execute_instruction(&mut self, instruction: I) -> Result<(), Error> {
        Ok(())
    }
    fn get_pc(&self) -> usize {
        self.pc
    }
    fn get_next_instruction(&mut self) -> Option<I> {
        self.program.pop()
    }
    fn can_run(&self) -> bool {
        true
    }
    fn is_done(&self) -> bool {
        self.pc < program.len()
    }
    fn increase_pc(&mut self, steps: usize) {
        self.pc += steps;
    }
}

struct ExampleConsensus {};

impl Consensus<u8> for ExampleConsensus {};

struct ExampleDistributedVM {
    cpu: ExampleCpu,
};

impl DistributedVM<Cpu<ExampleInstruction>, ExampleInstruction, u8, ExampleConsensus<u8>>
    for ExampleDistributedVM {
    fn serve(self) {
        while !cpu.is_done() {
            cpu.execute();
        }
    }
}

fn run(program: Vec<ExampleInstruction>) {
    let cpu = ExampleCpu {
        program,
        pc: 0,
    };
    let dvm = ExampleDistributedVM { cpu };
    dvm.serve();
}
```

---

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### CLI instructions

```bash
# Install Rust (nightly)
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Install clippy (Rust linter)
$ rustup component add clippy
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libtransport-http && cd libtransport-http
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```
