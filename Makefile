MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
TAG:=unstable
IMAGE:=nickgerace/image-logger:$(TAG)
NAMESPACE:=image-logger

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
	docker push $(IMAGE)

docker-test: docker-build
	@printf "This should fail... We are only trying to see if the binary works at runtime.\n"
	-docker run $(IMAGE)

docker-build:
	cd $(MAKEPATH); docker build -t $(IMAGE) .

install: uninstall
	helm install -n $(NAMESPACE) --create-namespace image-logger $(MAKEPATH)/chart

install-debug: uninstall
	helm install -n $(NAMESPACE) --create-namespace image-logger $(MAKEPATH)/chart --set logLevel=DEBUG

uninstall:
	-helm uninstall -n $(NAMESPACE) image-logger
	-kubectl delete namespace $(NAMESPACE)

logs:
	kubectl logs -n $(NAMESPACE) $(shell kubectl get pods -n $(NAMESPACE) --no-headers -o custom-columns=":metadata.name") --follow
