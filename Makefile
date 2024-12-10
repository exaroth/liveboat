prog :=liveboat

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=-debug
else
  release :=--release
  target :=release
  extension :=
endif


all: build install
.DEFAULT_GOAL: help
help:
	@echo "make build"
	@echo "		  build the liveboat binary"
	@echo "make install"
	@echo "		  install dependencies"
	@echo "make test"
	@echo "		  run tests"
	@echo "make setup-default-template-dev"
	@echo "		  setup default template for development (node required)"
	@echo "make build-default-template"
	@echo "		  build default template from source and update dist (node required)"

.PHONY: build
build:
	cargo build $(release)

.PHONY: install
install:
	git submodule update --init
	cp target/$(target)/$(prog) ~/bin/$(prog)$(extension)

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
	./scripts/bundle-templates.sh
