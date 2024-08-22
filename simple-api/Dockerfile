ARG BINARY_NAME=simple-api

FROM clux/muslrust:nightly AS build
ARG BINARY_NAME

# Build dummy main with the project's Cargo lock and toml
# This is a docker trick in order to avoid downloading and building
# dependencies when lock and toml not is modified.
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src \
    && echo "fn main() {print!(\"Dummy main\");} // dummy file" > src/main.rs
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN set -x && rm target/x86_64-unknown-linux-musl/release/deps/simple_api*

# copy app source tree & build for release
COPY src ./src
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN strip ./target/x86_64-unknown-linux-musl/release/$BINARY_NAME
RUN mkdir -p /out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /out/

# final base
FROM gcr.io/distroless/static:nonroot
ARG BINARY_NAME

# copy build artifact from the build stage
COPY --chown=nonroot:nonroot --from=build /out/$BINARY_NAME /
EXPOSE 8000
ENTRYPOINT ["/simple-api"]
