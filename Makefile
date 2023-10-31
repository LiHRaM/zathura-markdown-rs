CARGO_TARGET_DIR    ?= ./target
ZATHURA_PLUGINS_DIR ?= /usr/lib/zathura

.PHONY: build
build:
	cargo build

.PHONY: test
test: build
	zathura -p "${CARGO_TARGET_DIR}/debug/" README.md

.PHONY: install
install:
	install -D "${ZATHURA_PLUGINS_DIR}"
	install -m 644 "${CARGO_TARGET_DIR}/release/*.so" "${ZATHURA_PLUGINS_DIR}"
