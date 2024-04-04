#!/usr/bin/env sh
echo "----- Building target: x86_64-unknown-linux-gnu -----"
cargo build --target=x86_64-unknown-linux-gnu
echo "----- Building finished -----"
echo ""

echo "----- Building target: x86_64-pc-windows-gnu -----"
cargo build --target=x86_64-pc-windows-gnu
echo "----- Building finished -----"
echo ""

echo "----- Building target: x86_64-apple-darwin -----"
cargo build --target=x86_64-apple-darwin
echo "----- Building finished -----"
echo ""
