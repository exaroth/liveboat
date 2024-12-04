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
