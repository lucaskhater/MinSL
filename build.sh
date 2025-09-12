#!/bin/bash

set -e

help() {
    echo "Usage: ./build.sh [OPTIONS]"
    echo
    echo "Builds the shell crate with minimal overhead."
    echo
    echo "OPTIONS:"
    echo "  -c    Clean the build directory (recommended to use FIRST if combining options)"
    echo "  -d    Build in debug mode (output in ./target/x86_64-unknown-linux-gnu/debug)"
    echo "  -r    Build in release mode (output in ./target/x86_64-unknown-linux-gnu/release)"
    echo
    echo "EXAMPLES:"
    echo "  ./build.sh -cr       Clean and then build release (recommended)"
    echo "  ./build.sh -d        Just build debug"
    echo "  ./build.sh -r        Just build release"
    echo
    echo "NOTE: If you use -c (clean), it should come first to avoid deleting a fresh build."
}

args=""
while getopts ":cdrh" option; do
    arg_string+="$option"
done

OPTIND=1

if [[ "$arg_string" == *"c"* && "${arg_string:0:1}" != "c" ]]; then
    echo "[!] Warning: You used the clean flag (-c), but it wasn't the first option."
    echo "    This may delete artifacts created by subsequent build steps."
    read -p "Are you sure you want to proceed? [y/N]: " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo "Aborting."
        exit 1
    fi
fi

OPTIND=1

run=false

while getopts ":cdrh" opt; do
    case $opt in
        c)
            echo "[*] Cleaning..."
            cargo clean
            run=true
            ;;
        d)
            echo "[*] Building debug..."
            cargo +nightly build -Z build-std=core,compiler_builtins
            run=true
            ;;
        r)
            echo "[*] Building release..."
            cargo +nightly build -r -Z build-std=core,compiler_builtins
            run=true
            ;;
        h)
            help
            exit 0
            ;;
        *)
            echo "Invalid option: -$OPTARG" >&2
            help
            exit 1
            ;;
    esac
done

if [ "$run" = false ]; then
    help
    exit 0
fi
