# Changelog

All notable changes to this project will be documented in this file.
All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
<!--The latest version contains all changes.-->

### Added

- Base contents
- Description and example output to README
- Loop for watcher in case timeouts occur
- Security considerations to README
- Two Docker Hub tags: `unstable` for users and `broken` for development

### Changed

- Combined use statements in each file
- `env_logger` location from `main.rs` to `lib.rs`
- Hasher to only be created once during add events
- Name from "image-logger" to "cluster-image-logger"

### Removed

- File header comments
- Module path from `env_logger` output
