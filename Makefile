MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=cluster-image-logger
NAMESPACE:=$(NAME)
IMAGE_PREFIX:=nickgerace/$(NAME)

BROKEN_TAG:=broken
BROKEN_IMAGE:=$(IMAGE_PREFIX):$(BROKEN_TAG)

UNSTABLE_TAG:=unstable
UNSTABLE_IMAGE:=$(IMAGE_PREFIX):$(UNSTABLE_TAG)

run: pre-build
	cd $(MAKEPATH); cargo run

debug: pre-build
	cd $(MAKEPATH); RUST_LOG=debug cargo run

build: pre-build
	cd $(MAKEPATH); cargo build --release

pre-build:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fmt
	cd $(MAKEPATH); cargo clippy
	cd $(MAKEPATH); cargo test

doc:
	cd $(MAKEPATH); cargo doc --open

unused:
	cd $(MAKEPATH); cargo +nightly udeps

docker-push: docker-build
	docker push $(UNSTABLE_IMAGE)

docker-test: docker-build
	@printf "This should fail... We are only trying to see if the binary works at runtime.\n"
	-docker run $(UNSTABLE_IMAGE)

docker-build:
	cd $(MAKEPATH); docker build -t $(UNSTABLE_IMAGE) .

docker-push-dev: docker-build-dev
	docker push $(BROKEN_IMAGE)

docker-test-dev: docker-build-dev
	@printf "This should fail... We are only trying to see if the binary works at runtime.\n"
	-docker run $(BROKEN_IMAGE)

docker-build-dev:
	cd $(MAKEPATH); docker build -t $(BROKEN_IMAGE) .

install: uninstall
	helm install -n $(NAMESPACE) --create-namespace --wait $(NAME) $(MAKEPATH)/chart

install-dev: uninstall
	helm install -n $(NAMESPACE) --create-namespace --wait --set image.tag=broken $(NAME) $(MAKEPATH)/chart

install-debug: uninstall
	helm install -n $(NAMESPACE) --create-namespace --wait $(NAME) $(MAKEPATH)/chart --set logLevel=DEBUG

uninstall:
	-helm uninstall -n $(NAMESPACE) $(NAME)
	-kubectl delete namespace $(NAMESPACE)

logs:
	kubectl logs -n $(NAMESPACE) $(shell kubectl get pods -n $(NAMESPACE) --no-headers -o custom-columns=":metadata.name") --follow
