# Release

This document contains all information related to release.

## Preparation

- Change the version in `Cargo.toml` to the new tag
- Update `version` and `appVersion` in `Chart.yaml` to the new tag
- Run the commands and verify that everything looks/works as expected:

```sh
cargo update
cargo +nightly fmt --all -- --check
cargo clippy -- -D warnings
cargo test -- --nocapture
cargo doc --open
cargo build --release
```

- Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
- Test the publishing workflow within each crate:

```sh
cargo publish --dry-run
```

Finally, push (or merge) the preparation commit.

## Tagging and Publishing

Once the prepation commit has been pushed (or merged) into `main`, execute the following commands:

```sh
git tag $TAG
git push --tags origin main
```

Now, publish the crate.

```sh
cargo publish
```

Check `crates.io` and `docs.rs` afterwards via the [crate's page](https://crates.io/crates/kimager).

## Building and Pushing the Image

On a branch tracking the new tag, execute the following commands at the root of the repository:

```sh
docker build -t $IMAGE:$TAG .
docker push $IMAGE:$TAG
docker tag $IMAGE:$TAG $IMAGE:latest
docker push $IMAGE:latest
```
