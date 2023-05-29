# GBMU
A multiplatform GameBoy & GameBoy Color emulator, in pure Rust!

![A view of the emulator UI](https://i.imgur.com/gJfe3Gg.png)

It was written alone over the course of 2 months as part of a school project, as a way to learn Rust and to acquire knowledge in embedded electronics (and emulation thereof).

It will run most games somewhat accurately, from your most basic Tetris ROM to the infamous and more technically complex Metal Gear Solid: Ghost Babel.

# Features
## Working
- Accurate Sharp LR35902 CPU Emulation (passes Blargg's tests)
- Accurate PPU emulation
- Support for MBC1, MBC2, MBC3, MBC5 cartridges
- Automatic detection of GB/GBC compatibility
- Support for GBC only features (CPU frequency doubling, CPU-halting VRAM DMA Transfer)
- Force plain DMG (original monochrome GameBoy) emulation
- CPU Debugger
- VRAM contents inspector
- Pokémon is looking very good on this emulator

## To-Do
- Audio
- Emulation speed switching
- BIOS-less operation

# Build instructions
- Make sure the [Rust toolchain is installed](https://www.rust-lang.org/tools/install) (this was tested under versions 1.65.0 and 1.66.0)
- Clone the repository
- Run `cargo build --release`
- The binary is built under `target/release/gbmu`

The binary produced is a portable, statically-linked (except for libc) 20MB executable.

# Dependencies
- Window management based on [winit](https://crates.io/crates/winit)
- Frame rendering based on the [pixels](https://crates.io/crates/pixels) hardware-accelerated framebuffer
- Immediate-mode UI using [egui](https://crates.io/crates/egui)

# Credits
Original Logo art by RetroPunkZ - https://twitter.com/RetroPunkZ1

# Licence
This software is licensed under the GPL-3.0 License.  
See https://www.gnu.org/licenses/gpl-3.0.html
