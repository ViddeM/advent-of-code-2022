# FROM rust:1.65 as builder
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/aoc
COPY . .
RUN cargo install --path .

# FROM rust:1.65-slim
FROM rustlang/rust:nightly-slim
COPY --from=builder /usr/local/cargo/bin/aoc /usr/local/bin/aoc
COPY input.txt .
CMD ["aoc"]