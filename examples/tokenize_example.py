from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def main():
    dictionary_config = DictionaryConfig("ipadic")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Linderaは形態素解析エンジンです。ユーザー辞書も利用可能です。"
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    # output the tokens
    for token in tokens:
        print(f"token: {token.text}, details: {token.details}")


if __name__ == "__main__":
    main()
