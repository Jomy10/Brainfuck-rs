#!/usr/bin/env zsh

# This script builds for all 3 platforms and gzips them

cargo build --release --target x86_64-apple-darwin
docker run --rm -v `pwd`:/app rust/linux-aarch64
docker run --rm -v `pwd`:/app rust/windows-x86_64

mv "target/x86_64-apple-darwin/release/brainfuck" "target/x86_64-apple-darwin/release/bf"
mv "target/aarch64-unknown-linux-gnu/release/brainfuck" "target/aarch64-unknown-linux-gnu/release/bf"
mv "target/x86_64-pc-windows-gnu/release/brainfuck.exe" "target/x86_64-pc-windows-gnu/release/bf.exe"

tar -czf "build/bf-macos-x86_64.tar.gz" -C "target/x86_64-apple-darwin/release" "bf"
tar -czf "build/bf-linux-aarch65.tar.gz" -C "target/aarch64-unknown-linux-gnu/release" "bf"
tar -czf "build/bf-windows-x86_64.tar.gz" -C "target/x86_64-pc-windows-gnu/release" "bf.exe"
