#!/bin/bash

ls Decks | while read -r file
do
  python ../converter.py "Decks/$file" "$(echo $file | sed "s/Deck //g" | sed "s/.txt//g")"
done

