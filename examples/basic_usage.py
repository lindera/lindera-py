#!/usr/bin/env python3
"""
Basic usage example for lindera-python
Shows how to use the main features of the Japanese morphological analyzer.
"""


def basic_tokenization_example():
    """Basic tokenization example."""
    print("=== Basic Tokenization Example ===")

    # Import the library
    import lindera

    # Create a tokenizer with default settings
    builder = lindera.TokenizerBuilder()
    builder = builder.set_mode("normal")
    builder = builder.set_dictionary(
        "embedded://ipadic"
    )  # or "embedded://unidic", "embedded://ko-dic", "embedded://cc-cedict"

    tokenizer = builder.build()

    # Test text
    text = "すもももももももものうち"

    # Tokenize
    tokens = tokenizer.tokenize(text)

    print(f"Input text: {text}")
    print(f"Number of tokens: {len(tokens)}")
    print("Tokens:")
    for i, token in enumerate(tokens):
        print(f"  {i + 1}. Text: '{token.text}', Position: {token.position}, Details: {token.details[:3]}")


def character_filter_example():
    """Character filter usage example."""
    print("\n=== Character Filter Example ===")

    import lindera

    # Create character filters
    # 1. Mapping filter - replace characters
    mapping_filter = lindera.CharacterFilter.mapping({"ー": "-", "！": "!"})

    # 2. Unicode normalization filter
    unicode_filter = lindera.CharacterFilter.unicode_normalize("nfkc")

    # 3. Japanese iteration mark filter
    # iteration_filter = lindera.CharacterFilter.japanese_iteration_mark(normalize_kanji=True, normalize_kana=True)

    # Test text with various characters
    test_texts = [
        "テストー",
        "コーヒー！",
        "１２３４５",  # Full-width numbers
    ]

    for text in test_texts:
        print(f"\nOriginal: '{text}'")
        print(f"Mapping:  '{mapping_filter.apply(text)}'")
        print(f"Unicode:  '{unicode_filter.apply(text)}'")


def token_filter_example():
    """Token filter usage example."""
    print("\n=== Token Filter Example ===")

    import lindera

    # Create basic tokenizer
    builder = lindera.TokenizerBuilder()
    tokenizer = builder.set_mode("normal").set_dictionary("embedded://ipadic").build()

    # Get some tokens first
    tokens = tokenizer.tokenize("これはテストです")

    # Create token filters
    lowercase_filter = lindera.TokenFilter.lowercase()
    length_filter = lindera.TokenFilter.length(min=2, max=10)
    # stop_words_filter = lindera.TokenFilter.stop_words(["は", "です"])

    print("Original tokens:")
    for token in tokens:
        print(f"  '{token.text}'")

    # Apply filters
    filtered_tokens = lowercase_filter.apply(tokens[:])  # Copy the list
    print("\nAfter lowercase filter:")
    for token in filtered_tokens:
        print(f"  '{token.text}'")

    filtered_tokens = length_filter.apply(tokens[:])
    print("\nAfter length filter (2-10 chars):")
    for token in filtered_tokens:
        print(f"  '{token.text}'")


def integrated_filter_example():
    """Example of using filters with TokenizerBuilder."""
    print("\n=== Integrated Filter Example ===")

    import lindera

    # Create a tokenizer with integrated filters
    builder = lindera.TokenizerBuilder()
    builder = builder.set_mode("normal").set_dictionary("embedded://ipadic")

    # Add character filter
    char_filter = lindera.CharacterFilter.mapping({"ー": "-"})
    builder = builder.append_character_filter(char_filter)

    # Add token filter
    token_filter = lindera.TokenFilter.lowercase()
    builder = builder.append_token_filter(token_filter)

    # Build tokenizer with filters
    tokenizer = builder.build()

    # Test text
    text = "コーヒーショップ"
    tokens = tokenizer.tokenize(text)

    print(f"Input: '{text}'")
    print("Tokens with applied filters:")
    for token in tokens:
        print(f"  '{token.text}' (position: {token.position})")


def metadata_example():
    """Metadata and schema usage example."""
    print("\n=== Metadata and Schema Example ===")

    import lindera

    # Create default metadata
    metadata = lindera.Metadata.create_default()

    print("Default metadata:")
    print(f"  Name: {metadata.name}")
    print(f"  Encoding: {metadata.encoding}")
    print(f"  Compression algorithm: {metadata.compress_algorithm}")
    print(f"  Default word cost: {metadata.default_word_cost}")

    # Get the dictionary schema
    dict_schema = metadata.dictionary_schema
    print(f"\nDictionary schema has {dict_schema.field_count()} fields:")
    for i, field in enumerate(dict_schema.fields[:5]):  # Show first 5 fields
        print(f"  {i + 1}. {field}")
    print("  ...")

    # Create custom metadata
    custom_schema = lindera.Schema(["surface", "reading", "pos"])
    custom_metadata = lindera.Metadata(name="custom_dict", encoding="UTF-8", dictionary_schema=custom_schema)

    print(f"\nCustom metadata name: {custom_metadata.name}")
    print(f"Custom schema fields: {custom_metadata.dictionary_schema.fields}")


def advanced_example():
    """Advanced usage combining multiple features."""
    print("\n=== Advanced Usage Example ===")

    import lindera

    # Create a sophisticated tokenizer setup
    builder = lindera.TokenizerBuilder()

    # Configure the segmenter
    builder = builder.set_mode("normal")
    builder = builder.set_dictionary("embedded://ipadic")

    # Add multiple character filters
    # 1. Normalize iteration marks
    iter_filter = lindera.CharacterFilter.japanese_iteration_mark(normalize_kanji=True, normalize_kana=True)
    builder = builder.append_character_filter(iter_filter)

    # 2. Character mapping
    mapping_filter = lindera.CharacterFilter.mapping(
        {
            "！": "!",
            "？": "?",
            "。": ".",
        }
    )
    builder = builder.append_character_filter(mapping_filter)

    # Add multiple token filters
    # 1. Length filter (remove very short/long tokens)
    length_filter = lindera.TokenFilter.length(min=2, max=20)
    builder = builder.append_token_filter(length_filter)

    # 2. Japanese base form filter (if available)
    base_form_filter = lindera.TokenFilter.japanese_base_form()
    builder = builder.append_token_filter(base_form_filter)

    # Build the tokenizer
    tokenizer = builder.build()

    # Test with complex text
    complex_text = "私は東京都内のカフェーでコーヒーを飲んでいます！"

    print(f"Complex input: '{complex_text}'")
    tokens = tokenizer.tokenize(complex_text)

    print("Processed tokens:")
    for i, token in enumerate(tokens):
        print(f"  {i + 1:2d}. '{token.text:8s}' | pos: {token.position:2d} | details: {token.details[:2]}")


def main():
    """Run all examples."""
    print("Lindera-python Usage Examples")
    print("=" * 50)

    try:
        basic_tokenization_example()
        character_filter_example()
        token_filter_example()
        integrated_filter_example()
        metadata_example()
        advanced_example()

        print("\n" + "=" * 50)
        print("All examples completed successfully! 🎉")

    except Exception as e:
        print(f"\nExample failed with error: {e}")
        print("Make sure lindera-python is properly installed and compiled.")
        return 1

    return 0


if __name__ == "__main__":
    exit(main())
