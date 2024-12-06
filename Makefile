.DEFAULT_GOAL := build

VERSION := $(shell poetry version -s)
LINDERA_PY_VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="lindera-py") | .version')

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
	cargo clippy --features=cjk
	poetry run isort --check-only --diff ./examples ./tests
	poetry run black --check ./examples ./tests
	poetry run flake8 ./examples ./tests
	poetry run mypy ./examples ./tests

develop:
	poetry run maturin develop --features=cjk

build:
	poetry run maturin build -i python --release --features=cjk

.PHONY: tests
test:
	cargo test --features=cjk
	poetry run maturin develop --features=cjk
	poetry run pytest -v ./tests

publish:
ifeq ($(shell curl -s -XGET https://crates.io/api/v1/crates/lindera-py | jq -r '.versions[].num' | grep $(LINDERA_PY_VERSION)),)
	(cargo package && cargo publish)
endif

tag:
	git tag v$(VERSION)
	git push origin v$(VERSION)
