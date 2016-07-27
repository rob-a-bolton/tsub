#!/usr/bin/env bash

XMENU_LINES=${XMENU_LINES:-15}
if command -v bemenu; then
    XMENU=bemenu
elif command -v dmenu; then
    XMENU=dmenu
else
    echo "No menu program found (bemenu or dmenu supported)" >&2
    exit 1
fi

data_dir=$(dirname $(readlink -f $0))

choices=$(tsub -c "${data_dir}/data" -l)

choice=$($XMENU -i -l "$XMENU_LINES" -p "Filter: " <<< "$choices")

[[ -z "$choice" ]] && exit 1

old_text=$(xsel -b -o)
if [[ -z "$old_text" ]]; then
    old_text=$(xsel -p -o)
fi
new_text=$($XMENU -p "Text" <<< "$old_text")
if [[ ! -z "$new_text" ]]; then
    transformed_text=$(echo -n "$new_text" | tsub -c "${data_dir}/data" -s "$choice")
    echo -n "$transformed_text" | xsel -p -i
    echo -n "$transformed_text" | xsel -b -i
fi
