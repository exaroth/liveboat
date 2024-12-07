.DEFAULT_GOAL: help
help:
	@echo "make install"
	@echo "		  install dependencies"
	@echo "make test"
	@echo "		  run tests"
	@echo "make build-release"
	@echo "		  build release version"
	@echo "make lint"
	@echo "		  run flake8, mypy and bandit"
	@echo "make run"
	@echo "		  run adapter (requires .env file)"
	@echo "make update"
	@echo "		  pull latest branch/submodule"

.PHONY: install
install:
	git submodule update --init

.PHONY: setup-default-template-dev
setup-default-template-dev:
	cd ./templates/default/src/include && rm -Rf ./node_modules package-lock.json
	./target/debug/liveboat --template-path templates/default/src templates/default/src/include && git restore ./templates/default/src
	cd ./templates/default/src/include && npm install && npm run dev

.PHONY: build-default-template
build-default-template:
	cd ./templates/default/src/include && rm -Rf ./node_modules package-lock.json
	cd ./templates/default/src/include && npm install && npm run build
	cd ./templates/default/src/include && cp -Rf ./dist/assets ../../dist/include


