#!/bin/bash
rustup toolchain install nightly && rustup component add rustfmt && cargo install cargo-expand && cargo expand --test it_works > it_works.rs