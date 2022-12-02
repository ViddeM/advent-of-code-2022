FROM rustlang/rust:nightly
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Install musl-dev on Alpine to avoid error "ld: cannot find crti.o: No such file or directory"
RUN ((cat /etc/os-release | grep ID | grep alpine) && apk add --no-cache musl-dev || true) \
    && cargo install cargo-chef --locked \
    && rm -rf $CARGO_HOME/registry/