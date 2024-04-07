SHELL := /usr/bin/env bash
COMPILE_DIR := ./target
NES_TEST_ROM_SHA256SUM := e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
NES_TEST_ROM_URL := https://github.com/christopherpow/nes-test-roms/raw/master/other/nestest.nes

all:
	$(MAKE) create-compile-dir
	$(MAKE) test
	$(MAKE) download-nes-test-rom
	$(MAKE) compile-main
	$(MAKE) run-main

create-compile-dir:
	mkdir -p $(COMPILE_DIR)

test:
	cargo test

download-nes-test-rom:
	curl -L -o $(COMPILE_DIR)/nestest.nes $(NES_TEST_ROM_URL)

compile-main:
	rustc --out-dir $(COMPILE_DIR) src/main.rs

run-main:
	$(COMPILE_DIR)/main

clean:
	rm -rf $(COMPILE_DIR)
