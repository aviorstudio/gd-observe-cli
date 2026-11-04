<!-- Generated from private documentation source. Do not edit directly. Source SHA256: cd9a3dab104d1fd58b740cdf68ea8243fd485ad98673080e0b309e6d367fed84 -->

# gdobs

Open-source Rust companion to [gd-observe](https://github.com/aviorstudio/gd-observe).

## Install

Linux and macOS, x86_64 or arm64:

```sh
curl -fsSL https://raw.githubusercontent.com/aviorstudio/gd-observe-cli/main/scripts/install.sh | sh
```

The installer selects this repository's published `cli-v` releases, verifies archive checksums and installs `gdobs` into `~/.local/bin`. Use `sh -s -- --version X.Y.Z --install-dir /absolute/path` to select a release or destination. Windows x86_64 and arm64 ZIPs are available from GitHub Releases. Rust is only required to build from source.

## Usage

```sh
gdobs watch --addr ws://127.0.0.1:8765
gdobs --json snapshot --url ws://127.0.0.1:8765
gdobs assert --budget metrics-budget.json
gdobs capture --duration 30s --out .artifacts/capture
gdobs diff before.json after.json
```

`gdobs` connects to a running game's gd-observe WebSocket server. It provides a Ratatui live dashboard, metrics snapshots, performance assertions, diagnostic captures and offline comparisons. The Godot addon is installed separately.

Endpoint selection supports `--url`, `--addr`, `--env` and `--project`; the default is `ws://127.0.0.1:8765`. A configured remote server can use the process-only `GDOBS_AUTH_TOKEN` environment variable. WSS uses system trust roots. Use `gdobs --help` for command flags.

## Development

`mise.toml` pins contributor tools. Run `mise run check` for formatting, static checks, native behavioral tests, TLS trust tests, installer checks and the executable build. `cargo build --release --locked` builds a standalone release executable. `mise run clean` removes declared generated outputs.

## License and provenance

MIT. `SOURCE.json` records the original public source revision and file hashes.
