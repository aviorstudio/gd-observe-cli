#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
case "${1:-all}" in
lint)
  cargo fmt --all --check
  cargo clippy --locked --all-targets -- -D warnings
  actionlint
  shellcheck scripts/*.sh
  ;;
build)
  cargo build --locked
  mkdir -p .artifacts/bin
  cp "${CARGO_TARGET_DIR:-target}/debug/gd-observe-cli" .artifacts/bin/gd-observe-cli
  ;;
test)
  cargo test --locked
  python3 tests/installer_test.py
  python3 tests/release_guard_test.py
  python3 tests/tls_test.py
  ;;
*) echo 'usage: check.sh lint|build|test' >&2; exit 2 ;;
esac
