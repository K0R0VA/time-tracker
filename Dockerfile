FROM rust:latest AS builder
WORKDIR /app
COPY ./ .
RUN cargo build --release

FROM ubuntu
WORKDIR /app
COPY --from=builder /app/target/release/time-tracker-bot ./
CMD "/bin/bash"
