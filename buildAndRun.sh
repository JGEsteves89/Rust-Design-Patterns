#!/bin/bash

MAIN_PATH=./main.rs
BUILD_PATH=./build

if [ ! -f "$MAIN_PATH" ]; then
    echo "$MAIN_PATH does not exist in the current folder."
    exit 1
fi
if [ ! -d "$BUILD_PATH" ]; then
    mkdir -p ./build;
fi

rustc "$MAIN_PATH" --out-dir "$BUILD_PATH"
if [ $? -eq 0 ]; then
    ${BUILD_PATH}/main
fi

