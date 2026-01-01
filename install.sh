#!/bin/bash
echo "Building Protonhax GUI..."
cargo build --release
mkdir -p ~/.local/bin
cp target/release/protonhhaxora ~/.local/bin/
echo "Installation complete! You can now run 'protonhax-gui' or find it in your menu."
