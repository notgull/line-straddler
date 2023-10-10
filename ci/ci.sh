#!/bin/sh

set -eu

# Run CI-based tests for Line Straddler

rx() {
  cmd="$1"
  shift

  (
    set -x
    "$cmd" "$@"
  )
}

lstrl_check_target() {
  target="$1"
  command="$2"

  echo ">> Check for $target using $command"
  rustup target add "$target"
  rx cargo "$command" --target "$target"
  rx cargo "$command" --target "$target" --no-default-features \
      --features libm
  cargo clean
}

lstrl_test_version() {
  version="$1"
  extended_tests="$2"

  rustup toolchain add "$version" --profile minimal
  rustup default "$version"

  echo ">> Testing various feature sets..."
  rx cargo test
  rx cargo build --all --all-features --all-targets
  rx cargo build --no-default-features --features libm
  cargo clean

  if ! $extended_tests; then
    return
  fi
  
  lstrl_check_target wasm32-unknown-unknown build
}

lstrl_tidy() {
  rustup toolchain add stable --profile minimal
  rustup default stable

  rx cargo fmt --all --check
  rx cargo clippy --all-features --all-targets
}

. "$HOME/.cargo/env"

lstrl_tidy
lstrl_test_version stable true
lstrl_test_version beta true
lstrl_test_version nightly true
lstrl_test_version 1.63.0 false

