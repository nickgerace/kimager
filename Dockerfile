FROM clux/muslrust:stable AS build
WORKDIR /build/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/ src/
RUN cargo build --release && strip /build/target/x86_64-unknown-linux-musl/release/kimager

FROM scratch
WORKDIR /bin/
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/kimager .
ENTRYPOINT ["/bin/kimager"]
