# image-logger

[![License](https://img.shields.io/github/license/nickgerace/image-logger?style=flat-square)](./LICENSE)
[![Docker Image Size (tag)](https://img.shields.io/docker/image-size/nickgerace/image-logger/unstable?style=flat-square)](https://hub.docker.com/r/nickgerace/image-logger/tags)

<!--
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/image-logger?label=version&style=flat-square)](https://github.com/nickgerace/image-logger/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/image-logger?style=flat-square)](https://crates.io/crates/image-logger)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/image-logger/merge/main?style=flat-square)](https://github.com/nickgerace/image-logger/actions?query=workflow%3Amerge+branch%3Amain)
-->

`image-logger` is a service that logs container images in your Kubernetes cluster.

## WARNING: THIS REPOSITORY IS UNSTABLE UNTIL VERSION 1.0.0

Prerequisites to version `1.0.0`...

- [ ] Helm install without cloning repository
- [ ] GitHub action for binary building
- [ ] Publish stable Docker images
- [ ] Add to crates.io
- [ ] Tag as `1.0.0`
- [ ] Re-add unit tests

## Install

Clone the repository, and `cd` into it.
You can change the namespace name to whatever you prefer.

```bash
helm install -n image-logger --create-namespace image-logger ./chart
```

## Follow Logs

Follow logs via the pod's STDOUT.

```bash
kubectl logs -n image-logger $(kubectl get pods -n image-logger --no-headers -o custom-columns=":metadata.name") --follow
```

### Why not use labels to follow logs?

Using the label selector does not provide an up-to-date stream. Following the logs of the pod itself is preferred.

## Uninstall

Uninstall the helm chart and delete the namespace to return to where you started.

```bash
helm uninstall -n image-logger image-logger
kubectl delete namespace image-logger
```

## Changelog

Please check out [CHANGELOG.md](./CHANGELOG.md) for more information.
It follows the [Keep a Changelog](https://keepachangelog.com/) format.

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)

## Special Thanks

- [@emk](https://github.com/emk) for the [rust-musl-builder](https://github.com/emk/rust-musl-builder) *(Status: fully implmented)*
- [@joshimoo](https://github.com/joshimoo) for the recommendation on using undirected graphs *(Status: implmented in an out-of-tree-build)*
- [@steveklabnik](https://github.com/steveklabnik) and many members of the Rust community on Twitter for discussions around node weights in undirected graphs *(Status: implmented in an out-of-tree-build with some influence in the current design)*
