#!/bin/bash

all:
	cargo build --release

build:
	cargo build

build-release:
	cargo build --release

run:
	RUST_LOG=error,warn,info,debug cargo run

run-release:
	RUST_LOG=error,warn,info,debug cargo run --release

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental/*

clean:
	cargo clean

install:
	cp -f ./target/release/apisvr ~/bin
