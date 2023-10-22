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

POST_DIR=~/.local/share/sblog/post
POST_MD_DIR=${POST_DIR}/md
POST_SUMMARY_DIR=${POST_DIR}/summary
install-testdata:
	mkdir -p ${POST_MD_DIR} ${POST_SUMMARY_DIR}
	cp -f ./testdate/*.md ./testdate/*.html ${POST_MD_DIR}
	cp -f ./testdate/*.summary ${POST_SUMMARY_DIR}

uninstall-testdata:
	rm -f ${POST_MD_DIR}/* ${POST_SUMMARY_DIR}/*

