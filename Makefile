PY = python3
PY_PACKAGE = pyshim

RUST_DIR = rust

DIST_DIR = dist
OUT = app.pex


TARGET = $(DIST_DIR)/$(OUT)
RUST_TARGET = $(PY_PACKAGE)/rust.so

COPY_RUST_LIB = cp $(RUST_DIR)/target/debug/lib$(PY_PACKAGE).so $(RUST_TARGET)

build: build-rust

run:
	$(PY) main.py

build-release: clean build-rust-release pex submit

run-release:
	$(PY) $(TARGET)

build-rust: 
	cd $(RUST_DIR) && cargo build
	$(COPY_RUST_LIB)

build-rust-release:
	cd $(RUST_DIR) && cargo build --release
	$(COPY_RUST_LIB)

pex:
	pex . \
    --python-shebang '/usr/bin/python3' \
    --not-zip-safe  \
    -o $(DIST_DIR)/$(OUT) \
	-e $(PY_PACKAGE).shim:main
    # -v \

clean:
	rm -rf $(PY_PACKAGE).egg-info $(DIST_DIR) $(RUST_TARGET)

submit: $(TARGET)
	cp $(TARGET) main.txt

.PHONY: clean build build-release run run-release build-rust build-rust-release pex