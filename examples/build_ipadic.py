import tarfile
import urllib.request

from lindera_py import Segmenter, Tokenizer, build_dictionary, load_dictionary, version


def main():
    # https://Lindera.dev/mecab-ipadic-2.7.0-20070801.tar.gz
    url = "https://lindera.dev/mecab-ipadic-2.7.0-20070801.tar.gz"
    filename = "/tmp/mecab-ipadic-2.7.0-20070801.tar.gz"

    # Add User-Agent header to avoid 403 error
    opener = urllib.request.build_opener()
    opener.addheaders = [("User-Agent", f"lindera-py/{version()}")]
    urllib.request.install_opener(opener)

    # Download dictionary source file
    urllib.request.urlretrieve(url, filename)

    # Extract the dictionary source file
    with tarfile.open(filename, "r:gz") as tar:
        tar.extractall("/tmp/", filter="data")

    source_path = "/tmp/mecab-ipadic-2.7.0-20070801"
    destination_path = "/tmp/lindera-ipadic-2.7.0-20070801"

    # Build dictionary
    build_dictionary("ipadic", source_path, destination_path)

    # Load the built dictionary
    dictionary = load_dictionary(path=destination_path)

    # create a segmenter
    segmenter = Segmenter("normal", dictionary)

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
