from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def main():
    dictionary_config = DictionaryConfig("cc-cedict")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Lindera是一个词法分析引擎。用户词典也可用。"
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    # output the tokens
    for token in tokens:
        print(f"token: {token.text}, details: {token.details}")


if __name__ == "__main__":
    main()
