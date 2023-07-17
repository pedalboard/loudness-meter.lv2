# Loudness meter plugin

A plugin that measures the loudness of the signal and exposes it as midi messages

The loudness is measured according to the [EBU R128 loudness standard](https://tech.ebu.ch/docs/tech/tech3341.pdf)

To avoid MIDI compatibility issues the values are exposed as Note OFF messages: 

|            | type     | Channel | Note | Velocity                               |
|------------|----------|---------|------|----------------------------------------|
| momentary  | Note Off | 4       | C1   | 0-127 (rounded negated value in LUFS)  |


## Build dependencies

install rust with https://rustup.rs/

```
rustup target add aarch64-unknown-linux-gnu
cargo install cross release
make clean build bundle
```

Workaround on Apple M1 using orbstack

Manually pull the image before running the build:

```
docker pull ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5 --platform linux/amd64
```
