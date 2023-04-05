from random import randint

def get_guess(word_list):
    # for each letter in range 0 to length of word, get the probability of occurrence through the word list and return as dictionary 
    for x in range(5):
        counts = word_list['words'].str[x].value_counts(normalize=True).to_dict()
        word_list[f'p-{x}'] = word_list['words'].str[x].map(counts)

    # get the total probability across rows in word_list 
    word_list['total'] = word_list[[f'p-{x}' for x in range(5)]].sum(axis=1)

    # get the word with the highest probability

    # get index of row with highest total number 
    best_guess = word_list["words"].loc[word_list["total"].idxmax()]

    return best_guess
