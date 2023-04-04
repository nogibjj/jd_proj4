from wordle import play_wordle


def check_allowed_words(word, word_list):
    if word not in word_list:
        print("Word {} not in allowed list of words", word);
        exit(1);


def main():
    input = ["ghost", "there", "wrote"]


    # get list of potential words
    word_list = ["ghost", "there", "wrote"]
    # CHANGE TO BIG DATA SOURCE

    total_tries = 0;

    # for each one of the input, run through wordle function
    for word in input:
        # check if word is in allowed list of words
        check_allowed_words(word, word_list);

        guesses = play_wordle(word.upper(), word_list);
        total_tries += guesses;

    length = len(input)
    print("Average tries: " + str(total_tries / length));

    return 


if __name__ == "__main__":
    main()