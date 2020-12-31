FROM ekidd/rust-musl-builder:stable AS build
WORKDIR /build/
COPY Cargo.toml Cargo.lock .
COPY src/ src/
RUN cargo build --release

FROM scratch
WORKDIR /bin/
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/image-logger .
ENTRYPOINT ["/bin/image-logger"]
