# https://github.com/devcontainers/images/tree/main/src/rust
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# add wasm32-unknown-unknown target 
# and install nightly toolchain and rustfmt
RUN rustup target add wasm32-unknown-unknown && \
    rustup toolchain install nightly && \ 
    rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt

RUN cargo install just leptosfmt trunk

RUN sudo chown -R vscode /usr/local/cargo
