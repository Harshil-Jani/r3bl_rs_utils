#!/usr/bin/env fish

# 1. Make sure to install cargo-outdated via `cargo install --locked cargo-outdated`.
# More info about cargo-outdated: https://crates.io/crates/cargo-outdated

set -l folders . core macro redux tui

for folder in $folders
    pushd $folder
    echo (set_color brmagenta)"≡ Running cargo fmt --all in '$folder' .. ≡"(set_color normal)
    cargo fmt --all
    popd
end
