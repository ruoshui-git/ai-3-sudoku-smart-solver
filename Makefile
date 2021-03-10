PY = python3

build: clean
	pex . \
    --python-shebang '/usr/bin/python3' \
    --not-zip-safe  \
    -v \
    -o dist/prtest.pex \
	-e prtest.main:main

clean:
	rm -rf prtest.egg-info
	rm -rf dist

    
.PHONY: clean build