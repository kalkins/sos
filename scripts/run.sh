#!/usr/bin/env bash

script_path=$(realpath "${BASH_SOURCE[0]}")
script_dir="$(dirname "$script_path")"
root_dir=$(dirname "$script_dir")
partition_dir="$root_dir/tmp/partitions/"
esp_partition_dir="$partition_dir/esp"
root_partition_dir="$partition_dir/root"
uefi_output_dir="$esp_partition_dir/efi/boot/"

bootloader_project="$root_dir/boot/uefi_loader"
kernel_project="$root_dir/kernel"

bootloader_output="$root_dir/target/x86_64-unknown-uefi/debug/uefi_loader.efi"
kernel_output="$root_dir/target/x86_64-sos/debug/kernel"

cd "$bootloader_project" || exit 1
cargo build || exit 1

cd "$kernel_project" || exit 1
cargo build || exit 1

mkdir -p "$uefi_output_dir" || exit 1
mkdir -p "$root_partition_dir" || exit 1

cp "$bootloader_output" "$uefi_output_dir/bootx64.efi" || exit 1
cp "$kernel_output" "$root_partition_dir/boot/kernel" || exit 1

qemu-system-x86_64 \
    -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_CODE.4m.fd \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/x64/OVMF_VARS.4m.fd \
    -drive format=raw,file=fat:rw:"$esp_partition_dir" \
    -drive format=raw,file=fat:rw:"$root_partition_dir"
