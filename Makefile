MAKEFLAGS	+= --silent
PROGRAM		:= tapa-micro-mailer

.PHONY : clean clippy fmt upgrade test release docker all

all: | prepare debug

test:
	cargo test

prepare:
	mkdir -p build/release
	mkdir -p build/debug

check:
	cargo check

clippy:
	cargo clippy

upgrade:
	cargo update
	cargo upgrade --workspace

fmt:
	cargo fmt

clean:
	rm -rf target
	rm -rf build

debug: | prepare
	cargo build
	cp target/debug/${PROGRAM} build/debug/${PROGRAM}

release: | prepare
	RUSTFLAGS="-C link-args=-s -C target-feature=+crt-static" cargo build --release
	cp target/release/${PROGRAM} build/release/${PROGRAM}

docker: | release
	docker build --pull -t docker.pkg.github.com/tapalogi/tapa-micro-mailer/tmm-local:dev .
