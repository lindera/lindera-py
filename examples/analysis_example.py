from pathlib import Path
from lindera_py import Analyzer


def main():
    lindera_conf_path = Path("resources") / "lindera_ipadic_conf.json"
    analyzer = Analyzer(config_path=str(lindera_conf_path))

    text = "Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能です。"
    print(text)

    # tokenize the text
    tokens = analyzer.analyze(text)

    # output the tokens
    for token in tokens:
        print(f"token: {token.text}, details: {token.details}")


if __name__ == '__main__':
    main()
