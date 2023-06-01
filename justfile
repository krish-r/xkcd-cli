default:
	@just --list --unsorted

build:
	cargo build

release: 
	cargo build --release

test: 
	cargo test

insta:
	cargo insta test
