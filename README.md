# furiosa-schedule-viewer

Local web server that serves the furiosa-schedule-viewer.

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
cargo run --release -- --host 127.0.0.1 --port 9254
```

## Install

Install directly from the repository (no clone needed):

```sh
cargo install --path .
furiosa-schedule-viewer --host 127.0.0.1 --port 9254
```

Pin to a specific revision or tag with `--rev <sha>` or `--tag <tag>`.

From a local checkout:

```sh
cargo install --path .
```
