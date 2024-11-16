# Chip-8 minimal emulator

This is a minimal one-file Chip-8 emulator written in Rust.
The graphics and input are handled using `sdl2`, however any backend can be relatively easily swapped in.
Sound is not implemented.

![screenshot](https://i.imgur.com/IP4mW9d.png)

## Usage

```bash
cargo run --release -- path/to/rom
```

## Controls

The Chip-8 has a 16-key hexadecimal keypad, which is mapped to the following keys:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```
