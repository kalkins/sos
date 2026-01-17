#!/usr/bin/env bash

script_path="${BASH_SOURCE[0]}"
script_dir="$(dirname "$script_path")"
root_dir="$script_dir/.."
partition_dir="$root_dir/tmp/partitions/"
esp_partition_dir="$partition_dir/esp"
root_partition_dir="$partition_dir/root"
uefi_output_dir="$esp_partition_dir/efi/boot/"

uefi_loader_output="$root_dir/target/x86_64-unknown-uefi/debug/uefi_loader.efi"

cd "$root_dir" || exit 1

cargo build --target x86_64-unknown-uefi || exit 1

mkdir -p "$uefi_output_dir" || exit 1
mkdir -p "$root_partition_dir" || exit 1

cp "$uefi_loader_output" "$uefi_output_dir/bootx64.efi" || exit 1

qemu-system-x86_64 \
    -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_CODE.4m.fd \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_VARS.4m.fd \
    -drive format=raw,file=fat:rw:"$esp_partition_dir" \
    -drive format=raw,file=fat:rw:"$root_partition_dir"
