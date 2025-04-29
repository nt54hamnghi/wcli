fmt:
	cargo +nightly fmt && leptosfmt ./**/*.rs

alias f := fmt

check:
    cargo clippy --all-targets -- -D clippy::all -W clippy::pedantic

alias c := check

fix:
    cargo clippy --fix 

dev:
    RUSTFLAGS="--cfg erase_components" trunk serve --port 3000 --open