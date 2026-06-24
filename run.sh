#!/usr/bin/env bash
set -e
set -o pipefail

echo "Building Convoi..."

cargo bootimage -Zjson-target-spec 2>&1 | tee /tmp/convoi-build.log

echo "Finding boot image..."

BOOTIMAGE=$(grep -oE '/[^` ]*/bootimage-[^` ]+\.bin' /tmp/convoi-build.log | tail -n 1)

if [ -z "$BOOTIMAGE" ]; then
    echo "Could not read boot image path from build output."
    echo "Trying fallback search..."

    BOOTIMAGE=$(find /home/toon /media/sf_convoi "$PWD" -type f -name "bootimage-*.bin" 2>/dev/null | tail -n 1)
fi

if [ -z "$BOOTIMAGE" ]; then
    echo "Could not find boot image."
    exit 1
fi

echo "Running Convoi..."
echo "Boot image: $BOOTIMAGE"

qemu-system-x86_64 -drive format=raw,file="$BOOTIMAGE" -display curses