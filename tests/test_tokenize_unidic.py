from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def test_analyze():
    dictionary_config = DictionaryConfig("unidic")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Linderaは形態素解析エンジンです。ユーザー辞書も利用可能です。"
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    assert tokens[0].text == "Lindera"
    assert tokens[1].text == "は"
    assert tokens[2].text == "形態"
    assert tokens[3].text == "素"
    assert tokens[4].text == "解析"
    assert tokens[5].text == "エンジン"
    assert tokens[6].text == "です"
    assert tokens[7].text == "。"
    assert tokens[8].text == "ユーザー"
    assert tokens[9].text == "辞書"
    assert tokens[10].text == "も"
    assert tokens[11].text == "利用"
    assert tokens[12].text == "可能"
    assert tokens[13].text == "です"
    assert tokens[14].text == "。"

    assert len(tokens) == 15
