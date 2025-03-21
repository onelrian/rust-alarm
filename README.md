# Run-Alarm

A simple Rust-based alarm application that allows users to set alarms with custom sounds and repeat modes. The alarm runs in the background and alerts users with a notification and sound.

## Features
- Set alarms with custom sounds.
- Repeat modes: one-time, daily, weekly, or custom days.
- Runs as a background process with notifications.

## Installation & Usage
Ensure you have Rust installed, then build and run:
```sh
cargo build --release
./target/release/run-alarm
```

## Dependencies
- `aplay` (for playing sound)
- `zenity` (for notifications)

## License
MIT License
```