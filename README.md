# kimager

[![License](https://img.shields.io/github/license/nickgerace/kimager?style=flat-square)](./LICENSE)
[![Docker Image Size (tag)](https://img.shields.io/docker/image-size/nickgerace/kimager/unstable?style=flat-square)](https://hub.docker.com/r/nickgerace/kimager/tags)

<!--
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/kimager?label=version&style=flat-square)](https://github.com/nickgerace/kimager/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/kimager?style=flat-square)](https://crates.io/crates/kimager)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/kimager/merge/main?style=flat-square)](https://github.com/nickgerace/kimager/actions?query=workflow%3Amerge+branch%3Amain)
-->

`kimager` is a service that logs the existence of container images in a Kubernetes cluster.
Container image "existence" is determined by pod creation and deletion events.
Container image "loggging" involves using the [log crate](https://crates.io/crates/log) to record the timestamp, log level, and a message indicating what image is new to the cluster, or is no longer being used by any pods in the cluster.

## Installation

Clone the repository, and `cd` into it.
You can change the namespace name to whatever you prefer.

```bash
helm install -n kimager-system --create-namespace --wait kimager ./chart
```

You can also set the log level with `--set logLevel`.
This sets the `RUST_LOG` environment variable in the container (e.g. `DEBUG`).

## Usage

You can follow logs via the container's STDOUT within the pod. You may have to run the nested command (below) separately depending on your system.

```bash
kubectl logs -n kimager-system $(kubectl get pods -n kimager-system --no-headers -o custom-columns=":metadata.name") --follow
```

When reading or parsing the logs, reference the following format:

```
[TIMESTAMP LOG_LEVEL ] [ADDED_OR_DELETED_SYMBOL|RESOURCE_TYPE]  IMAGE_NAME:IMAGE_TAG  [POD_NAMESPACE|POD_NAME]
```

### Why not use labels to follow logs?

Using the label selector does not provide an up-to-date stream.
Following the logs of the pod itself is preferred.

## Uninstallation

Uninstall the helm chart and delete the namespace to return to where you started.
This will clean up all artifacts from installation and usage.

```bash
helm uninstall -n kimager-system kimager
kubectl delete namespace kimager-system
```

## Security Implications

This service aims to be as secure as possible, and it is able to `get` and `watch` pods at a `cluster` scope.
This means that information can be read (not written) from any pod in any namespace.
Please look at the [chart](./chart) for more information.

## Other Documentation

- **[CHANGELOG.md](./CHANGELOG.md):** follows the [Keep a Changelog](https://keepachangelog.com/) format
- **[DEVELOPING.md](./DEVELOPING.md):** developer tips, tricks, and notes
- **[RELEASE.md](./RELEASE.md):** release process notes

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)

## Reaching "Stable" (Version 1.0.0)

The following prerequisites must be met before version `1.0.0` is released:

- [ ] Helm install without needing to clone repository
- [ ] GitHub action that runs `fmt` and `clippy` for `x86_64-unknown-linux-gnu`, and `build` for `x86_64-unknown-linux-musl`
- [ ] Publish a `latest`, and at least one "released" (e.g. `0.1.0`), Dockerhub image
- [ ] Publish to crates.io (`0.1.0` tag first)
- [ ] Re-add unit tests

## Special Thanks

Description | Status
--- | --- 
[@joshimoo](https://github.com/joshimoo) for the recommendation on using undirected graphs | in progress
[@steveklabnik](https://github.com/steveklabnik) and many members of the Rust community on Twitter for discussions around node weights in undirected graphs | in progress, but there's some influence in the current design
