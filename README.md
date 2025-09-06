# lindera-python

Python binding for [Lindera](https://github.com/lindera/lindera), a Japanese morphological analysis engine.

## Overview

lindera-python provides a comprehensive Python interface to the Lindera 1.1.1 morphological analysis engine, supporting Japanese, Korean, and Chinese text analysis. This implementation includes all major features:

- **Multi-language Support**: Japanese (IPADIC, UniDic), Korean (ko-dic), Chinese (CC-CEDICT)
- **Character Filters**: Text preprocessing with mapping, regex, Unicode normalization, and Japanese iteration mark handling
- **Token Filters**: Post-processing filters including lowercase, length filtering, stop words, and Japanese-specific filters
- **Flexible Configuration**: Configurable tokenization modes and penalty settings
- **Metadata Support**: Complete dictionary schema and metadata management

## Features

### Core Components

- **TokenizerBuilder**: Fluent API for building customized tokenizers
- **Tokenizer**: High-performance text tokenization with integrated filtering
- **CharacterFilter**: Pre-processing filters for text normalization
- **TokenFilter**: Post-processing filters for token refinement
- **Metadata & Schema**: Dictionary structure and configuration management

### Supported Dictionaries

- **Japanese**: IPADIC (embedded), UniDic (embedded)
- **Korean**: ko-dic (embedded)
- **Chinese**: CC-CEDICT (embedded)
- **Custom**: User dictionary support

### Filter Types

**Character Filters:**

- Mapping filter (character replacement)
- Regex filter (pattern-based replacement)
- Unicode normalization (NFKC, etc.)
- Japanese iteration mark normalization

**Token Filters:**

- Text case transformation (lowercase, uppercase)
- Length filtering (min/max character length)
- Stop words filtering
- Japanese-specific filters (base form, reading form, etc.)
- Korean-specific filters

## Install project dependencies

- pyenv : <https://github.com/pyenv/pyenv?tab=readme-ov-file#installation>
- Poetry : <https://python-poetry.org/docs/#installation>
- Rust : <https://www.rust-lang.org/tools/install>

## Install Python

```shell
# Install Python
% pyenv install 3.13.5
```

## Setup repository and activate virtual environment

```shell
# Clone lindera-python project repository
% git clone git@github.com:lindera/lindera-python.git
% cd lindera-python

# Set Python version for this project
% pyenv local 3.12.3

# Make Python virtual environment
% python -m venv .venv

# Activate Python virtual environment
% source .venv/bin/activate

# Initialize lindera-python project
(.venv) % make init
```

## Install lindera-python as a library in the virtual environment

This command takes a long time because it builds a library that includes all the dictionaries.

```shell
(.venv) % make maturin-develop
```

## Quick Start

### Basic Tokenization

```python
import lindera_py

# Create a tokenizer with default settings
builder = lindera_py.TokenizerBuilder()
builder = builder.set_mode("normal").set_dictionary_kind("ipadic")
tokenizer = builder.build()

# Tokenize Japanese text
text = "すもももももももものうち"
tokens = tokenizer.tokenize(text)

for token in tokens:
    print(f"Text: {token.text}, Position: {token.position}")
```

### Using Character Filters

```python
import lindera_py

# Create character filters
mapping_filter = lindera_py.CharacterFilter.mapping({"ー": "-"})
unicode_filter = lindera_py.CharacterFilter.unicode_normalize("nfkc")

# Apply filters
text = "テストー"
filtered_text = mapping_filter.apply(text)  # "テスト-"
normalized_text = unicode_filter.apply("１２３")  # "123"
```

### Using Token Filters

```python
import lindera_py

# Create token filters
lowercase_filter = lindera_py.TokenFilter.lowercase()
length_filter = lindera_py.TokenFilter.length(min=2, max=10)
stop_words_filter = lindera_py.TokenFilter.stop_words(["は", "です"])

# Apply to tokens (from tokenization)
filtered_tokens = lowercase_filter.apply(tokens)
```

### Integrated Pipeline

```python
import lindera_py

# Build tokenizer with integrated filters
builder = lindera_py.TokenizerBuilder()
builder = builder.set_mode("normal").set_dictionary_kind("ipadic")

# Add character filter
char_filter = lindera_py.CharacterFilter.mapping({"ー": "-"})
builder = builder.append_character_filter(char_filter)

# Add token filter  
token_filter = lindera_py.TokenFilter.lowercase()
builder = builder.append_token_filter(token_filter)

# Build and use
tokenizer = builder.build()
tokens = tokenizer.tokenize("コーヒーショップ")
```

### Working with Metadata

```python
import lindera_py

# Get default metadata
metadata = lindera_py.Metadata.default()
print(f"Dictionary: {metadata.name}")
print(f"Encoding: {metadata.encoding}")

# Access schema information
schema = metadata.dictionary_schema
print(f"Schema has {schema.field_count()} fields")
print(f"Fields: {schema.fields[:5]}")  # First 5 fields
```

## Advanced Usage

See `examples/basic_usage.py` for comprehensive examples including:

- Multi-language tokenization
- Custom filter chains
- Advanced configuration options
- Error handling patterns

## Dictionary Support

### Japanese

- **IPADIC**: Default Japanese dictionary, good for general text
- **UniDic**: Academic dictionary with detailed morphological information

### Korean  

- **ko-dic**: Standard Korean dictionary for morphological analysis

### Chinese

- **CC-CEDICT**: Community-maintained Chinese-English dictionary

### Custom Dictionaries

- User dictionary support for domain-specific terms
- CSV format for easy customization

## API Reference

### Core Classes

- `TokenizerBuilder`: Fluent builder for tokenizer configuration
- `Tokenizer`: Main tokenization engine
- `Token`: Individual token with text, position, and linguistic features
- `CharacterFilter`: Text preprocessing filters
- `TokenFilter`: Token post-processing filters
- `Metadata`: Dictionary metadata and configuration
- `Schema`: Dictionary schema definition

See the `test_basic.py` file for comprehensive API usage examples.
