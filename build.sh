#!/usr/bin/env bash

cd $(dirname $0)

docker run --rm -it -v "$(pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry -v "$(pwd)/target/":/home/rust/src/target ekidd/rust-musl-builder:nightly-2021-01-01 sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/src/target

docker run --rm -it -v "$(pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry -v "$(pwd)/target/":/home/rust/src/target ekidd/rust-musl-builder:nightly-2021-01-01 cargo build --release
