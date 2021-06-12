import argparse
import sys

import pandas as pd
import numpy as np
import pathlib
import psycopg2
import matplotlib.pyplot as plt
from wordcloud import WordCloud


def remove_characters(text: str, characters):
    for chr in characters:
        text = text.replace(chr, "")

    return text


if __name__ == "__main__":
    argparser = argparse.ArgumentParser()
    argparser.add_argument("--dbconnection",
                           type=str,
                           help="database connection string",
                           default="host=0.0.0.0 user=polidb password=develop")

    args = argparser.parse_args()

    conn = psycopg2.connect(args.dbconnection)

    cursor = conn.cursor()
    cursor.execute(
        "SELECT content, time FROM anforande WHERE affiliation = 'SD' AND time > '2016-01-01' AND time < '2016-12-01';")

    print("Generating Text")
    text = ""
    for (content, tid) in cursor.fetchall():
        text += content

    with open("swedish-stop-words.txt") as stop_words_file:
        stop_words = set(stop_words_file.read().split("\n"))

    with open("allowed-words.txt") as allowed_words_file:
        allowed_words = set(allowed_words_file.read().split("\n"))

    with open("word-standards.txt") as word_standards_file:
        word_mapping = {}
        for line in word_standards_file.readlines():
            f, t = line.strip().split(" ")
            word_mapping[f] = t

    text = remove_characters(text, [",", ".", "!", "?", "-", "'", ":", ";"])
    text = text.lower()

    words = text.split(" ")

    standardized_words = []
    for word in words:
        if word in word_mapping:
            word = word_mapping[word]

        standardized_words.append(word)

    words = standardized_words

    words = list(filter(lambda word: word in allowed_words, words))

    # for word in words:
    #    if "regeringen" in word:
    #        print(word)

    text = " ".join(words)

    # print("words", text)

    print("Generating WordCloud")
    cloud = WordCloud(width=1500, height=1000, random_state=1, background_color='salmon', colormap='Pastel1',
                      collocations=False).generate(text)

    print("Plotting WordCloud")
    # Set figure size
    plt.figure(figsize=(40, 30))
    # Display image
    plt.imshow(cloud)
    # No axis details
    plt.axis("off")

    plt.show()

    cursor.close()
    conn.close()
