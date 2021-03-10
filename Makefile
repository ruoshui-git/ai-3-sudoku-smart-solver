PY = python3
PY_PACKAGE = sodoku_swap

RUST_DIR = rust

DIST_DIR = dist
OUT = app.pex


TARGET = $(DIST_DIR)/$(OUT)
RUST_TARGET = $(PY_PACKAGE)/rust.so

build-dev: rust

build-rel: clean rust pex


run-rel:
	$(PY) $(TARGET)

run-dev:
	$(PY) main.py

rust: 
	cd $(RUST_DIR) && cargo build
	cp $(RUST_DIR)/target/debug/lib$(PY_PACKAGE).so $(RUST_TARGET)

pex:
	pex . \
    --python-shebang '/usr/bin/python3' \
    --not-zip-safe  \
    -o $(DIST_DIR)/$(OUT) \
	-e $(PY_PACKAGE).shim:main
    # -v \

clean:
	rm -rf $(PY_PACKAGE).egg-info $(DIST_DIR) $(RUST_TARGET)

.PHONY: clean build rust pex