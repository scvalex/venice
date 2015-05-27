.PHONY: all build p

all: build

build:
	cargo build

p:
	permamake.sh **/*.rs
