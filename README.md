# schedule-viewer-runner

Local web server that serves the IR Viewer static bundle.

The `dist/` directory at the crate root holds the pre-built web assets and is embedded into the binary at compile time.

## Build

```sh
cargo build --release
```

## Run

```sh
cargo run --release
```

By default the server binds to `127.0.0.1:9254` and opens the page in your
default browser. Override with `--host` and `--port`:

```sh
cargo run --release -- --host 0.0.0.0 --port 8000
```

## Install

```sh
cargo install --path .
furiosa-schedule-viewer --host 0.0.0.0 --port 8000
```
