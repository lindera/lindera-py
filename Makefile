.DEFAULT_GOAL := build

VERSION := $(shell poetry version -s)

init:
	poetry self add poetry-plugin-export
	poetry config warnings.export false
	poetry config virtualenvs.in-project true
	poetry install --no-root

update:
	poetry update

clean:
	cargo clean
	find . | grep -E "(__pycache__|.pytest_cache|.mypy_cache|\.pyc|\.pyo$$)" | xargs rm -rf

format:
	cargo fmt
	poetry run isort ./docs ./examples ./tests
	poetry run black ./docs ./examples ./tests

lint:
	cargo clippy --all-features
	poetry run isort --check-only --diff ./docs ./examples ./tests
	poetry run black --check ./docs ./examples ./tests
	poetry run flake8 ./docs ./examples ./tests

typecheck:
	poetry run mypy ./examples ./tests

.PHONY: tests
test:
	cargo test --all-features
	maturin develop --release --all-features
	poetry run pytest -v ./tests

build:
	maturin build -i python --release --all-features

tag:
	git tag v$(VERSION)
	git push origin v$(VERSION)
