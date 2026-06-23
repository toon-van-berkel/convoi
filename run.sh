#!/usr/bin/env bash
set -e

echo "Building Convoi..."
cargo bootimage

echo "Finding boot image..."
BOOTIMAGE=$(find "$HOME/convoi-target" -type f -name "bootimage-convoi.bin" | head -n 1)

if [ -z "$BOOTIMAGE" ]; then
    echo "Could not find boot image."
    exit 1
fi

echo "Running Convoi..."
echo "Boot image: $BOOTIMAGE"

qemu-system-x86_64 -drive format=raw,file="$BOOTIMAGE" -display curses
