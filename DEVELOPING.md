# Developing

This document contains tips, tricks, workflows, etc. for developing within this repository.

## Formatting

This repository includes a `rustfmt.toml` file, which is only used for organizing imports and newline characters.
The goal is to provide extra formatting that would otherwise be performed manually, and it is *not* to deviate from [rust-lang/rustfmt](https://github.com/rust-lang/rustfmt).

The TOML file includes [imports_granularity](https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#imports_granularity), which is an unstable option that requires a nightly toolchain.
With a nightly toolchain installed, you can add the nightly `rustfmt` component.

```sh
rustup component add rustfmt --toolchain nightly
```

Now, you can format the code with all settings from the `rustfmt.toml` file.

```sh
cargo +nightly fmt
```

## Building

Execute the following commands to build `kimager`:

```sh
cargo update
cargo +nightly fmt
cargo clippy
cargo build
```

On compatible platforms, you can reduce the binary size by using `strip` to remove debug symbols.

```sh
du -h $BINARY
strip $BINARY
du -h $BINARY
```

## Testing with the Chart

First, we need to publish and image for our [Helm chart](./chart).
You can use a local, or remote, personal container repository for testing the end-to-end workflow.

```sh
docker build -t $IMAGE .
docker push $IMAGE
```

Now, we can install our chart and use our development image.

```sh
helm install -n kimager-system \
	--create-namespace --wait \
	--set image.repository=$REPOSITORY \
	--set image.tag=$TAG \
	./chart
```

Refer to the [README](./README.md) for more instructions on usage.

## Where is the Makefile?

This project aims to be cross-platform development friendly, and leverages the extensive options of `cargo` to do so.
In addition, this project aims to fill gaps with documentation.
However, this approach is subject to change if it does not suffice.