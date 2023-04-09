from pathlib import Path
from lindera_py import Tokenizer
from lindera_py import DictionaryConfig
from lindera_py import TokenizerConfig


def test_analyze():
    dictionary_config = DictionaryConfig("ko-dic")
    tokenizer_config = TokenizerConfig(dictionary_config, "normal")

    tokenizer = Tokenizer(tokenizer_config)

    text = "Lindera는형태소해석엔진입니다.사용자사전도사용할수있습니다."
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    assert tokens[0].text == "Lindera"
    assert tokens[1].text == "는"
    assert tokens[2].text == "형태소"
    assert tokens[3].text == "해석"
    assert tokens[4].text == "엔진"
    assert tokens[5].text == "입니다"
    assert tokens[6].text == "."
    assert tokens[7].text == "사용자"
    assert tokens[8].text == "사전도"
    assert tokens[9].text == "사용"
    assert tokens[10].text == "할"
    assert tokens[11].text == "수"
    assert tokens[12].text == "있"
    assert tokens[13].text == "습니다"
    assert tokens[14].text == "."

    assert len(tokens) == 15
