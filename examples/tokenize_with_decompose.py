from lindera_py import Segmenter, Tokenizer, load_dictionary


def main():
    # load the dictionary
    dictionary = load_dictionary("ipadic")

    # create a segmenter
    segmenter = Segmenter("decompose", dictionary)

    # create a tokenizer
    tokenizer = Tokenizer(segmenter)

    text = "関西国際空港限定トートバッグを東京スカイツリーの最寄り駅であるとうきょうスカイツリー駅で買う"
    print(f"text: {text}\n")

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    for token in tokens:
        print(token.text)


if __name__ == "__main__":
    main()
