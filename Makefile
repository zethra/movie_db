DEBUG_DIR = dist/debug
RELEASE_DIR = dist/release
DEBUG_STATIC_DIR = dist/debug/static
RELEASE_STATIC_DIR = dist/release/static
DEBUG_BIN = dist/debug/movie_db
RELEASE_BIN = dist/release/movie_db

RS_SRC = $(wildcard *.rs)
WEB_DIR = web
ELM_SRC_DIR = $(WEB_DIR)/src
HTML_SRC = $(wildcard *.html)

.PHONY: debug
debug: rust_debug html_debug

.PHONY: run
run: debug
	cd dist/debug && \
	./movie_db

.PHONY: rust_debug
rust_debug: $(DEBUG_BIN)

.PHONY: rust_release
rust_release: $(RELEASE_BIN)

.PHONY: html_debug
html_debug: $(DEBUG_STATIC_DIR)/index.html


$(DEBUG_DIR):
	mkdir -p $@

$(RELEASE_DIR):
	mkdir -p $@

$(DEBUG_BIN): $(RS_SRC)
	cargo build
	cp target/debug/movie_db $@

$(RELEASE_BIN): $(RS_SRC)
	cargo build --release
	cp target/release/movie_db $@

$(DEBUG_STATIC_DIR):
	mkdir -p $@

$(RELEASE_STATIC_DIR):
	mkdir -p $@

$(DEBUG_STATIC_DIR)/index.html: $(ELM_SRC_DIR)/Main.elm
	cd $(WEB_DIR) && \
	elm make --output=../$@ src/Main.elm