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
	poetry run isort ./examples ./tests
	poetry run black ./examples ./tests

lint:
	cargo clippy --all-features
	poetry run isort --check-only --diff ./examples ./tests
	poetry run black --check ./examples ./tests
	poetry run flake8 ./examples ./tests

typecheck:
	poetry run mypy ./examples ./tests

.PHONY: tests
test:
	cargo test --all-features
	poetry maturin develop --all-features
	poetry run pytest -v ./tests

maturin-develop:
	poetry run maturin develop --all-features

maturin-build:
	poetry maturin build -i python --release --all-features

tag:
	git tag v$(VERSION)
	git push origin v$(VERSION)
