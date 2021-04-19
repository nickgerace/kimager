# Changelog

All notable changes to this project will be documented in this file.
All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
<!--The latest version contains all changes.-->

### Added

- `DEVELOPING.md` file
- `RELEASE.md` file
- `rustfmt.toml` file for automatic changes that would have been enforced manually
- Base contents
- Description and example output to README
- Elements from `lib.rs` before the removal of `watcher.rs`
- Loop for watcher in case timeouts occur
- Security considerations to README
- Two Docker Hub tags: `unstable` for users and `broken` for development

### Changed

- `ekidd/rust-musl-builder` to `clux/muslrust` due to smaller, final binary sizes and newer Rust base images
- `env_logger` location from `main.rs` to `lib.rs`, and then back to `main.rs` (best used in an application; not a libary)
- `eyre` to `anyhow` since we only needed the `Result` type from the crate for basic error reporting
- `kube::api::Meta` to `kube::api::Resource` due to deprecation
- Combined `lib.rs` and `watcher.rs` functions to reduce runtime complexity
- Combined use statements in each file
- Hasher to only be created once during add events
- Kubernetes client creation to `main.rs` and out of the library entirely
- Name from "image-logger" to "cluster-imager-logger" to "kimager"
- Names in Helm chart to be hardcoded to "kimager"
- RBAC in Helm chart into different files

### Removed

- `watcher.rs` in favor of using `lib.rs` for the watcher loop
- Execution example from README with STDOUT
- File header comments
- Module path from `env_logger` output
