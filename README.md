<!-- Generated from private documentation source. Do not edit directly. Source SHA256: 01f05b014cbfe27a409b84ff0a4431a40c5aca1748fef403a69841bc83032d27 -->

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

Current-source endpoint files use `.env.toml` and `.env.example.toml` (dotenv `.env` remains supported). `--project` discovers these files at the project root or under `godot_client/`; legacy endpoint JSON requires explicit conversion. `--budget` reads TOML, while snapshots, captures, diffs and WebSocket messages retain JSON. Installed releases retain their released interface.

Endpoint selection supports `--url`, `--addr`, `--env` and `--project`; the default is `ws://127.0.0.1:8765`. A configured remote server can use the process-only `GDOBS_AUTH_TOKEN` environment variable. WSS uses system trust roots. Use `gd-observe-cli --help` for command flags.

Current-source migration is manual: create the named TOML source with the same effective settings and archive the legacy JSON v1 source outside the active configuration directory. No automatic import command is provided. Preserve explicit `false`, zero and empty arrays; omit null overrides only where they meant inheritance or defaults. Do not copy credentials into tracked source files. Existing installed settings are not migrated by a source update.

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
