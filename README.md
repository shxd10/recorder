A program to record MKWii Time Trials without an emulator

## Prerequisites

This project depends on the `brres` crate, which requires LLVM 19.1.7 to build correctly (or at least that's the version i tested) <br/>
on windows, install the binary [here](https://github.com/llvm/llvm-project/releases/tag/llvmorg-19.1.7)

make sure to create `.cargo/config.toml` and put your paths or change the linker toolchain

### todo:
- brres renderer
- UI
- kinoko integration
- camera movement
- events that could be hooked up to gui/audio
- mp4 exporting
- input rendering
- top 10 rendering
- kcl/kmp rendering
- rewrite of szs and brres crates