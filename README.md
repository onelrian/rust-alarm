# Run-Alarm

A simple Rust-based alarm application that allows users to set alarms with custom sounds and repeat modes. The alarm runs in the background and alerts users with a notification and sound.

## Features
- Set an alarm with a specific time.
- Choose a custom sound from the `sounds/` directory.
- Set repeat modes (one-time, daily, weekly, or custom days).
- Runs as a background process.
- Built-in Docker support to avoid dependency issues.

## Installation & Usage
### Run Locally
Ensure you have Rust installed, then build and run:
```sh
cargo build --release
./target/release/run-alarm
```

### Run with Docker
Build and run using Docker:
```sh
docker build -t run-alarm .
docker run -it run-alarm
```

## Dependencies
- Rust (for manual builds)
- `aplay` (for playing sound)
- `zenity` (for notifications)

## License
MIT License
