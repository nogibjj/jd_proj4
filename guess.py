import pandas as pd
pd.options.mode.chained_assignment = None 

def get_guess(word_list, original_word_list):
    # for each letter in range 0 to length of word, get the probability of occurrence through the word list and return as dictionary 
    for x in range(5):
        counts = word_list['words'].str[x].value_counts(normalize=True).to_dict()
        word_list[f'p-{x}'] = word_list['words'].str[x].map(counts)

    # get the total probability across rows in word_list 
    word_list['total'] = word_list[[f'p-{x}' for x in range(5)]].sum(axis=1)

    # find max of total column 
    max_total = word_list['total'].max()

    # return all indices where total is equal to max_total
    max_indices = word_list.index[word_list['total'] == max_total].tolist()

    if len(max_indices) == 1:
        return word_list['words'][max_indices[0]]
    else:
        # find the letters that are different between the words of max_indices and then return a word that contains all those words
        tied_words = word_list.loc[max_indices]

        # find the letters that are different in tied_words 
        letters = []
        for x in range(5):
            if len(tied_words['words'].str[x].unique()) > 1:
                letters.append(tied_words['words'].str[x].unique())

        letters = [item for sublist in letters for item in sublist]

        # find word in original list that contains all letters in letters list 
        word_choices = original_word_list[original_word_list['words'].str.contains('|'.join(letters))]

        # find unique letters in each word of word_choices words
        word_choices['letters'] = word_choices.loc[:,'words'].str.split('').apply(set)

        # find count of letters in word choices letters
        word_choices['count'] = word_choices.loc[:,'letters'].apply(lambda x: len(x.intersection(letters)))

        # find word with max count
        return word_choices['words'][word_choices['count'].idxmax()]
