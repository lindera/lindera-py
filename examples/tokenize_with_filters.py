from lindera import Segmenter, Tokenizer, load_dictionary


def main():
    # load the dictionary
    dictionary = load_dictionary("ipadic")

    # create a segmenter
    segmenter = Segmenter("normal", dictionary)

    # create a tokenizer
    tokenizer = Tokenizer(segmenter)

    # append character filters
    tokenizer.append_character_filter("unicode_normalize", **{"kind": "nfkc"})
    tokenizer.append_character_filter("japanese_iteration_mark", **{"normalize_kanji": True, "normalize_kana": True})
    tokenizer.append_character_filter("mapping", **{"mapping": {"リンデラ": "lindera"}})

    # append token filters
    tokenizer.append_token_filter("japanese_katakana_stem", **{"min": 3})
    tokenizer.append_token_filter(
        "japanese_stop_tags",
        **{
            "tags": [
                "接続詞",
                "助詞",
                "助詞,格助詞",
                "助詞,格助詞,一般",
                "助詞,格助詞,引用",
                "助詞,格助詞,連語",
                "助詞,係助詞",
                "助詞,副助詞",
                "助詞,間投助詞",
                "助詞,並立助詞",
                "助詞,終助詞",
                "助詞,副助詞／並立助詞／終助詞",
                "助詞,連体化",
                "助詞,副詞化",
                "助詞,特殊",
                "助動詞",
                "記号",
                "記号,一般",
                "記号,読点",
                "記号,句点",
                "記号,空白",
                "記号,括弧閉",
                "その他,間投",
                "フィラー",
                "非言語音",
            ]
        },
    )
    tokenizer.append_token_filter("lowercase")

    text = "Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能で、様々なフィルターも内包しています。Linderaはリンデラと読みます。"
    print(f"text: {text}\n")

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    for token in tokens:
        print(token.text)


if __name__ == "__main__":
    main()
