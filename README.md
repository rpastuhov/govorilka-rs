# Govorilka-RS

A simple text-to-speech application written in Rust. Allows you to voice the typed text through a virtual audio cable and headphones at the same time.

Govorilka helps people who cannot use voice chat to communicate in voice channels. The user types text, which is converted to speech and played back through an audio device.

## Installation

1. Install Rust - https://www.rust-lang.org/tools/install
2. Clone the repository
3. Install the dependencies and build the project:
``bash
cargo build --release
``

## Language Setting

The project does not detect the language automatically. Before compiling, you must manually specify the desired language. Find the following code block in `main.rs` and change the value of `language`:

```rust
let narrator = GTTSClient {
volume: 0.5,
language: Languages::English, // Change to the desired language
    tld: "com",
};
```


## Usage

1. Install a virtual audio cable (for example, [VB-Audio Virtual Cable](https://vb-audio.com/Cable))
2. Launch the application
3. Select the **input** virtual audio cable from the device list
4. Select the **output** virtual audio cable in the target application (Discord, TeamSpeak)
3. Enter the text
4. Press Enter to play