# cluster-image-logger

[![License](https://img.shields.io/github/license/nickgerace/cluster-image-logger?style=flat-square)](./LICENSE)
[![Docker Image Size (tag)](https://img.shields.io/docker/image-size/nickgerace/cluster-image-logger/unstable?style=flat-square)](https://hub.docker.com/r/nickgerace/cluster-image-logger/tags)

<!--
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/cluster-image-logger?label=version&style=flat-square)](https://github.com/nickgerace/cluster-image-logger/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/cluster-image-logger?style=flat-square)](https://crates.io/crates/cluster-image-logger)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/cluster-image-logger/merge/main?style=flat-square)](https://github.com/nickgerace/cluster-image-logger/actions?query=workflow%3Amerge+branch%3Amain)
-->

`cluster-image-logger` is a service that logs container images in your Kubernetes cluster.
Specifically, this service logs the "existence" of images based on pod creation and deletion events.
The "logging" part involves sending a string to STDOUT with the timestamp, log level, and a message indicating what images are new to the cluster, or are no longer being used by any pods in the cluster.

```
[user at hostname in ~]
% kubectl logs -n cluster-image-logger $(kubectl get pods -n cluster-image-logger --no-headers -o custom-columns=":metadata.name") --follow
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/coredns-coredns:1.8.0  [kube-system|coredns-854c77959c-wtqcc]
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/klipper-helm:v0.3.2  [kube-system|helm-install-traefik-p8sg9]
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/library-traefik:1.7.19  [kube-system|traefik-6f9cbd9bd4-5c6s6]
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/klipper-lb:v0.1.2  [kube-system|svclb-traefik-xn5k5]
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/local-path-provisioner:v0.0.14  [kube-system|local-path-provisioner-7c458769fb-wsdz5]
[2021-01-13T20:50:51Z INFO ] [+|image]  rancher/metrics-server:v0.3.6  [kube-system|metrics-server-86cbb8457f-hdccq]
[2021-01-13T20:50:51Z INFO ] [+|image]  nickgerace/cluster-image-logger:unstable  [cluster-image-logger|cluster-image-logger-deployment-5ccdc7f99b-rrcqt]
[2021-01-13T20:51:14Z INFO ] [+|image]  perl  [foo|pi-t5zl5]
[2021-01-13T20:51:15Z INFO ] [+|image]  nginx:1.14.2  [foo|nginx-deployment-66b6c48dd5-4g5nj]
[2021-01-13T20:51:35Z INFO ] [-|image]  perl  [foo|pi-t5zl5]
[2021-01-13T20:51:15Z INFO ] [-|image]  nginx:1.14.2  [foo|nginx-deployment-66b6c48dd5-4g5nj]
```

## WARNING: THIS REPOSITORY IS UNSTABLE UNTIL VERSION 1.0.0

Prerequisites to releasing version `1.0.0`...

- [ ] Helm install without cloning repository
- [ ] GitHub action for binary stability on `main`
- [ ] Publish stable Docker images
- [ ] Add to crates.io
- [ ] Tag as `1.0.0`
- [ ] Re-add unit tests

## Install

Clone the repository, and `cd` into it.
You can change the namespace name to whatever you prefer.

```bash
helm install -n cluster-image-logger --create-namespace --wait cluster-image-logger ./chart
```

## Follow Logs

Follow logs via the pod's STDOUT.

```bash
kubectl logs -n cluster-image-logger $(kubectl get pods -n cluster-image-logger --no-headers -o custom-columns=":metadata.name") --follow
```

When reading or parsing the logs, reference the following format...

```
[TIMESTAMP LOG_LEVEL ] [ADDED_OR_DELETED_SYMBOL|RESOURCE_TYPE]  IMAGE_NAME:IMAGE_TAG  [POD_NAMESPACE|POD_NAME]
```

### Why not use labels to follow logs?

Using the label selector does not provide an up-to-date stream. Following the logs of the pod itself is preferred.

## Uninstall

Uninstall the helm chart and delete the namespace to return to where you started.

```bash
helm uninstall -n cluster-image-logger cluster-image-logger
kubectl delete namespace cluster-image-logger
```

## Security Implications

The service can `get` and `watch` pods at a `cluster` scope.
This means that information can be read (not written) from any pod in any namespace.
Please look at the [chart](./chart) for more information.

## Changelog

Please look at [CHANGELOG.md](./CHANGELOG.md) for more information.
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
