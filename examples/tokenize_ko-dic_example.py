from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def main():
    dictionary_config = DictionaryConfig("ko-dic")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Lindera는형태소해석엔진입니다.사용자사전도사용할수있습니다."
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    # output the tokens
    for token in tokens:
        print(f"token: {token.text}, details: {token.details}")


if __name__ == "__main__":
    main()
