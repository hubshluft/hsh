SRC_DIR := src
OUT_DIR := target
BIN_NAME := hsh
OUT_FILE := $(OUT_DIR)/$(BIN_NAME)

build:
	cargo build

run: build
	cargo run

clean:
	cargo clean

.PHONY: build run clean
