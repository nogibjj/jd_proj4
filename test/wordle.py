from guess import get_guess

def process_guess(true_word, guess, word_list):
    styled_string = ""

    for i in range(len(true_word)):
        if guess[i] == true_word[i]:
            styled = "\033[0;30;42m" + guess[i] 
        elif guess[i] in true_word:
            styled = "\033[0;30;43m" + guess[i] 
        else:
            styled = "\033[0;30;47m" + guess[i] 

        styled_string += styled
    
    print(styled_string)
            
    # filter word list HERE 

    return word_list 


def play_wordle(true_word, word_list):
    guess_count = 0
    solved = False
    filtered_word_list = word_list

    while not solved:
        if guess_count == 0:
            guess = "salet".upper()
        else:
            guess = get_guess(filtered_word_list)
        
        guess_count += 1

        filtered_word_list = process_guess(true_word, guess, filtered_word_list)

        if guess == true_word:
            solved = True
            print("Successfully solved for word {} in {} guesses".format(true_word, guess_count))
    return guess_count