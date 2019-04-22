RS_DIR = src
RS_SRC = $(RS_DIR)/$(shell find src -name *.rs)
DEBUG_BIN = target/debug/move_db
RELEASE_BIN = target/release/move_db
WASM_SRC = $(shell find frontend/src -name '*.rs')
WASM_BIN_DIR = frontend/target/wasm32-unknown-unknown/release
WASM_BIN = $(WASM_BIN_DIR)/frontend.wasm
JS_BIN = $(WASM_BIN_DIR)/frontend.js
STATIC = $(shell find frontend/static | tail -n +2)

.PHONY: debug
debug: rust_debug wasm

.PHONY: release
release: rust_release wasm

.PHONY: run
run: debug
	cargo run

.PHONY: rust_debug
rust_debug: $(DEBUG_BIN)

.PHONY: rust_release
rust_release: $(RELEASE_BIN)

.PHONY: static
static: $(STATIC)
	mkdir -p static
	cp -R frontend/static/* static/

.PHONY: wasm
wasm: $(WASM_BIN) $(JS_BIN) static
	cp $(WASM_BIN) static/
	cp $(JS_BIN) static/

$(DEBUG_BIN): $(RS_SRC)
	cargo build

$(RELEASE_BIN): $(RS_SRC)
	cargo build --release

$(WASM_BIN): $(WASM_SRC)
	cd frontend && \
	cargo web build --release

$(JS_BIN): $(WASM_BIN)
