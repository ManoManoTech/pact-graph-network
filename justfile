#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

alias t := tests

alias l := lints

# Display all the target
default:
    just --list

# Lint the project
lints: _clippy _fmt

_clippy:
    cargo clippy --all --all-targets -- -D warnings

_fmt:
    cargo fmt --all -- --check

# Runs tests on the project
tests:
    cargo test --all

# Builds a debug  binary for current os/arch
build:
    cargo build --bin pact-graph-network --locked

# Release a binary for current os/arch ready for the production
release:
    cargo build --bin pact-graph-network --locked --release