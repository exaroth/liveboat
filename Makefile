prog :=liveboat
target_t :=x86_64-unknown-linux-musl
bin_name :=liveboat-linux-musl 
debug ?=

$(info debug is $(debug))

ifdef debug
  release :=--target=$(target_t)
  target :=debug
  extension :=-debug
else
  release :=--release --target=$(target_t)
  target :=release
  extension :=
endif

CARGO=cargo
CARGO_TEST_FLAGS=--workspace

all: build install
.DEFAULT_GOAL: help
help:
	@echo "make setup"
	@echo "		  install dependencies for the project"
	@echo "make build"
	@echo "		  build the liveboat binary"
	@echo "make install"
	@echo "		  install the binary"
	@echo "make test"
	@echo "		  run tests"
	@echo "make setup-default-template-dev"
	@echo "		  setup default template for development (node required)"
	@echo "make build-default-template"
	@echo "		  build default template from source and update dist (node required)"

.PHONY: setup
setup:
	git submodule update --init
	rustup target add ${target_t}

.PHONY: build
build:
	TARGET=$(target_t) BIN_NAME=$(bin_name) cargo build $(release)

.PHONY: install
install:
	sudo cp target/$(target_t)/$(target)/$(prog) /usr/local/bin/$(prog)$(extension)

.PHONY: test
test:
	$(CARGO) test $(CARGO_TEST_FLAGS) --no-fail-fast src;

.PHONY: setup-default-template-dev
setup-default-template-dev:
	cargo build;
	cd ./templates/default/src/include && rm -Rf ./node_modules package-lock.json
	./target/debug/liveboat --template-path templates/default/src templates/default/src/include && git restore ./templates/default/src
	cd ./templates/default/src/include && npm install && npm run dev

.PHONY: build-default-template
build-default-template:
	cd ./templates/default/src/include && npm run build
	cd ./templates/default/src/include && cp -Rf ./dist/assets ../../dist/include

.PHONY: bundle-templates
bundle-templates:
	./scripts/bundle-templates

.PHONY: build-deb
build-deb:
	cargo deb --target=x86_64-unknown-linux-musl 
