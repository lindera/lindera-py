from lindera import load_dictionary  # type: ignore
from lindera import Tokenizer


def main():
    dictionary = load_dictionary("ipadic")
    tokenizer = Tokenizer("decompose", dictionary)

    text = "関西国際空港限定トートバッグを東京スカイツリーの最寄り駅であるとうきょうスカイツリー駅で買う"
    print(f"text: {text}\n")

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    for token in tokens:
        print(token.text)


if __name__ == "__main__":
    main()
