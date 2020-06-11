CARGO_TARGET_DIR?=$(CARGO_TARGET_DIR)

.PHONY: build
build:
	cargo build

.PHONY: test
test: build
	zathura -p ${CARGO_TARGET_DIR}/debug/ test.md