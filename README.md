# Simple interpreter for INTEL 8086 CPU family assembly

This library is for education purposes only.

## Usage

As input 16 bits binary compiled in 8086 cpu assembly. Output decompiled assembly.

There are existing simple listings: [37](./listing37.asm), [38](./listing38.asm)

As a etalon compiler used [nasm](https://hasm.us)

## How to run

Required `rust` compiler and dev env to be installed. Use [rustup](https://rustup.rs/) to install

Run

`nasm <path-to-listing>` -> creates binary file

`cargo run <path-to-binary>` -> decompiles binary back to listing

## Details of the CPU family

For details read the [manual](./INTEL_The-8086-Family-Users-Manual.pdf)
