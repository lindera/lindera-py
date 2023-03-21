clean:
	cargo clean
	find . | grep -E "(__pycache__|\.pyc|\.pyo$$)" | xargs rm -rf

lint:
	black --check examples tests

fmt:
	cargo fmt
	black examples/* tests/*

.PHONY: tests
test:
	cargo test
	python -m pytest tests

develop:
	maturin develop --release

build:
	maturin build -i python --release

install:
	pip install . lindera_py

uninstall:
	pip uninstall -y lindera_py
