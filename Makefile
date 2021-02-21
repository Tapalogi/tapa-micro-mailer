MAKEFLAGS	+= --silent
PROGRAM		:= tapa-micro-mailer

.PHONY : all test prepare check clippy upgrade fmt run clean debug release docker

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

run:
	set -a; . ./local.env; set +a | cargo run

clean:
	rm -rf target
	rm -rf build

debug: | prepare
	cargo build
	cp target/debug/${PROGRAM} build/debug/${PROGRAM}

release: | prepare
	RUSTFLAGS="-C target-cpu=native -C link-args=-s" cargo build --release
	cp target/release/${PROGRAM} build/release/${PROGRAM}

docker:
	docker build --pull -t docker.pkg.github.com/tapalogi/tapa-micro-mailer/tmm-local:dev .
