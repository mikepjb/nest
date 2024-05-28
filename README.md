# Nest

Nest is a personal trial/test to create a NES emulator using Rust.

## Step-by-step

- I want to understand iNES format.
- I want to draw a single sprite from a game.

## Information on NES

- 8-bit 6502 CPU running at 1.79MHz.
    - 3 general purpose registers A/X/Y.
    - 3 special register P (status)/SP (stack pointer)/PC (program counter, or
      instruction pointer) all of which are 8-bit except the 16-bit PC.
- 16-bit addressable memory space, theoretical max of 64kb but actually only had
  2kb RAM.
- PPU (Picture Processing Unit) renders to 256x240 screen using 8x8 tiles for
  background, 64 8x8 or 8x16 sprites for moving objects. Also supports
  pixel-level scrolling.
- APU (Audio Processing Unit) supports 2 pulse channel, 1 triangle channel, 1
  noise channel, and 1 DMC (delta modulation) channel.
- Also, some cartridges had their own APUs, battery-backed RAM (for saves) and
  ofcourse ROM for the game data.

## Approach to Emulation

- Aim to reproduce the behaviour of the system at a surface level. This means
  we only care about internal implementations if they are necessary to
  accurately execute NES games.
- The emulator will have to operate concurrently with all the parts
  synchronised.

## Reference

A big thanks to yhang82's blog post providing an overview on the NES:
https://yizhang82.dev/nes-emu-overview
