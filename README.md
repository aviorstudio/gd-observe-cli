<!-- Generated from private documentation source. Do not edit directly. Source SHA256: 307c692884d68b5a1dd7cac680fe2aa6388b63de034e90636938f11ed8c2d3b4 -->

# gd-observe-cli

Open-source Rust companion to [gd-observe](https://github.com/aviorstudio/gd-observe).

## Install

Linux and macOS, x86_64 or arm64:

```sh
curl -fsSL https://raw.githubusercontent.com/aviorstudio/gd-observe-cli/main/scripts/install.sh | sh
```

The installer selects this repository's published `cli-v` releases, verifies archive checksums and installs `gd-observe-cli` into `~/.local/bin`. Use `sh -s -- --version X.Y.Z --install-dir /absolute/path` to select a release or destination. Windows x86_64 and arm64 ZIPs are available from GitHub Releases. Rust is only required to build from source.

## Usage

```sh
gd-observe-cli watch --addr ws://127.0.0.1:8765
gd-observe-cli --json snapshot --url ws://127.0.0.1:8765
gd-observe-cli assert --budget metrics-budget.json
gd-observe-cli capture --duration 30s --out .artifacts/capture
gd-observe-cli diff before.json after.json
```

`gd-observe-cli` connects to a running game's gd-observe WebSocket server. It provides a Ratatui live dashboard, metrics snapshots, performance assertions, diagnostic captures and offline comparisons. The Godot addon is installed separately.

Endpoint selection supports `--url`, `--addr`, `--env` and `--project`; the default is `ws://127.0.0.1:8765`. A configured remote server can use the process-only `GDOBS_AUTH_TOKEN` environment variable. WSS uses system trust roots. Use `gd-observe-cli --help` for command flags.

## Development

`mise.toml` pins contributor SDKs. The generated root Makefile selects an immutable revision of the shared cookbook and binds it to `cookbook.toml`.

```sh
make help
make setup
make build
make test
```

`make help` works offline before setup. `make setup` explicitly prepares the pinned SDKs, locked dependencies and declared fixtures. `make build` compiles the executable, packages the native archive and verifies its source and output identity. `make test` runs lint, deterministic native unit tests, security checks and the declared integration journeys against those verified artifacts. Run build again after changing source; tests do not install tools or build missing shipping artifacts.

Use `make fmt` to apply formatting, `make test:lint`, `make test:unit`, `make test:security` or `make test:integration` to select a group, and `make clean` to remove the declared generated outputs. Build and test evidence is recorded in `.artifacts/verification`.

CI retains Linux, macOS and Windows builds for x86_64 and arm64. The integration group preserves installer and TLS trust checks. Published installers continue to use checksummed release archives.

## License and provenance

MIT. `SOURCE.json` records the original public source revision and file hashes.
