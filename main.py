from wordle import play_wordle
import pandas as pd 
from databricks import sql
import os

def querydb(query="SELECT * from words"):
    with sql.connect(
        server_hostname=os.getenv("DATABRICKS_HOST_NAME"),
        http_path=os.getenv("DATABRICKS_HTTP_PATH"),
        access_token=os.getenv("DATABRICKS_TOKEN"),
    ) as connection:

        with connection.cursor() as cursor:
            cursor.execute(query)
            result = cursor.fetchall()

    return result


def check_allowed_words(word, word_list):
    if ~word_list["words"].str.contains(word).any():
        print("Word " + word +  " not in allowed list of words");
        exit(1);


def main():
    input = ["feast", "ghost", "later", "wrath", "salty", "quail", "sleet", "slept", "sleek", "phone",
             "trope", "might", "patch", "arise"]

    # get list of potential words from databricks
    word_list = pd.DataFrame(querydb(), columns=["words"])

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