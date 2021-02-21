# --------------- #
# Stage 1 - Build #
# --------------- #

## Context Transfer
FROM rust:1.50-buster as builder
RUN mkdir /app
WORKDIR /app
COPY . .

## Additional System Packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libzstd-dev libsass-dev make cmake \
    ninja-build yasm nasm libsasl2-dev

## Package Caching
RUN cargo fetch

## Final Binary
RUN RUSTFLAGS="-C link-args=-s" cargo build --release

# --------------- #
# Stage 2 - Final #
# --------------- #

FROM debian:buster-slim
LABEL maintainer="Aditya Kresna <kresna@tapalogi.com>"

RUN mkdir /app
WORKDIR /app
COPY --from=builder /app/target/release/tapa-micro-mailer tapa-micro-mailer

CMD [ "./tapa-micro-mailer" ]
