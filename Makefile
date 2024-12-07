.DEFAULT_GOAL: help
help:
	@echo "make install"
	@echo "		  install dependencies"
	@echo "make test"
	@echo "		  run tests"
	@echo "make setup-default-template-dev"
	@echo "		  setup default template for development (node required)"
	@echo "make build-default-template"
	@echo "		  build default template from source and update dist (node required)"

.PHONY: install
install:
	git submodule update --init

.PHONY: setup-default-template-dev
setup-default-template-dev:
	cargo build;
	cd ./templates/default/src/include && rm -Rf ./node_modules package-lock.json
	./target/debug/liveboat --template-path templates/default/src templates/default/src/include && git restore ./templates/default/src
	cd ./templates/default/src/include && npm install && npm run dev

.PHONY: build-default-template
build-default-template:
	cd ./templates/default/src/include && rm -Rf ./node_modules package-lock.json
	cd ./templates/default/src/include && npm install && npm run build
	cd ./templates/default/src/include && cp -Rf ./dist/assets ../../dist/include


