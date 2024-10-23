# lindera-py

Python binding for [Lindera](https://github.com/lindera-morphology/lindera), a Japanese morphological analysis engine.

## Install project dependencies

- pyenv : <https://github.com/pyenv/pyenv?tab=readme-ov-file#installation>
- Poetry : <https://python-poetry.org/docs/#installation>
- Rust : <https://www.rust-lang.org/tools/install>

## Install Python

```shell
# Install Python
% pyenv install 3.12.3
```

## Setup repository and activate virtual environment

```shell
# Clone lindera-py project repository
% git clone git@github.com:lindera/lindera-py.git
% cd lindera-py

# Set Python version for this project
% pyenv local 3.12.3

# Make Python virtual environment
% python -m venv .venv

# Activate Python virtual environment
% source .venv/bin/activate

# Initialize lindera-py project
(.venv) % make init
```

## nstall lindera-py as a library in the virtual environment

This command takes a long time because it builds a library that includes all the dictionaries.

```shell
(.venv) % make maturin-develop
```

## Example code

```python
from lindera import load_dictionary  # type: ignore
from lindera import Tokenizer


def main():
    dictionary = load_dictionary("ipadic")
    tokenizer = Tokenizer("normal", dictionary)

    text = "関西国際空港限定トートバッグを東京スカイツリーの最寄り駅であるとうきょうスカイツリー駅で買う"
    print(f"text: {text}\n")

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    for token in tokens:
        print(token.text)


if __name__ == "__main__":
    main()
```
