# syntax = docker/dockerfile:1.3

FROM rust:1.74-bookworm as builder
ARG DAY_NUM
WORKDIR /aoc2023
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/aoc2023/target \
    cargo build --locked --release --bin day$DAY_NUM && cp /aoc2023/target/release/day$DAY_NUM /bin/day$DAY_NUM

FROM gcr.io/distroless/cc-debian12:nonroot
ARG DAY_NUM
COPY --from=builder /bin/day$DAY_NUM /aoc
ENTRYPOINT ["/aoc"]
