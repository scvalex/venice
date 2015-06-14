.PHONY: all build p test

all: build test

build:
	cargo build

test:
	cargo test

p:
	permamake.sh **/*.rs
