# bell

A minimal Windows CLI sound player written in Rust.

It can be appended to other commands to play a notification sound when the preceding command finishes.

## Usage

Without arguments, `bell.exe` plays the built-in `Aria_task_finished.wav`:

    bell.exe

To play a specified WAV file:

    bell.exe "C:\path\to\sound.wav"

The program waits until playback finishes before exiting.

## Features

- Native Windows CLI application
- Written in Rust
- No additional runtime required

## Build

Rust toolchain is required.

    cargo build --release

## License

See [LICENSE](LICENSE).
