FROM viddem/cargo-chef-nightly AS chef
WORKDIR /usr/src/aoc
RUN cargo install cargo-chef --locked

FROM chef as planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /usr/src/aoc/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo +nightly chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo +nightly build --release --bin aoc

FROM alpine AS runtime
# COPY --from=builder /usr/src/aoc/target/release/aoc /usr/local/bin/aoc
COPY --from=builder /usr/src/aoc/target/release/aoc /usr/local/bin/aoc
RUN ls -lah /usr/local/bin/
COPY input.txt .
CMD ["aoc"]
