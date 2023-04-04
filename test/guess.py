from random import randint

def get_guess(word_list):
    # get random word from word_list 
    # return word
    index = randint(0, len(word_list) - 1)
    return word_list[index].upper()
