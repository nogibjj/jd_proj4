# Project 4 - Wordle Solver 

## Overview

A command-line Wordle solver, which takes in a list of 5-letter words and uses a expectation maximization algorithm to guess and solve for the inputted words. Like the official [game](https://www.nytimes.com/games/wordle/index.html), the solver displays its subsequent guesses where each letter is colored accordingly. The letter has a background of green when the letter is exactly correct, yellow if the letter is in the word but in the incorrect position, or grey if the letter does not appear in the actual word. 

At the end of all guesses, it will display a summary that displays the number of guesses needed for each input as well as the overall average guesses the solver takes at the very end. Unlike the actual game where there is a limit of six guesses, the Wordle solver allows any number of guesses as the goal is to see how well the Wordle solver can perform.

An example of the official Wordle game is shown below:

<img width="178" alt="Screen Shot 2023-04-11 at 6 33 41 PM" src="https://user-images.githubusercontent.com/86393045/231324513-d48d04f5-e5ef-434b-a5c4-dd2a49d3aec9.png">

## Data 
The primary data source is the list of all 2,311 words that the offical Wordle uses to populate each day's game. This data was found [here](https://github.com/tabatkins/wordle-list), and is stored as a table in Databricks. The data is read into the wordle solver with a SQL query using the Databricks SQL connector functionality. The code to do so was modeled based on the [example documentation](https://docs.databricks.com/dev-tools/python-sql-connector.html). 

The solver uses `argparse` to create the command-line functionality. The user inputted words are those in which the application solves for. Beforehand, it checks that the user inputted words exist in the allowed list of words before proceeding to guess the word. 


## Algorithm 

The first guess for each word is fixed using `salet`, which is considered one of the [best first guesses](https://news.abplive.com/gaming/online-puzzle-wordle-mit-study-says-this-is-the-best-wordle-starter-word-do-you-agree-1554068#:~:text=The%20Optimal%20Word%2C%20According%20To,helmet%2C%20in%20the%2015th%20century). Based on the results of each guess, the list of potential words is filtered for the remaining possibilities. 

To make a subsequent guess, the probability that each letter occurs in a given position is calculated for each word. For example, if a potential guess is `there`, the probability of 't' occurring at the first letter across all possible words is calculated, then 'h' for the second letter, and so on. This is done in a vectorized fashion for all remaining words. 

If there is an unique word that has the highest overall probability (or has the letters that are most likely to occur), then that word is returned as the next guess. If there is a tie across words, then the unique letters across the tied words are found. Then, the word that contains the most amount of these letters in the original word list (and not restrained to the remaining words) is returned as the next guess. The motivation is when a popular prefix / suffix is guessed, like a word ending with `hing`. Instead of arbitrarily guessing words that satisfy the four letter suffix and potentially running out of guesses, the solver will sacrifice a guess to eliminate as many of the letters that could be the first letter. 

## Architecture 

<img width="400" alt="image" src="https://user-images.githubusercontent.com/86393045/231326863-0c83d6cd-de28-4a4b-bd6d-85712076322d.png">


## Benchmarking 


## Configuration 

Databricks 
- create new cluster 
- enable `DBFS File Browser` under `Admin Settings`
- generate new user token under `User Setttings` and copy 
- import data and create table `words`

Connecting codespace with Databricks 
- Create/update the following secrets in the repository settings 
    - `DATABRICKS_TOKEN`: the user token created from the previous section 
    - `DATABRICKS_HOST_NAME`: server host name under 'JDBC/ODCB' section of cluster 'advanced options' info
    - `DATABRICKS_HTTP_PATH`: HTTP Path under 'JDBC/ODCB' section of cluster 'advanced options' info

To read data from Databricks using Python 
- ensure that `databricks` and `databricks-sql-connector` is installed using `pip install package_name`
