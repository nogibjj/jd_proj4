from wordle import play_wordle
import pandas as pd 


def check_allowed_words(word, word_list):
    if ~word_list["words"].str.contains(word).any():
        print("Word " + word +  " not in allowed list of words");
        exit(1);


def main():
    input = ["ghoul", "ghost", "later"]

    # get list of potential words
    # read in txt file to pandas dataframe
    word_list = pd.read_csv("test/words.txt", header=None, names=["words"])
    # CHANGE TO BIG DATA SOURCE

    total_tries = 0;

    # for each one of the input, run through wordle function
    for word in input:
        # check if word is in allowed list of words
        check_allowed_words(word, word_list);

        guesses = play_wordle(word, word_list);
        total_tries += guesses;

    length = len(input)
    print("Average tries: " + str(round(total_tries / length, 2)));

    return 


if __name__ == "__main__":
    main()