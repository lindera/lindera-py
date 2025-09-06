from lindera import Segmenter, Tokenizer, load_dictionary


def test_tokenize_with_ipadic():
    dictionary = load_dictionary("embedded://ipadic")
    segmenter = Segmenter("normal", dictionary)
    tokenizer = Tokenizer(segmenter)

    text = "すもももももももものうち"
    print(text)

    # tokenize the text
    tokens = tokenizer.tokenize(text)

    assert tokens[0].text == "すもも"
    assert tokens[1].text == "も"
    assert tokens[2].text == "もも"
    assert tokens[3].text == "も"
    assert tokens[4].text == "もも"
    assert tokens[5].text == "の"
    assert tokens[6].text == "うち"

    assert len(tokens) == 7
