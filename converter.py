#!/bin/python3

"""
This is a script to convert from the old Karuta deck format to the new one.
It assumes that the songs and visuals are respectively in the
"Sounds" and "Visuals" subdirectories of the current directory
and that the name of the files are as written in the deck files with .mp3 or
.png extensions

BTW : If you read this, you are a nerd
"""

import sys
import os
import json


def format_title(title: str) -> (str, str):
    """
    In case someone wants to have an op n>1 or ed in their decks, we detect it
    If nothing is specified, default to OP1
    For the geekiests, this is a hand made automaton that finds the substrings
    "OP" or "ED" followed by a number or a space then a number
    Note : Yeah I could have used a regex but I like programming
    """
    if title[:3].isupper() and title[3] == ' ':
        title = title[4:]
    state = 0
    split = 0
    for i, c in enumerate(title.lower()):
        if state == 0 and c == ' ':
            state = 1
        elif state in (1, 6) and c == 'o':
            state = 2
        elif state == 2 and c == 'p':
            split = i - 1
            state = 3
        elif state in (1, 6) and c == 'e':
            state = 4
        elif state == 4 and c == 'd':
            split = i - 1
            state = 5
        elif state in (3, 5, 7) and c.isdigit():
            state = 7
        elif state in (3, 5) and c == ' ':
            state = 6
        elif state in (6, 8) and c.isdigit():
            state = 8
        else:
            state = 0
    if state == 3:                      # case OP
        return (title[:split-1], "OP 1")
    elif state == 5:                    # case ED
        return (title[:split-1], "ED 1")
    if state == 7:                      # case OP/EDn
        return (title[:split-1], (title[split:split+2] + " " + title[split+2:]).upper())
    elif state == 8:                    # case OP/ED n
        return (title[:split-1], title[split:].upper())
    else:
        return (title, "OP 1")


def main(deck_file_path: str, deck_name: str):
    songs = []
    with open(deck_file_path, "r") as deck_file:
        for line in deck_file.readlines():
            songs.append(line.strip())

    print("List of songs : ")
    for song in songs:
        song = format_title(song)
        print(f"{song[0]} - {song[1]}")

    result = {
        "name": deck_name,
        "category": "KARUTA",
        "type": "NORMAL",
        "cover": "default.png",
        "cards": []
    }

    for song in songs:
        anime, music_type = format_title(song)
        result["cards"].append({
            "anime": anime,
            "type": music_type,
            "visual": f"{song}.png",
            "audio": f"{anime} - {music_type}.mp3"
        })
        os.rename(
            f"Sounds/{song}.mp3".replace("'", "_"),
            f"Sounds/{anime} - {music_type}.mp3".replace("'", "_")
        )

    with open(f"{deck_name}.json", "w") as target_file:
        json.dump(result, target_file)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage : {sys.argv[0]} <path to deck file> <deck name>")
    main(sys.argv[1], sys.argv[2])
