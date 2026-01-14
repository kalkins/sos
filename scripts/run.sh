#!/usr/bin/env bash

script_path="${BASH_SOURCE[0]}"
script_dir="$(dirname "$script_path")"
root_dir="$script_dir/.."

cd "$root_dir" || exit 1

cargo build --target x86_64-unknown-uefi || exit 1

mkdir -p "$root_dir/tmp/esp/efi/boot/" || exit 1

cp "$root_dir/target/x86_64-unknown-uefi/debug/uefi_loader.efi" "$root_dir/tmp/esp/efi/boot/bootx64.efi" || exit 1

qemu-system-x86_64 \
    -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_CODE.4m.fd \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_VARS.4m.fd \
    -drive format=raw,file=fat:rw:"$root_dir/tmp/esp"
