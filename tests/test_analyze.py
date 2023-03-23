from pathlib import Path
from lindera_py import Analyzer


def test_analyze():
    lindera_conf_path = Path("resources") / "lindera_ipadic_conf.json"
    analyzer = Analyzer(str(lindera_conf_path))

    text = "Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能です。"

    # tokenize the text
    tokens = analyzer.analyze(text)

    assert tokens[0].text == "Lindera"
    assert tokens[1].text == "形態素"
    assert tokens[2].text == "解析"
    assert tokens[3].text == "エンジン"
    assert tokens[4].text == "ユーザ"
    assert tokens[5].text == "辞書"
    assert tokens[6].text == "利用"
    assert tokens[7].text == "可能"

    assert len(tokens) == 8
