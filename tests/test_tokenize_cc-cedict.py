from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def test_analyze():
    dictionary_config = DictionaryConfig("cc-cedict")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Lindera是一个词法分析引擎。用户词典也可用。"
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    assert tokens[0].text == "Lindera"
    assert tokens[1].text == "是"
    assert tokens[2].text == "一"
    assert tokens[3].text == "个"
    assert tokens[4].text == "词法"
    assert tokens[5].text == "分析"
    assert tokens[6].text == "引擎"
    assert tokens[7].text == "。"
    assert tokens[8].text == "用户"
    assert tokens[9].text == "词典"
    assert tokens[10].text == "也"
    assert tokens[11].text == "可"
    assert tokens[12].text == "用"
    assert tokens[13].text == "。"

    assert len(tokens) == 14
