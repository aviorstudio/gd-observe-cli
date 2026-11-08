<!-- Generated from private documentation source. Do not edit directly. Source SHA256: 008a9808c321444e81fbc5af7f5787e4ce6fac9992c6615008eba7fe0f6b4d51 -->

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
gd-observe-cli assert --budget metrics-budget.toml
gd-observe-cli capture --duration 30s --out .artifacts/capture
gd-observe-cli diff before.json after.json
```

`gd-observe-cli` connects to a running game's gd-observe WebSocket server. It provides a Ratatui live dashboard, metrics snapshots, performance assertions, diagnostic captures and offline comparisons. The Godot addon is installed separately.

Set `GDOBS_URL` in the process environment, or use `--url` / `--addr` for one command. Explicit flags take precedence; the default is `ws://127.0.0.1:8765`. Keep public endpoint defaults in Mise and inject approved Preview credentials through ghenv. Endpoint file discovery and the `--env` / `--project` flags have been removed from current source. Installed releases retain their released interface.

Authentication uses only the process variable `GDOBS_AUTH_TOKEN`. WSS uses system trust roots. `--budget` reads TOML, while snapshots, captures, diffs and WebSocket messages retain JSON. Use `gd-observe-cli --help` for command flags.

## Development

`mise.toml` pins contributor SDKs. The canonical generated task block in root `mise.toml` selects an immutable revision of the shared cookbook and binds it to `cookbook.toml`.

```sh
mise run help
mise run setup
mise run build
mise run test
```

`mise run help` works offline before setup. `mise run setup` explicitly prepares the pinned SDKs, locked dependencies and declared fixtures. `mise run build` compiles the executable, packages the native archive and verifies its source and output identity. `mise run test` runs lint, deterministic native unit tests, security checks and the declared integration journeys against those verified artifacts. Run build again after changing source; tests do not install tools or build missing shipping artifacts.

Use `mise run fmt` to apply formatting, `mise run test:lint`, `mise run test:unit`, `mise run test:security` or `mise run test:integration` to select a group, and `mise run clean` to remove the declared generated outputs. Build and test evidence is recorded in `.artifacts/verification`.

CI retains Linux, macOS and Windows builds for x86_64 and arm64. The integration group preserves installer and TLS trust checks. Published installers continue to use checksummed release archives.

## License and provenance

MIT. `SOURCE.json` records the original public source revision and file hashes.
