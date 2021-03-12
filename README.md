# AI 2 Sudoku Swap

## Running Instructions

To build for development:
```
make
```

To run in dev mode:
```
python3 runner.py [args]
```

To build for submission (prod):
```
make build-release
```
This will compile rust in release mode and run `pex` to package everything into `/dist/app.pex`.

To run submission:
```
python3 dist/app.pex [args]
```

Or you can run as executable if you have the right permission set:
```
./dist/app.pex [args]
```

# Some Folder Structures

`/sudoku_swap` is a python package that acts as a shim to run Rust code.

Rust code is in  `/rust`.

`crate::run` is imported into python and ran. This essentially acts as the `main` fn of a program.

Rust is compiled into a library that can be imported from Python.
Because Mr. Brooks's program tester runs on Ubuntu, the Rust portion is compiled for Linux (into .so file).