RUSTC := rustc
SRC_DIR := src
OUT_DIR := target
BIN_NAME := hsh
SRC_FILES := $(wildcard $(SRC_DIR)/*.rs)
OUT_FILE := $(OUT_DIR)/$(BIN_NAME)

build:
	$(RUSTC) $(SRC_FILES) -o $(OUT_FILE)

run: build
	$(OUT_FILE)

clean:
	rm -rf $(OUT_DIR)

.PHONY: build run clean
