A program to record MKWii Time Trials without an emulator.

> [!NOTE]
> Some BRRES rendering/parsing logic was inspired by [Riistudio](https://github.com/snailspeed3/RiiStudio) and [noclip.website](https://github.com/magcius/noclip.website/).

## Prerequisites
This project is in Rust and uses the Bevy game engine. It also depends on the `brres` crate, which requires LLVM 19.1.7 to build correctly (or at least that's the version i tested) <br/>
on windows, install the binary [here](https://github.com/llvm/llvm-project/releases/tag/llvmorg-19.1.7)

make sure to create `.cargo/config.toml` and put your paths or change the linker toolchain

### TODOs:
  - [ ] brres renderer (current)
  - [ ] Kinoko integration (crucial)
  - [ ] UI
  - [ ] camera movement
  - [ ] mp4 exporting
  - [ ] input rendering
  - [ ] top 10 rendering
  - [ ] stuff like speedometer and custom texts
  - [ ] kcl/kmp rendering
  - [ ] rewrite of szs and brres crates (very late in the project, not needed)