alias r := run

default: 
	just --list

clear:
	clear

run: clear
	cargo watch -c -x run

build: clear
	cargo build --release
